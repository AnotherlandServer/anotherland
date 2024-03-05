// Copyright (C) 2024 AnotherlandServer
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

use std::sync::Arc;

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetPeer}, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktRealmStatusList, RealmStatus};
use log::{error, debug};
use tokio::{select, sync::RwLock};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{actors::{Authenticator, LoginResult, RealmList}, cluster::{frontend::Frontend, RemoteActorRef}, components::SessionHandler, util::{AnotherlandError, AnotherlandResult}, ARGS, CONF, NODE, RAKNET_PRIVATE_KEY};

pub struct LoginFrontend {
    //listener: RakNetListener,
    authenticator: Option<RemoteActorRef<Authenticator>>,
    session_handler: Arc<RwLock<SessionHandler>>,
    realm_list: Option<RemoteActorRef<RealmList>>,
    tasks: TaskTracker,
}

impl LoginFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self { 
            //listener: RakNetListener::bind(CONF.login_server.listen_address).await?,
            authenticator: None,
            session_handler: SessionHandler::new(),
            realm_list: None,
            tasks: TaskTracker::new(),
        })
    }

    fn authenticator(&mut self) -> &mut RemoteActorRef<Authenticator> {
        self.authenticator.as_mut().unwrap()
    }

    fn realm_list(&self) -> &RemoteActorRef<RealmList> {
        self.realm_list.as_ref().unwrap()
    }
}

#[async_trait]
impl Frontend for LoginFrontend {
    fn name(&self) -> &str { "login" }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        self.authenticator = Some(NODE.get_remote_actor("authenticator").unwrap());
        self.realm_list = Some(NODE.get_remote_actor("realm_list").unwrap());

        //self.listener.listen(CONF.login_server.listen_address).await?;

        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let mut listener = RakNetListener::bind(
            CONF.login_server.listen_address, 
            if ARGS.insecure_raknet { 
                None 
            } else { 
                Some(RAKNET_PRIVATE_KEY.clone()) 
            }
        ).await?;

        loop {
            select! {
                peer = listener.accept() => {
                    let mut peer = peer?;

                    let mut client_session = LoginFrontendSession {
                        authenticator: self.authenticator().clone(),
                        session_handler: self.session_handler.clone(),
                        realm_list: self.realm_list().clone(),
                    };

                    self.tasks.spawn(async move {
                        'net_loop: loop {
                            match peer.recv().await {
                                Ok(message) => {
                                    if let Err(e) = client_session.handle_request(&mut peer, message).await {
                                        error!("Failed to handle client request: {:#?}", e);
                                        break 'net_loop;
                                    }
                                },
                                Err(_) => {
                                    break 'net_loop;
                                }
                            }
                        }

                        debug!("Stopping client netloop");

                        // cleanup connection
                        peer.disconnect().await;
                        client_session.session_handler.write().await.forget_peer(*peer.id()).await;
                    });
                },
                _ = token.cancelled() => break Ok(()),
            }
        }
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> { 
        //let _ = self.listener.shutdown().await;

        self.tasks.close();
        self.tasks.wait().await;

        Ok(()) 
    }
}

struct LoginFrontendSession {
    authenticator: RemoteActorRef<Authenticator>,
    session_handler: Arc<RwLock<SessionHandler>>,
    realm_list: RemoteActorRef<RealmList>,
}

impl LoginFrontendSession {

    async fn handle_request(&mut self, peer: &mut RakNetPeer, message: Message) -> AnotherlandResult<()> {
        use Message::*;

        match message {
            AtlasPkt(CPkt::CPktLogin(pkt)) => {
                match self.authenticator.login(pkt.username.to_owned(), pkt.password.to_owned()).await? {
                    LoginResult::Session(session) => {
                        // Assign session to peer
                        let session_ref = self.session_handler.write().await.initiate_trusted(*peer.id(), session.id).await?;
                        let mut session_ref_s = session_ref.lock().await;

                        let realms = self.realm_list.get_realms().await;
                        if realms.is_empty() {
                            // immediately destroy the session
                            let _ = self.session_handler.write().await.destroy_session(session_ref_s.session().id).await;

                            Self::report_login_error(peer, "#Login.ERROR_CONNECTIONFAILED#").await
                        } else if realms.len() == 1 {
                            // select realm
                            session_ref_s.select_realm(realms[0].id).await?;

                            // Send login result with change to realm
                            peer.send(Priority::High, Reliability::Reliable, CPktLoginResult {
                                login_success: true,
                                ui_state: CpktLoginResultUiState::CharacterSelection,
                                user_id: Some(session_ref_s.account().numeric_id),
                                username: Some(session_ref_s.account().username.clone()),
                                magic_bytes: Some(pkt.magic_bytes.clone()),
                                field_0x4: Some(false),
                                field29_0x24: Some(realms[0].id),
                                realm_ip: Some(u32::from_be(realms[0].address.ip().to_owned().into())),
                                realm_port: Some(realms[0].address.port()),
                                field38_0x34: Some(0),
                                unknown_string: Some(String::new()),
                                session_id: Some(session.id),
                                ..Default::default()
                            }.into_message()).await?;

                            Ok(())
                        } else {
                            // Send login result
                            peer.send(Priority::High, Reliability::Reliable, CPktLoginResult {
                                login_success: true,
                                ui_state: CpktLoginResultUiState::RealmSelection,
                                user_id: Some(session_ref_s.account().numeric_id),
                                username: Some(session_ref_s.account().username.clone()),
                                magic_bytes: Some(pkt.magic_bytes.clone()),
                                field_0x4: Some(false),
                                field29_0x24: Some(0),
                                realm_ip: Some(0),
                                realm_port: Some(0),
                                field38_0x34: Some(0),
                                unknown_string: Some(String::new()),
                                session_id: Some(session.id),
                                ..Default::default()
                            }.into_message()).await?;

                            // Immediately follow-up with the realm list
                            peer.send(Priority::High, Reliability::Reliable, oaPktRealmStatusList {
                                realm_count: realms.len() as u32,
                                realms: realms.iter().map(|realm| {
                                    RealmStatus {
                                        id: realm.id,
                                        name: realm.name.clone(),
                                        channel_count: realm.channels.len() as u32,
                                        channel_id: realm.channels.iter().map(|c| c.id).collect(),
                                        channel_population_count: realm.channels.len() as u32,
                                        channel_population: realm.channels.iter().map(|c| c.population).collect(),
                                    }
                                }).collect(),
                                ..Default::default()
                            }.into_message()).await?;

                            Ok(())
                        }
                    },
                    LoginResult::InvalidCredentials => {
                        Self::report_login_error(peer, "#Login.ERROR_PASSWORDMISMATCH#").await
                    },
                    LoginResult::Banned => {
                        Self::report_login_error(peer, "#Login.ERROR_BANNED#").await
                    },
                    LoginResult::ServersLocked => {
                        Self::report_login_error(peer, "#Login.ERROR_SERVERSLOCKED#").await
                    }
                }
            },
            _ => Err(AnotherlandError::app_err("unknown message")),
        }
    }

    async fn report_login_error(peer: &mut RakNetPeer, message: &str) -> AnotherlandResult<()> {
        let message = message.as_bytes();

        peer.send(Priority::High, Reliability::Reliable, CPktLoginResult {
            login_success: false,
            ui_state: CpktLoginResultUiState::RealmSelection,
            message_len: Some(message.len() as u8),
            message: Some(message.to_vec()),
            ..Default::default()
        }.into_message()).await?;
        Ok(())
    }
}
