// Copyright (C) 2023 AnotherlandServer
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

use std::{net::{SocketAddr, Ipv6Addr}, sync::Arc, time::Duration};

use atlas::{AvatarId, raknet::Message, Uuid};
use log::{trace, debug};
use nom::AsBytes;
use quinn::{RecvStream, SendStream, Chunk, Endpoint, ServerConfig, Connecting, VarInt, Connection, ClientConfig};
use rustls::server;
use serde::{Serialize, Deserialize};
use tokio::{task::JoinHandle, sync::mpsc::{self, Receiver, Sender}, select};
use tokio_util::{sync::CancellationToken, task::TaskTracker, bytes::Bytes};

use crate::util::{AnotherlandResult, AnotherlandErrorKind};


#[derive(Serialize, Deserialize, Debug)]
pub enum ZoneMessage {
    EnterZone { session_id: Uuid, avatar_id: AvatarId },
    Travel { session_id: Uuid },
    Message { session_id: Uuid, message: Vec<u8> },
    LeaveZone { session_id: Uuid },
}

pub struct ZoneServerListener {
    endpoint: Endpoint,
    token: CancellationToken,
    tasks: TaskTracker,
    pending_accept: Option<JoinHandle<AnotherlandResult<ZoneServerClient>>>,
}

impl ZoneServerListener {
    pub async fn listen(mut server_config: ServerConfig, addr: SocketAddr) -> AnotherlandResult<Self> {
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.keep_alive_interval(Some(Duration::from_millis(250)));

        Ok(Self {
            endpoint: Endpoint::server(server_config, addr)?,
            token: CancellationToken::new(),
            tasks: TaskTracker::new(),
            pending_accept: None,
        })
    }

    pub async fn accept(&mut self) -> Option<ZoneServerClient> {
        if let Some(accept) = self.pending_accept.as_mut() {
            let res = accept.await.ok()?;
            self.pending_accept.take();
            res.ok()
        } else {
            if let Some(incoming) = self.endpoint.accept().await {
                // spawn accept logic into it's own task, as this method needs to be 
                // cancel safe.
                self.pending_accept = Some(self.tasks.spawn(async move {
                    ZoneServerClient::accept(incoming).await
                }));

                let res = self.pending_accept.as_mut().unwrap().await.ok()?;
                self.pending_accept.take();
                res.ok()
            } else {
                None
            }
        }
    }

    pub async fn close(&mut self) {
        self.token.cancel();
        self.endpoint.close(VarInt::from_u32(0), &[]);

        self.tasks.close();
        self.tasks.wait().await;
    }

    pub fn local_addr(&self) -> AnotherlandResult<SocketAddr> { 
        Ok(self.endpoint.local_addr()?)
    }
}


pub struct ZoneServerClient {
    endpoint: Option<Endpoint>,
    connection: Connection,
    token: CancellationToken,
    tasks: TaskTracker,
    receiver: Receiver<ZoneMessage>,
}

impl ZoneServerClient {
    fn spawn_task(connection: Connection, tasks: TaskTracker, token: CancellationToken, sender: Sender<ZoneMessage>) {
        tasks.spawn({
            let tasks = tasks.clone();
            async move {
                'accept_loop: loop {
                    select! {
                        res = connection.accept_uni() => {
                            match res {
                                Ok(channel) => {
                                    let sender = sender.clone();
                                    tasks.spawn(async move {
                                        if let Ok(message) = Self::read_message(channel).await {
                                            let _ = sender.send(message).await;
                                        }
                                    });
                                },
                                Err(e) => {
                                    debug!("Error while accepting new messages: {:#?}", e);
                                    break 'accept_loop;
                                }
                            }
                        },
                        _ = token.cancelled() => break 'accept_loop,
                    }                   
                }

                trace!("Zone server connection ended");
            }
        });
    }

    async fn read_message(mut stream: RecvStream) -> AnotherlandResult<ZoneMessage> {
        let mut buffer = Vec::new();

        while let Some(chunk) = stream.read_chunk(usize::MAX, false).await? {
            let computed_size = chunk.bytes.len() + chunk.offset as usize;

            if buffer.len() < computed_size {
                buffer.resize(computed_size, 0);
            }

            buffer[chunk.offset as usize..(chunk.offset as usize + chunk.bytes.len())].copy_from_slice(chunk.bytes.as_bytes());
        }

        Ok(bson::from_slice(buffer.as_slice()).map_err(|_| AnotherlandErrorKind::ParseError)?)
    }

    async fn accept(incoming: Connecting) -> AnotherlandResult<ZoneServerClient> {
        let connection = incoming.await?;
        let (sender, receiver) = mpsc::channel(10);

        let tasks = TaskTracker::new();
        let token = CancellationToken::new();
        
        // start connection task
        Self::spawn_task(connection.clone(), tasks.clone(), token.clone(), sender);

        Ok(Self{
            endpoint: None,
            connection,
            token: token.clone(),
            tasks,
            receiver,
        })
    }

    pub async fn connect(addr: SocketAddr) -> AnotherlandResult<Self> {
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(ZoneServerSkipVerification::new())
            .with_no_client_auth();

        let mut endpoint = Endpoint::client(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0))?;
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(config)));

        let connection = endpoint
            .connect(addr, "localhost")?
            .await?;
        let (sender, receiver) = mpsc::channel(10);

        let tasks = TaskTracker::new();
        let token = CancellationToken::new();
        
        // start connection task
        Self::spawn_task(connection.clone(), tasks.clone(), token.clone(), sender);

        Ok(Self{
            endpoint: Some(endpoint),
            connection,
            token: token.clone(),
            tasks,
            receiver,
        })
    }

    pub async fn send(&self, message: &ZoneMessage) -> AnotherlandResult<()> {
        let connection = self.connection.clone();
        let mut buffer = Vec::new();

        bson::to_bson(message).unwrap()
            .as_document().unwrap()
            .to_writer(&mut buffer).unwrap();

        let res = self.tasks.spawn(async move {
            let mut channel = connection.open_uni().await?;
            channel.write_all(&buffer).await?;
            channel.finish().await?;

            Ok(())
        }).await.unwrap();

        res
    }

    pub async fn recv(&mut self) -> Option<ZoneMessage> {
        self.receiver.recv().await
    }

    pub async fn close(&mut self) {
        // only close endpoint if we are owning the connection
        if let Some(endpoint) = self.endpoint.as_ref() {
            self.token.cancel();
            self.connection.close(VarInt::from_u32(0), &[]);
        
            endpoint.close(VarInt::from_u32(0), &vec![]);
        }

        self.tasks.close();
        self.tasks.wait().await;
    }
}

struct ZoneServerSkipVerification;

impl ZoneServerSkipVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for ZoneServerSkipVerification {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}