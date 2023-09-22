use std::{sync::{Arc}, time::Duration, net::SocketAddr};

use log::{info, debug, trace, error};
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};
use tokio::{sync::RwLock, task::JoinHandle, time::{Interval, self}};

use crate::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest, RakNetPeerHandle}, util::AnotherlandResult, CONF, atlas::{CPktLogin, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktS2XConnectionState, Uuid, oaPktRealmStatusList}, db::{AccountRecord}, DB, login_server::Session};

pub struct LoginServer {
    internal: Arc<RwLock<LoginServerInternal>>
}

impl LoginServer {
    pub async fn init() -> AnotherlandResult<Self> {
        Ok(LoginServer {
            internal: LoginServerInternal::init().await?
        })
    }
}

pub struct LoginServerInternal {
    listener: RakNetListener,
    task: Option<JoinHandle<()>>,
}

enum LoginError {
    UsernameNotFound,
    WrongPassword,
    Banned,
    InternalError,
}

impl LoginServerInternal {
    async fn handle_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        println!("Message: {:#?}", request.message());
        match request.message() {
            AtlasPkt(CPkt::CPktLogin(pkt)) => {
                debug!("Login request for {}", pkt.username);

                let session = Session::create(&Uuid::new_v4()).await;

                info!("Session {} created for user {}", session.read().await.id().to_string(), pkt.username);

                let realm_ip: SocketAddr = CONF.realm.advertise_address.parse().unwrap();

                // Send login result
                let mut result = CPktLoginResult::default();

                result.login_success = true;
                result.ui_state = CpktLoginResultUiState::CharacterSelection;
                result.user_id = Some(1234);
                result.username = Some(pkt.username.clone());
                result.magic_bytes = Some(pkt.magic_bytes.clone());
                result.field_0x4 = Some(false);
                result.field29_0x24 = Some(0);
                result.realm_ip = Some(u32::from_be(match realm_ip.ip() {
                    std::net::IpAddr::V4(ip) => ip,
                    _ => panic!(),
                }.into()));
                result.realm_port = Some(realm_ip.port());
                result.field38_0x34 = Some(0);
                result.unknown_string = Some(String::new());
                result.session_id = Some(Uuid::from_str(session.read().await.id().to_string().as_str())?);

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;

                // Immediately follow-up with the realm list
                let mut realm_status: oaPktRealmStatusList = oaPktRealmStatusList::default();
        
                realm_status.realm_count = 1;
                realm_status.realm_id = 1;
                realm_status.realm_name = CONF.realm.name.clone();
                realm_status.channel_count = 2;
                realm_status.field_5.push(1);
                realm_status.field_5.push(2);
                realm_status.channel_flag_count = 2;
                realm_status.field_7.push(0);
                realm_status.field_7.push(0);
                
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, realm_status.as_message()).await?;

                /*if let Some(account) = AccountRecord::get_by_username_or_mail(&pkt.username).await? {
                    if account.banned {
                        Self::report_login_error(request.peer(), LoginError::Banned).await?;   
                    } else {
                        // verify password
                        match bcrypt::verify(&pkt.password, &account.password) {
                            Ok(true) => {
                                let session = Session::create(&Uuid::from_str(&account.id.id.to_string()).unwrap()).await;

                                info!("Session {} created for user {}", session.read().await.id().to_string(), account.username);

                                // Send login result
                                let mut result = CPktLoginResult::default();

                                result.login_success = true;
                                result.ui_state = CpktLoginResultUiState::CharacterSelection;
                                result.user_id = Some(account.numeric_id);
                                result.username = Some(account.username);
                                result.magic_bytes = Some(pkt.magic_bytes.clone());
                                result.field_0x4 = Some(false);
                                result.field29_0x24 = Some(0);
                                result.realm_ip = Some(0);
                                result.realm_port = Some(0);
                                result.field38_0x34 = Some(0);
                                result.unknown_string = Some(String::new());
                                result.session_id = Some(Uuid::from_str(session.read().await.id().to_string().as_str())?);
        
                                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;

                                // Immediately follow-up with the realm list
                                let mut realm_status: oaPktRealmStatusList = oaPktRealmStatusList::default();
                        
                                realm_status.realm_count = 1;
                                realm_status.realm_id = 1;
                                realm_status.realm_name = CONF.realm.name.clone();
                                realm_status.channel_count = 2;
                                realm_status.field_5.push(1);
                                realm_status.field_5.push(2);
                                realm_status.channel_flag_count = 2;
                                realm_status.field_7.push(0);
                                realm_status.field_7.push(0);
                                
                                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, realm_status.as_message()).await?;
                            },
                            Ok(false) => {
                                Self::report_login_error(request.peer(), LoginError::WrongPassword).await?;
                            }
                            Err(e) => {
                                error!("Failed to verify password: {:#?}", e);
                                Self::report_login_error(request.peer(), LoginError::InternalError).await?;
                            }
                        }
                    }
                } else {
                    Self::report_login_error(request.peer(), LoginError::UsernameNotFound).await?;
                }*/
            },
            _ => debug!("Unhandled request: {:#?}", request.message()),
        }

        Ok(())
    }

    async fn report_login_error(peer: RakNetPeerHandle, reason: LoginError) -> AnotherlandResult<()> {
        let mut result = CPktLoginResult::default();
        result.login_success = false;
        result.ui_state = CpktLoginResultUiState::RealmSelection;
        
        let message = match reason {
            LoginError::UsernameNotFound => "Account does not exist!",
            LoginError::WrongPassword => "Incorrect password!",
            LoginError::Banned => "Your account has been banned!",
            LoginError::InternalError => "Internal server error!\nPlease try again later.",
        }.as_bytes();

        result.message_len = Some(message.len() as u8);
        result.message = Some(message.to_vec());

        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;
        Ok(())
    }

    pub async fn init() -> AnotherlandResult<Arc<RwLock<Self>>> {
        info!("Starting login server...");

        let mut inst = Arc::new(RwLock::new(Self {
            listener: RakNetListener::new(),
            task: None,
        }));

        inst.write().await.listener.listen(&CONF.login_server.listen_address).await?;

        let task_handle = {
            let inst = inst.clone();

            Some(tokio::spawn(async move {
                let listener = inst.read().await.listener.clone();

                while let Some(request) = listener.next_request().await {
                    if let Err(e) = inst.write().await.handle_request(&request).await {
                        error!("Error handling request from peer {}: {:#?}", request.peer().read().await.guid(), e);
                    }
                }

                trace!("Stopping login server loop...");
            }))
        };

        inst.write().await.task = task_handle;

        Ok(inst)
    }
}