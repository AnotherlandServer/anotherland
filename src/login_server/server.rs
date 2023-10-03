use std::{time::Duration, net::Ipv4Addr};

use async_trait::async_trait;
use bson::doc;
use log::{info, debug, error, warn};
use mongodb::Database;
use tokio::{time::{Interval, self}, net::{TcpListener, TcpStream}};
use tokio_stream::StreamExt;

use crate::{util::{AnotherlandResult, AnotherlandError}, CONF, db::{Account, Session, Realm, cluster_database}, cluster::{ServerInstance, MessageQueue, ClusterMessage::{InvalidateSession, self}}};
use atlas::raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest, RakNetPeerHandle};
use atlas::{CPktLogin, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktRealmStatusList};
use atlas::RealmStatus;

pub struct LoginServer {
    listener: RakNetListener,
    queue_listener: TcpListener,
    db: Database,

    queue_update: Interval,
    queued_clients: Vec<TcpStream>,

    cluster: MessageQueue,
}

enum LoginError {
    UsernameNotFound,
    WrongPassword,
    Banned,
    InternalError,
}

#[async_trait]
impl ServerInstance for LoginServer {
    async fn init() -> AnotherlandResult<Box<Self>> {
        let db = cluster_database().await;

        // Create Admin account if it doesn't exist yet
        if Account::get_by_username_or_mail(db.clone(), "admin").await?.is_none() {
            Account::create(db.clone(), "admin".to_owned(), "admin@localhost".to_owned(), "1234".to_owned()).await?;



            info!("=========== ADMIN ACCOUNT PASSWORD ===========");
            info!("1234");
            info!("==============================================");
        }

        info!("Starting login server...");

        let mut listener = RakNetListener::new();
        listener.listen(&CONF.login_server.listen_address).await?;

        let queue_listener = TcpListener::bind(&CONF.login_server.queue_listen_address).await?;

        Ok(Box::new(Self {
            listener,
            queue_listener,
            db,
            queue_update: time::interval(Duration::from_secs(1)),
            queued_clients: Vec::new(),
            cluster: MessageQueue::connect().await?,
        }))
    }

    async fn close(&mut self) {
        
    }

    async fn next_request(&mut self) -> AnotherlandResult<Option<RakNetRequest>> {
        Ok(self.listener.next_request().await)
    }

    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        println!("Message: {:#?}", request.message());
        match request.message() {
            AtlasPkt(CPkt::CPktLogin(pkt)) => {
                debug!("Login request for {}", pkt.username);

                if let Some(account) = Account::get_by_username_or_mail(self.db.clone(), &pkt.username).await? {
                    if account.banned {
                        Self::report_login_error(request.peer(), LoginError::Banned).await?;   
                    } else {
                        // verify password
                        match self.verify_and_create_auth_token(&pkt, &account).await {
                            Ok(Some(mut session)) => {
                                info!("Session {} created for user {}", session.id, account.username);

                                let realms = Realm::list(self.db.clone()).await?;

                                if realms.len() == 1 {
                                    // select realm
                                    session.select_realm(self.db.clone(), realms[0].id).await?;

                                    // Send login result with change to realm
                                    let mut result = CPktLoginResult::default();

                                    result.login_success = true;
                                    result.ui_state = CpktLoginResultUiState::CharacterSelection;
                                    result.user_id = Some(account.numeric_id);
                                    result.username = Some(account.username);
                                    result.magic_bytes = Some(pkt.magic_bytes.clone());
                                    result.field_0x4 = Some(false);
                                    result.field29_0x24 = Some(realms[0].id);
                                    result.realm_ip = Some(u32::from_be(realms[0].external_ip.parse::<Ipv4Addr>().unwrap().into()));
                                    result.realm_port = Some(realms[0].external_port);
                                    result.field38_0x34 = Some(0);
                                    result.unknown_string = Some(String::new());
                                    result.session_id = Some(session.id);
            
                                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;

                                    
                                } else {
                                    // Send login result
                                    let mut result = CPktLoginResult::default();

                                    result.login_success = true;
                                    result.ui_state = CpktLoginResultUiState::RealmSelection;
                                    result.user_id = Some(account.numeric_id);
                                    result.username = Some(account.username);
                                    result.magic_bytes = Some(pkt.magic_bytes.clone());
                                    result.field_0x4 = Some(false);
                                    result.field29_0x24 = Some(0);
                                    result.realm_ip = Some(0);
                                    result.realm_port = Some(0);
                                    result.field38_0x34 = Some(0);
                                    result.unknown_string = Some(String::new());
                                    result.session_id = Some(session.id);
            
                                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;

                                    // Immediately follow-up with the realm list
                                    let mut realm_status: oaPktRealmStatusList = oaPktRealmStatusList::default();
                                    realm_status.realm_count = realms.len() as u32;
                                    realm_status.realms = realms.into_iter().map(|realm| {
                                        RealmStatus {
                                            id: realm.id,
                                            name: realm.name,
                                            channel_count: 1,
                                            channel_id: vec![1],
                                            channel_population_count: 1,
                                            channel_population: vec![realm.population],
                                        }
                                    }).collect();
                                    
                                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, realm_status.as_message()).await?;
                                }
                            },
                            Ok(None) => {
                                debug!("Wrong password for: {}", pkt.username);
                                Self::report_login_error(request.peer(), LoginError::WrongPassword).await?;
                            }
                            Err(e) => {
                                error!("Failed to verify password: {:#?}", e);
                                Self::report_login_error(request.peer(), LoginError::InternalError).await?;
                            }
                        }
                    }
                } else {
                    warn!("Unkown username: {}", pkt.username);
                    Self::report_login_error(request.peer(), LoginError::UsernameNotFound).await?;
                }
            },
            _ => debug!("Unhandled request: {:#?}", request.message()),
        }

        Ok(())
    }

    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> {
        match message {
            _ => Ok(())
        }
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        self.queued_clients = self.queued_clients.drain(..).filter(|s| {
            if let Ok(err) = s.take_error() {
                if let Some(_) = err {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }).collect();

        Ok(())
    }
}

impl LoginServer {
    async fn verify_and_create_auth_token(&self, pkt: &CPktLogin, account: &Account) -> AnotherlandResult<Option<Session>> {
        if bcrypt::verify(&pkt.password, &account.password)? {
            // Check if we have a session already running and invalidate those
            let collection = cluster_database().await.collection::<Session>("sessions");
            let mut result = collection.find(doc! { "account": { "$eq": account.id.to_string() } }, None).await?;

            // Notify cluster
            while let Some(session) = result.try_next().await? {
                self.cluster.send(InvalidateSession(session.id)).await?;
            }

            // Delete all sessions
            collection.delete_many(doc! { "account": { "$eq": account.id.to_string() } }, None).await?;

            // Create new session
            Ok(Some(Session::create(self.db.clone(), account).await?))
        } else {
            Ok(None)
        }
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
}