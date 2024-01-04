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

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetPeer}, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktRealmStatusList, RealmStatus};
use futures::future::Remote;
use log::{error, debug};
use tokio::select;
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{cluster::{frontend::Frontend, RemoteActorRef, ActorRef}, util::{AnotherlandResult, AnotherlandError}, components::{Authenticator, LoginResult, SessionHandler, RealmList}, CONF, NODE, db::Session};

pub struct LoginFrontend {
    //listener: RakNetListener,
    authenticator: Option<RemoteActorRef<Authenticator>>,
    session_handler: Option<ActorRef<SessionHandler<()>>>,
    realm_list: Option<RemoteActorRef<RealmList>>,
    tasks: TaskTracker,
}

impl LoginFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self { 
            //listener: RakNetListener::bind(CONF.login_server.listen_address).await?,
            authenticator: None,
            session_handler: None,
            realm_list: None,
            tasks: TaskTracker::new(),
        })
    }

    fn authenticator(&mut self) -> &mut RemoteActorRef<Authenticator> {
        self.authenticator.as_mut().unwrap()
    }

    fn session_handler(&mut self) -> &mut ActorRef<SessionHandler<()>> {
        self.session_handler.as_mut().unwrap()
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
        self.session_handler = Some(NODE.add_actor(SessionHandler::<()>::initialize("login_session_handler").await));
        self.realm_list = Some(NODE.get_remote_actor("realm_list").unwrap());

        //self.listener.listen(CONF.login_server.listen_address).await?;

        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let mut listener = RakNetListener::bind(CONF.login_server.listen_address).await?;

        loop {
            select! {
                peer = listener.accept() => {
                    let mut peer = peer?;

                    let mut client_session = LoginFrontendSession {
                        authenticator: self.authenticator().clone(),
                        session_handler: self.session_handler().clone(),
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
                        client_session.session_handler.forget_peer(peer.id().clone()).await;
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
    session_handler: ActorRef<SessionHandler<()>>,
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
                        let session_ref = self.session_handler.initiate_trusted(peer.id().clone(), session.id.into()).await?;
                        let mut session_ref_s = session_ref.lock().await;

                        let realms = self.realm_list.get_realms().await;
                        if realms.is_empty() {
                            // immediately destroy the session
                            let _ = self.session_handler.destroy_session(session_ref_s.session().id.into()).await;

                            Self::report_login_error(peer, "#Login.ERROR_CONNECTIONFAILED#").await
                        } else if realms.len() == 1 {
                            // select realm
                            session_ref_s.select_realm(realms[0].id).await?;

                            // Send login result with change to realm
                            let mut result = CPktLoginResult::default();

                            result.login_success = true;
                            result.ui_state = CpktLoginResultUiState::CharacterSelection;
                            result.user_id = Some(session_ref_s.account().numeric_id);
                            result.username = Some(session_ref_s.account().username.clone());
                            result.magic_bytes = Some(pkt.magic_bytes.clone());
                            result.field_0x4 = Some(false);
                            result.field29_0x24 = Some(realms[0].id);
                            result.realm_ip = Some(u32::from_be(realms[0].address.ip().to_owned().into()));
                            result.realm_port = Some(realms[0].address.port());
                            result.field38_0x34 = Some(0);
                            result.unknown_string = Some(String::new());
                            result.session_id = Some(session.id.into());

                            let _ = peer.send(Priority::High, Reliability::Reliable, result.into_message()).await?;

                            Ok(())
                        } else {
                            // Send login result
                            let mut result = CPktLoginResult::default();

                            result.login_success = true;
                            result.ui_state = CpktLoginResultUiState::RealmSelection;
                            result.user_id = Some(session_ref_s.account().numeric_id);
                            result.username = Some(session_ref_s.account().username.clone());
                            result.magic_bytes = Some(pkt.magic_bytes.clone());
                            result.field_0x4 = Some(false);
                            result.field29_0x24 = Some(0);
                            result.realm_ip = Some(0);
                            result.realm_port = Some(0);
                            result.field38_0x34 = Some(0);
                            result.unknown_string = Some(String::new());
                            result.session_id = Some(session.id.into());
    
                            let _ = peer.send(Priority::High, Reliability::Reliable, result.into_message()).await?;

                            // Immediately follow-up with the realm list
                            let mut realm_status: oaPktRealmStatusList = oaPktRealmStatusList::default();
                            realm_status.realm_count = realms.len() as u32;
                            realm_status.realms = realms.iter().map(|realm| {
                                RealmStatus {
                                    id: realm.id,
                                    name: realm.name.clone(),
                                    channel_count: realm.channels.len() as u32,
                                    channel_id: realm.channels.iter().map(|c| c.id).collect(),
                                    channel_population_count: realm.channels.len() as u32,
                                    channel_population: realm.channels.iter().map(|c| c.population).collect(),
                                }
                            }).collect();
                            
                            let _ = peer.send(Priority::High, Reliability::Reliable, realm_status.into_message()).await?;

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
        let mut result = CPktLoginResult::default();
        result.login_success = false;
        result.ui_state = CpktLoginResultUiState::RealmSelection;
        
        let message = message.as_bytes();

        result.message_len = Some(message.len() as u8);
        result.message = Some(message.to_vec());

        let _ = peer.send(Priority::High, Reliability::Reliable, result.into_message()).await?;
        Ok(())
    }
}
