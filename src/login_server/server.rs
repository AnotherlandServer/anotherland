use std::{time::Duration, net::SocketAddrV4};

use async_trait::async_trait;
use bson::doc;
use log::{info, debug, error, warn};
use mongodb::Database;
use tokio::{time::{Interval, self}, net::{TcpListener, TcpStream}};
use tokio_stream::StreamExt;

use crate::{util::AnotherlandResult, CONF, db::{Account, Session, cluster_database}, cluster::{ServerInstance, ClusterMessage::{InvalidateSession, self}, MessageChannel, MessageQueueProducer, connect_queue, ApiRequest, ApiResponse, ApiError}, api_server::schema};
use atlas::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest, RakNetPeerHandle}, Uuid};
use atlas::{CPktLogin, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktRealmStatusList};
use atlas::RealmStatus;

struct RealmEntry {
    id: u32,
    name: String,
    population: usize,
    address: SocketAddrV4,
}

pub struct LoginServer {
    listener: RakNetListener,
    queue_listener: TcpListener,
    db: Database,

    queue_update: Interval,
    queued_clients: Vec<TcpStream>,

    realms: Vec<RealmEntry>,

    cluster: MessageQueueProducer,
    api_frontend: MessageQueueProducer,
}

enum LoginError {
    UsernameNotFound,
    WrongPassword,
    Banned,
    InternalError,
}

#[async_trait]
impl ServerInstance for LoginServer {
    type ServerProperties = ();

    async fn init(_: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        let db = cluster_database().await;

        // Create Admin account if it doesn't exist yet
        /*if Account::get_by_username_or_mail(db.clone(), "admin").await?.is_none() {
            Account::create(db.clone(), "admin".to_owned(), "admin@localhost".to_owned(), "1234".to_owned()).await?;



            info!("=========== ADMIN ACCOUNT PASSWORD ===========");
            info!("1234");
            info!("==============================================");
        }*/

        info!("Starting login server...");

        let mut listener = RakNetListener::new();
        listener.listen(&CONF.login_server.listen_address).await?;

        let queue_listener = TcpListener::bind(&CONF.login_server.queue_listen_address).await?;

        let (producer, _) = connect_queue(MessageChannel::ClusterChannel).await?;
        let (api_frontend, _) = connect_queue(MessageChannel::ApiFrontend).await?;

        Ok(Box::new(Self {
            listener,
            queue_listener,
            db,
            queue_update: time::interval(Duration::from_secs(1)),
            queued_clients: Vec::new(),
            realms: Vec::new(),
            cluster: producer,
            api_frontend,
        }))
    }

    async fn close(&mut self) {
        
    }

    fn raknet_listener(&self) -> Option<&RakNetListener> {
        Some(&self.listener)
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

                                //let realms = Realm::list(self.db.clone()).await?;

                                if self.realms.len() == 1 {
                                    // select realm
                                    session.select_realm(self.db.clone(), self.realms[0].id).await?;

                                    // Send login result with change to realm
                                    let mut result = CPktLoginResult::default();

                                    result.login_success = true;
                                    result.ui_state = CpktLoginResultUiState::CharacterSelection;
                                    result.user_id = Some(account.numeric_id);
                                    result.username = Some(account.username);
                                    result.magic_bytes = Some(pkt.magic_bytes.clone());
                                    result.field_0x4 = Some(false);
                                    result.field29_0x24 = Some(self.realms[0].id);
                                    result.realm_ip = Some(u32::from_be(self.realms[0].address.ip().to_owned().into()));
                                    result.realm_port = Some(self.realms[0].address.port());
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
                                    realm_status.realm_count = self.realms.len() as u32;
                                    realm_status.realms = self.realms.iter().map(|realm| {
                                        RealmStatus {
                                            id: realm.id,
                                            name: realm.name.clone(),
                                            channel_count: 1,
                                            channel_id: vec![1],
                                            channel_population_count: 1,
                                            channel_population: vec![realm.population as f32],
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
            ClusterMessage::RealmServerHearthbeat { realm_id, name, population, address } => {
                // Update realm list
                let realm = self.realms.iter_mut().find(|r| r.id == realm_id);
                match realm {
                    Some(realm) => {
                        realm.population = population;
                        realm.address = address;
                    },
                    None => {
                        self.realms.push(RealmEntry { 
                            id: realm_id, 
                            name, 
                            population, 
                            address
                        });
                    }
                }

                Ok(())
            },
            ClusterMessage::ApiRequest { request_id, request } => {
                match self.handle_api_request(request).await {
                    Ok(Some(response)) => self.api_frontend.send(ClusterMessage::ApiResponse { request_id, response }).await,
                    Ok(None) => Ok(()),
                    Err(e) => self.api_frontend.send(ClusterMessage::ApiResponse { 
                        request_id, 
                        response: ApiResponse::Error(ApiError::Custom { message: e.to_string() }) 
                    }).await,
                }
                
            }
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

    fn get_subscribed_channels(&self) -> Vec<MessageChannel> {
        vec![MessageChannel::ClusterChannel, MessageChannel::ClusterApiChannel]
    }
}

impl LoginServer {
    async fn handle_api_request(&self, request: ApiRequest) -> AnotherlandResult<Option<ApiResponse>> {
        match request {
            ApiRequest::CreateAccout { name, email, password } => {
                let account = Account::create(self.db.clone(), name, email, password).await?;
                Ok(Some(ApiResponse::Account(account.into())))
            },

            ApiRequest::QueryAccount { id } => {
                if let Some(account) = Account::get_by_id(self.db.clone(),  &Uuid::from_str(&id)?).await? {
                    Ok(Some(ApiResponse::Account(account.into())))
                } else {
                    Ok(Some(ApiResponse::Error(ApiError::NotFound)))
                }
            },

            // silently ignore requests we can't answer
            _ => Ok(None),
        }
    }

    async fn verify_and_create_auth_token(&self, pkt: &CPktLogin, account: &Account) -> AnotherlandResult<Option<Session>> {
        if bcrypt::verify(&pkt.password, &account.password)? {
            // Check if we have a session already running and invalidate those
            let collection = cluster_database().await.collection::<Session>("sessions");
            let mut result = collection.find(doc! { "account": { "$eq": account.id.to_string() } }, None).await?;

            // Notify cluster
            while let Some(session) = result.try_next().await? {
                self.cluster.send(InvalidateSession{session_id: session.id}).await?;
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