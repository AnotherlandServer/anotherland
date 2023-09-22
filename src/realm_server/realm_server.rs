use std::{sync::{Arc}, time::Duration, net::{SocketAddrV4, SocketAddr}};

use log::{info, debug, trace, error};
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};
use tokio::{sync::RwLock, task::JoinHandle, time::{Interval, self}};

use crate::{raknet::{RakNetListener, Message, Priority, Reliability, RakNetRequest, RakNetPeerHandle}, util::AnotherlandResult, CONF, atlas::{CPktLogin, CPkt, CPktLoginResult, CpktLoginResultUiState, oaPktS2XConnectionState, Uuid, oaPktRealmStatusList, CParamClass_player, CParam, oaCharacter, CPktStream_126_1, oaCharacterList, CPktStream_126_5, oaPktResponseSelectWorld, oaPktCharacterSelectSuccess}, db::{AccountRecord}, DB, login_server::Session};

pub struct RealmServer {
    internal: Arc<RwLock<RealmServerInternal>>
}

impl RealmServer {
    pub async fn init() -> AnotherlandResult<Self> {
        Ok(RealmServer {
            internal: RealmServerInternal::init().await?
        })
    }
}

pub struct RealmServerInternal {
    listener: RakNetListener,
    task: Option<JoinHandle<()>>,
}

impl RealmServerInternal {
    async fn handle_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        println!("Message: {:#?}", request.message());
        match request.message() {
            AtlasPkt(CPkt::oaPktRequestCharacterList(pkt)) => {

                // Create params
                let mut character = CParamClass_player::default();
                character.alive = Some(CParam::Bool(true));
                character.lvl = Some(CParam::Int32(1));
                character.tutorial_mode = Some(CParam::Bool(true));

                let character_data = character.to_bytes();

                println!("{:#?}", character_data);

                let character = oaCharacter {
                    field_0: 0,
                    name: "Test".to_owned(),
                    world_id: 128,
                    length: character_data.len() as u32,
                    params: character_data,
                    field_5: 0,
                };

                let mut character_list = CPktStream_126_1::default();
                character_list.list = oaCharacterList {
                    count: 1,
                    characters: vec![character],
                };

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_list.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktCharacterCreate(pkt)) => {
                println!("Character create: {}", pkt.character_name);

                // Create params
                let mut player = CParamClass_player::default();
                player.alive = Some(CParam::Bool(true));
                player.lvl = Some(CParam::Int32(1));
                player.world_map_guid = Some(CParam::CGuid(Uuid::from_str("58340efa-9495-47e2-b4e4-723a2978a6f1").unwrap()));
                player.zone_guid = Some(CParam::CGuid(Uuid::from_str("b1bbd5c5-0990-454b-bcfa-5dfe176c6756").unwrap()));
                player.tutorial_mode = Some(CParam::Bool(true));

                let player_data = player.to_bytes();

                println!("{:#?}", player_data);

                let mut character_create_successful = CPktStream_126_5::default();
                character_create_successful.character = oaCharacter {
                    field_0: 0,
                    name: pkt.character_name.clone(),
                    world_id: 65,
                    length: player_data.len() as u32,
                    params: player_data,
                    field_5: 0,
                };

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_create_successful.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktRequestSelectWorld(pkt)) => {
                let mut response_select_world = oaPktResponseSelectWorld::default();
                response_select_world.field_1 = true;
                response_select_world.field_2 = 0;
                response_select_world.field_3 = pkt.field_3.clone();
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response_select_world.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktCharacterSelect(pkt)) => {
                let mut character_select_success = oaPktCharacterSelectSuccess::default();

                let world_ip: SocketAddr = CONF.world.advertise_address.parse().unwrap();

                character_select_success.world_ip = u32::from_be(match world_ip.ip() {
                    std::net::IpAddr::V4(ip) => ip,
                    _ => panic!(),
                }.into());
                character_select_success.world_port = world_ip.port();
                character_select_success.magic_bytes = Vec::new();
                character_select_success.magic_bytes.resize(16, 0);

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, character_select_success.as_message()).await?;
            },
            _ => debug!("Unhandled request: {:#?}", request.message()),
        }

        Ok(())
    }

    pub async fn init() -> AnotherlandResult<Arc<RwLock<Self>>> {
        info!("Starting realm server...");

        let mut inst = Arc::new(RwLock::new(Self {
            listener: RakNetListener::new(),
            task: None,
        }));

        inst.write().await.listener.listen(&CONF.realm.listen_address).await?;

        let task_handle = {
            let inst = inst.clone();

            Some(tokio::spawn(async move {
                let listener = inst.read().await.listener.clone();

                while let Some(request) = listener.next_request().await {
                    if let Err(e) = inst.write().await.handle_request(&request).await {
                        error!("Error handling request from peer {}: {:#?}", request.peer().read().await.guid(), e);
                    }
                }

                trace!("Stopping realm server loop...");
            }))
        };

        inst.write().await.task = task_handle;

        Ok(inst)
    }
}