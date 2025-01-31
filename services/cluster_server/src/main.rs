// Copyright (C) 2025 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(let_chains)]

use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use clap::Parser;
use cluster_context::{ClusterContext, Message};
use core_api::CoreApi;
use error::ClusterFrontendResult;
use once_cell::sync::Lazy;
use protocol::{CPkt, CPktChat, CpktChatChatType, OtherlandPacket};
use raknet::RakNetListener;
use realm_api::{proto::{Destination, NodeAddress, NodeType, RealmClient, RealmRequest}, RealmApi};
use reqwest::Url;
use log::{error, info};
use router::Router;
use tokio::{select, sync::{mpsc, Mutex}};
use toolkit::{print_banner, types::Uuid};

mod error;
mod cluster_context;
mod router;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_CORE_API_URL", default_value = "http://127.0.0.1:8000")]
    service_core_url: Url,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    #[arg(long, env = "REALM_ZMQ_ADDR", default_value = "tcp://127.0.0.1:15001")]
    realm_zmq_addr: String,

    #[arg(long, env = "RAKNET_BIND_ADDR", default_value = "0.0.0.0:6114")]
    raknet_bind_addr: SocketAddr,

    #[arg(long, env = "PUBLIC_ADDR")]
    public_addr: SocketAddr,

    #[arg(long, default_value_t = false)]
    insecure: bool,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[toolkit::service_main]
async fn main() -> ClusterFrontendResult<()> {
    print_banner();

    let realm_api = RealmApi::new(ARGS.service_realm_url.clone());
    let core_api = CoreApi::new(ARGS.service_core_url.clone());

    let (realm_client, notifications) = RealmClient::connect(&ARGS.realm_zmq_addr).await
        .expect("failed to connect to realm zmq server");

    let realm_client = Arc::new(realm_client);

    realm_client.subscribe("core.session.terminated").await?;

    let router = Router::new(realm_api.clone());

    // raknet server
    tokio::spawn(async move {
        let connections = Arc::new(Mutex::new(
            HashMap::<Uuid, mpsc::Sender<Message>>::new()
        ));
        let mut listener = RakNetListener::bind(ARGS.raknet_bind_addr).await?;

        if !ARGS.insecure {
            listener.generate_random_rsa_key();
        }
        
        listener.listen(100).await;

        info!("Server started...");

        // notify realm server we're online
        realm_client.send(RealmRequest::RegisterNode(NodeType::Cluster, NodeAddress::Public(ARGS.public_addr))).await?;

        loop {
            select! {
                socket = listener.accept() => {
                    if let Ok(socket) = socket {
                        let realm_api = realm_api.clone();
                        let core_api = core_api.clone();
                        let router = router.clone();
                        let realm_client = realm_client.clone();
                        let connections = connections.clone();
            
                        tokio::spawn(async move {
                            // Silently drop all connections which do not send oaPktRequestEnterGame as
                            // their first message or whose session is invalid.
                            if 
                                let Ok(pkt) = socket.recv().await &&
                                let Ok((_, CPkt::oaPktRequestEnterGame(pkt))) = CPkt::from_bytes(&pkt) &&
                                let Ok(Some(session)) = core_api.get_session(&pkt.session_id).await
                            {
                                let session_id = *session.id();
            
                                match ClusterContext::create_and_start(
                                    core_api, 
                                    realm_api, 
                                    router, 
                                    socket, 
                                    session,
                                    realm_client
                                ).await {
                                    Ok(sender) => { 
                                        let mut connections = connections.lock().await;
                                        connections.retain(|_, c| !c.is_closed()); // Cleanup connections
                                        connections.insert(session_id, sender);
                                    },
                                    Err(e) => { error!("Failed to start cluster session: {:#?}", e); },
                                }
                            }
                        });
                    } else {
                        break;
                    }
                },
                msg = realm_client.recv() => {
                    if let Ok(realm_api::proto::RealmResponse::ChatMessage { 
                        recipients, 
                        sender_id, 
                        sender_name, 
                        destination, 
                        message 
                    }) = msg {
                        if matches!(destination, Destination::Broadcast) {
                            // Send message to all connected clients
                            let connections = connections.lock().await;
        
                            let pkt = CPktChat {
                                chat_type: CpktChatChatType::Broadcast,
                                message,
                                ..Default::default()
                            }.into_pkt();
        
                            for connection in connections.values() {
                                let _ = connection.send(Message::Sidechannel(pkt.clone())).await;
                            }
                        } else {
                            let connections = connections.lock().await;

                            let (chat_type, receiver) = match destination {
                                Destination::Broadcast => (CpktChatChatType::Broadcast, String::default()),
                                Destination::Whisper(name) => (CpktChatChatType::Whisper, name),
                                Destination::Clan(_) => (CpktChatChatType::Clan, String::default()),
                                Destination::ClanOfficer(_) => (CpktChatChatType::ClanOfficer, String::default()),
                                Destination::Party(_) => (CpktChatChatType::Party, String::default()),
                            };
        
                            let pkt = CPktChat {
                                field_2: sender_id.unwrap_or_default(),
                                chat_type,
                                message,
                                sender: sender_name,
                                receiver,
                                ..Default::default()
                            }.into_pkt();
        
                            for recipient in recipients {
                                if let Some(connection) = connections.get(&recipient) {
                                    let _ = connection.send(Message::Sidechannel(pkt.clone())).await;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(())
    }).await?
}
