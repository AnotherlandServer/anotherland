use std::{collections::{HashMap, HashSet, VecDeque}, cell::RefCell, sync::Arc, ops::{DerefMut, Deref}, time::Instant, fs};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use glam::{Vec3, Quat};
use legion::{Entity, IntoQuery, World, component, EntityStore};
use mongodb::Client;
use tokio::sync::{RwLock, Mutex};
use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, RakNetRequest, Message}, CPkt, CPktResourceNotify, CpktResourceNotifyResourceType, Uuid, AvatarId, PositionUpdate, ParamClass, CPktBlob, Player, PlayerComponent, PlayerParam, BoundParamClass, NonClientBase, NonClientBaseComponent, TriggerComponent, StartingPointComponent, SpawnNodeComponent, oaPktS2XConnectionState, NpcOtherlandParam, PortalParam, SpawnNodeParam, StartingPointParam, StructureParam, TriggerParam, ParamClassContainer, CPktAvatarUpdate};
use log::{debug, info, kv::{ToValue, Value}, error, trace, warn};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use atlas::ParamEntity;

use crate::{world::{Zone, self, load_zone_from_definition, AvatarComponent, InterestList, AvatarType}, db::ZoneDef};
use crate::{db::{WorldDef, DatabaseRecord, realm_database, Account, Session, cluster_database, Character}, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue}, util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind}};

#[derive(Clone, PartialEq, Eq)]
enum ClientLoadState {
    EarlyLoadSequence,
    RequestAvatarStream,
    StreamedAvatars,
    RequestSpawn,
    Spawned
}

impl Into<u32> for ClientLoadState {
    fn into(self) -> u32 {
        match self {
            ClientLoadState::EarlyLoadSequence => 0,
            ClientLoadState::RequestAvatarStream => 5,
            ClientLoadState::StreamedAvatars => 6,
            ClientLoadState::RequestSpawn => 7,
            ClientLoadState::Spawned => 8,
        }
    }
}

struct ClientState {
    account: Account,
    session: Session,
    peer_id: Uuid,
    avatar_id: AvatarId,
    entity: Entity,
    load_state: ClientLoadState,
    zone: Uuid,
    interest_list: HashSet<Entity>,
    avatar_upload_queue: VecDeque<Entity>,
    avatar_despawn_queue: VecDeque<AvatarId>,
}

impl Serialize for ClientState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut state = serializer.serialize_struct("Client", 3)?;
        state.serialize_field("account", &self.account.username)?;
        state.serialize_field("session", &self.session.id)?;
        state.serialize_field("peer_id", &self.peer_id)?;
        //state.serialize_field("avatar_id", &self.avatar_id)?;
        state.end()
    }
}

impl ToValue for ClientState {
    fn to_value(&self) -> Value<'_> {
        Value::from_serde(self)
    }
}

impl ToValue for &mut ClientState {
    fn to_value(&self) -> Value<'_> {
        Value::from_serde(self)
    }
}

pub struct WorldServerOptions {
    pub world_id: u16,
    pub realm_id: u32,
}

pub struct WorldServerData {
    realm_id: u32,
    worlddef: WorldDef,
    zones: Arc<RwLock<HashMap<Uuid, Zone>>>,
    //world: World,
    frontend: MessageQueueProducer,
    client_state: HashMap<Uuid, Arc<RwLock<ClientState>>>,
    //player_avatar_events: VecDeque<(AvatarId, AvatarEvent)>,
}

#[derive(Clone)]
pub struct WorldServer(Arc<RwLock<WorldServerData>>, u32, u16);

impl Deref for WorldServer {
    type Target = Arc<RwLock<WorldServerData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl WorldServer {
    async fn send(&self, peer_id: &Uuid, message: Message) -> AnotherlandResult<()> {
        self.read().await.frontend.send(ClusterMessage::Response { 
            peer_id: peer_id.to_owned(), 
            data: message.to_bytes()
        }).await
    }

    async fn handle_message(&self, state: &mut ClientState, request: Message) -> AnotherlandResult<()> {
        use Message::*;

        match request {
            AtlasPkt(CPkt::oaPktRequestEnterGame(_)) => {
                info!(client = state; "Player joining world!");

                // Send resource notification 
                let mut worlddef = CPktResourceNotify::default();
                worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                worlddef.field_2 = self.read().await.worlddef.guid.clone();
                worlddef.field_3 = "".to_owned();

                self.send(&state.peer_id, worlddef.as_message()).await?;

                // Update and get avatar data
                let (name, params, pos) = {
                    let mut param_buffer = Vec::new();
                    let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                    let world_state = self.read().await;
                    let zones = world_state.zones.read().await;
                    let zone = zones.get(&state.zone)
                    .ok_or(AnotherlandError::app_err("zone not found"))?;

                    // update player state
                    let character_component = {
                        let mut instance = zone.instance().write().await;
                        let mut entry = instance
                            .entry(state.entity).ok_or(AnotherlandError::app_err("entity not found"))?;

                        let character_component = entry.get_component::<AvatarComponent>().unwrap().to_owned();

                        let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();
                        player_component.set_spawn_mode(2);
                        player_component.set_client_ready(false);
                        player_component.set_player_loading(true);
                        player_component.set_player_node_state(2);
                        player_component.set_world_map_guid(world_state.worlddef.guid.clone());
                        player_component.set_zone(zone.zonedef().zone.clone());
                        player_component.set_zone_guid(state.zone.clone());

                        character_component
                    };

                    let params = PlayerParam::from_component(zone.instance().write().await.deref_mut(), state.entity)?;
                    params.write(&mut writer)?;

                    (
                        character_component.name,
                        param_buffer,
                        PositionUpdate {
                            pos: params.pos().unwrap().to_owned().into(),
                            rot: params.rot().unwrap().to_owned().into(),
                            vel: character_component.vel.into(),
                        }.to_bytes()
                    )
                };

                // Transfer character to client
                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_id = state.avatar_id.as_u64();
                avatar_blob.avatar_name = name;
                avatar_blob.class_id = PlayerParam::CLASS_ID.as_u32();
                avatar_blob.param_bytes = params.len() as u32;
                avatar_blob.params = params;
                avatar_blob.movement_bytes = pos.len() as u32;
                avatar_blob.movement = pos;
                avatar_blob.has_guid = true;
                avatar_blob.field_9 = Some(state.session.id.clone());

                self.send(&state.peer_id, avatar_blob.as_message()).await?;

                Ok(())
            },
            AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                let mut action = pkt.clone();
                action.version = 2;
                self.send(&state.peer_id, action.as_message()).await?;

                Ok(())
            },
            AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                state.load_state = match pkt.field_1 {
                    5 => ClientLoadState::RequestAvatarStream,
                    6 => ClientLoadState::StreamedAvatars,
                    7 => ClientLoadState::RequestSpawn,
                    8 => ClientLoadState::Spawned,
                    _ => {
                        warn!(client = state; "Invalid client loadstate: {}", pkt.field_1);
                        ClientLoadState::EarlyLoadSequence
                    }
                };

                // Confirm loading state
                let mut response = pkt.clone();
                response.field_1 = state.load_state.clone().into();
                response.field_2 = pkt.field_2 + 1;

                self.send(&state.peer_id, response.as_message()).await?;
                
                Ok(())
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                //println!("Routed pkt {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                        // Update world state
                    
                        /*let avatar = self.world
                            .get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap()
                            .get_avatar_mut(state.avatar_id.as_ref().unwrap()).unwrap();
                        avatar.set_position(pkt.pos.pos.into());
                        avatar.set_rotation(pkt.pos.rot.into());
                        avatar.set_velocity(pkt.pos.vel.into());*/
                    },
                    _ => {
                        warn!("Unhandled routed packet: {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                    },
                }

                Ok(())
            },
            _ => {
                debug!(client = state; "Unhandled message: {:#?}", request);
                Ok(())
            }
        }
    }

    fn query_interests(pos: Vec3, radius: f32, zone: &World) -> Vec<Entity> {
        let mut interests = Vec::new();

        // query nonclient avatars
        {
            let mut query = <&NonClientBaseComponent>::query()
                .filter(!component::<TriggerComponent>() & !component::<StartingPointComponent>() & !component::<SpawnNodeComponent>());

            for chunk in query.iter_chunks(zone) {
                for (entity, nonclient_base) in chunk.into_iter_entities() {
                    trace!("Entity dist {:#?} - {} - {}/{} - {}", entity, radius, pos, nonclient_base.pos().unwrap(), nonclient_base.pos().unwrap().distance(pos).abs());

                    if nonclient_base.pos().unwrap().distance(pos) <= radius {
                        interests.push(entity);
                    }
                }
            }
        }

        // query player avatars
        {
            let mut query = <&PlayerComponent>::query();

            for chunk in query.iter_chunks(zone) {
                for (entity, player) in chunk.into_iter_entities() {
                    if player.pos().unwrap().distance(pos) <= radius {
                        interests.push(entity);
                    }
                }
            }
        }

        interests
    }
}

#[async_trait]
impl ServerInstance for WorldServer {
    type ServerProperties = WorldServerOptions;

    async fn init(properties: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        let db = realm_database().await;

        let worlddef = WorldDef::get(db.clone(), &properties.world_id).await?.unwrap();

        let mut zones = HashMap::new();

        for zone in ZoneDef::load_for_world(db.clone(), &worlddef.guid).await?.into_iter() {
            zones.insert(zone.guid.clone(), load_zone_from_definition(db.clone(), zone).await?);
        }

        let (frontend, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: properties.realm_id, 
            channel: RealmChannel::FrontendChannel 
        }).await?;

        Ok(Box::new(Self(Arc::new(RwLock::new(WorldServerData {
            realm_id: properties.realm_id,
            worlddef,
            zones: Arc::new(RwLock::new(zones)),
            frontend,
            client_state: HashMap::new(),
        })), properties.realm_id, properties.world_id)))
    }

    async fn close(&mut self) {

    }

    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> {
        match message {
            ClusterMessage::InvalidateSession { session_id } => { Ok(()) },
            ClusterMessage::Request { session_id, peer_id, data } => {
                let world_state = self.clone();

                // run requests in parallel
                tokio::spawn(async move {
                    // load or create client state
                    let state: AnotherlandResult<Arc<RwLock<ClientState>>> = async {
                        let state = world_state.read().await.client_state.get(&session_id)
                            .map(|r| r.to_owned()); 
                        
                        match state {
                            Some(state) => Ok(state),
                            None => {
                                trace!("Create new client-state");

                                let db = realm_database().await;

                                // Lookup session
                                let session = Session::get(cluster_database().await, &session_id).await?
                                    .ok_or(AnotherlandError::app_err( "unknown session"))?;

                                // Lookup character
                                let mut character = Character::get(db.clone(),
                                    &session.character_id.ok_or(AnotherlandError::app_err("no character selected"))?).await?
                                    .ok_or(AnotherlandError::app_err("character not found"))?;
                            
                                trace!("Spawn character: {}", character.name);

                                let (avatar_id, entity, zone_id, interests) = {
                                    trace!("Character: {:#?}", character.data.as_anyclass().as_hashmap());

                                    let world_state = world_state.read().await;
                                    let mut zones = world_state.zones.write().await;
                                    let zone = zones.get_mut(
                                        character.data.zone_guid().ok_or(AnotherlandError::app_err("character zone not found"))?
                                        ).ok_or(AnotherlandError::app_err("zone not found"))?;

                                    // initial setup
                                    if *character.data.first_time_spawn().unwrap_or(&true) {
                                        character.data.set_pos(zone.start_pos());
                                        character.data.set_rot(zone.start_rot());
                                        character.data.set_first_time_spawn(false);
                                        character.save(db.clone()).await?;
                                    }

                                    let pos = character.data.pos().unwrap_or(&zone.start_pos()).to_owned();
                                    let radius = character.data.aware_range().unwrap_or(&3900.0).to_owned();

                                    // Tho we spawn the avatar here, we'll only tell other clients once
                                    // the player finished loading.
                                    let (avatar_id, entity) = zone.spawn_avatar(world::AvatarType::Player, &character.name, character.data.to_entity()).await;
                                    //zone.instance().write().await.entry(entity).unwrap().add_component(interest_list);
                                    /*zone.instance().write().await.entry(entity).unwrap().add_component(AvatarComponent {
                                        avatar_id
                                        name: character.name.clone(),
                                        vel: Vec3::default(),
                                    });*/

                                    // collect initial interest list
                                    let interests = Self::query_interests(pos, radius, zone.instance().read().await.deref());
                                   
                                    (avatar_id, entity, zone.zonedef().guid.clone(), interests)
                                };

                                // Create client state
                                let state = Arc::new(RwLock::new(ClientState { 
                                    account: Account::get_by_id(cluster_database().await, &session.account).await?
                                        .ok_or(AnotherlandError::app_err("account gone"))?, 
                                    session,
                                    peer_id,
                                    avatar_id,
                                    entity,
                                    zone: zone_id,
                                    load_state: ClientLoadState::EarlyLoadSequence,
                                    interest_list: HashSet::from_iter(interests.clone().into_iter()),
                                    avatar_upload_queue: VecDeque::from_iter(interests.into_iter()),
                                    avatar_despawn_queue: VecDeque::new(),
                                }));

                                trace!("Store new client-state");

                                world_state.write().await.client_state.insert(session_id.clone(), state.clone());

                                Ok(state)
                            }
                        }
                    }.await;

                    trace!("Handling message");

                    match state {
                        Ok(state) => {
                            // Allow only one active message per client
                            let mut state_lock = state.write().await;
                            let message = Message::from_bytes(data.as_slice()).unwrap().1;
                            match world_state.handle_message(state_lock.deref_mut(), message).await {
                                Err(e) => {
                                    let state = state.read().await;
                                    error!(client = state.deref(); "Reqeuest failed: {}", e);
                                },
                                _ => (),
                            }
                        },
                        Err(e) => {
                            error!("Failed to intialize client-state for session: {}\n{:#?}", session_id, e);
                        }
                    }


                });

                Ok(())
            },
            _ => Ok(()),
        }
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        let world_state = self.0.read().await;
        for (session_id, client_state) in &world_state.client_state {
            let mut client_state_s = client_state.write().await;

            // upload avatars
            if client_state_s.load_state != ClientLoadState::EarlyLoadSequence {
                //let slice = client_state_s.avatar_upload_queue.retain(f);

                // limit to push up to 5 avatars per tick
                for _ in 0..5 {
                    if let Some(entity) = client_state_s.avatar_upload_queue.pop_front() {
                        // don't push client owned avatar, as that one is alreary transfered via the
                        // initial avatar blob
                        if entity == client_state_s.entity {
                            continue;
                        }

                        trace!(client = client_state_s.deref(); "Push entity {:#?}", entity);

                        let zones = world_state.zones.read().await;
                        let zone = zones.get(&client_state_s.zone).unwrap();
                        let instance = zone.instance().read().await;

                        let entry = if let Ok(entry) = instance.entry_ref(entity) {
                            entry
                        } else {
                            error!(client = client_state_s.deref(); "Interest {:#?} not found!", entity);
                            continue;
                        };

                        let (avatar_type, params): (_, ParamClassContainer) = {
                            let avatar_type = match entry.get_component::<AvatarType>() {
                                Ok(avatar_type) => avatar_type.to_owned(),
                                _ => {
                                    error!(client = client_state_s.deref(); "Entity {:#?} doesn't have an avatar type set!", entity);
                                    continue;
                                }
                            };

                            let params = match avatar_type {
                                AvatarType::Player => PlayerParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::NpcOtherland => NpcOtherlandParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::Portal => PortalParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::SpawnNode =>  SpawnNodeParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::StartingPoint =>  StartingPointParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::Structure =>  StructureParam::from_component(instance.deref(), entity)?.into(),
                                AvatarType::Trigger =>  TriggerParam::from_component(instance.deref(), entity)?.into(),
                            };

                            (avatar_type, params)
                        };

                        if avatar_type == AvatarType::Player {
                            let avatar_component = entry.get_component::<AvatarComponent>().unwrap();
                            let player_component = entry.get_component::<PlayerComponent>().unwrap();

                            trace!(client = client_state_s.deref(); "Upload player avatar {:#?}", avatar_component);

                            let pos = player_component.pos().unwrap().clone();
                            let rot = player_component.rot().unwrap();
                        
                            let pos = PositionUpdate {
                                pos: pos.into(),
                                rot: Quat::from_euler(glam::EulerRot::XYZ, rot.x, rot.y, rot.z).into(),
                                vel: avatar_component.vel.clone().into(),
                            };
    
                            let mut buf = Vec::new();
                            let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                    
                            let _ = writer.write_bytes(&pos.to_bytes());
    
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u16);
                            let _ = writer.write(0u64);
                            let _ = writer.write(0u64);
    
                            let mut data = Vec::new();
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            params.write(&mut writer)?;
    
                            let mut avatar_update = CPktAvatarUpdate::default();
                            avatar_update.full_update = true;
                            avatar_update.avatar_id = Some(avatar_component.id.as_u64());
                            avatar_update.field_2 = Some(false);
                            avatar_update.name = Some(avatar_component.name.clone());
                            avatar_update.class_id = Some(params.class_id().as_u32());
                            avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            //avatar_update.flags = Some(2);
                            //avatar_update.flag_2_uuid = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.param_bytes = data.len() as u32;
                            avatar_update.params = data;
                            
                            avatar_update.update_source = 0;
                            avatar_update.move_mgr_bytes = Some(buf.len() as u32);
                            avatar_update.move_mgr_data = Some(buf);

                            self.send(&session_id, avatar_update.as_message()).await?;
                        } else {
                            let avatar_component = entry.get_component::<AvatarComponent>().unwrap();
                            let nonclient_base = entry.get_component::<NonClientBaseComponent>().unwrap();

                            trace!(client = client_state_s.deref(); "Upload non-client avatar {:#?}", avatar_component);

                            let pos = nonclient_base.pos().unwrap().clone();
                            let rot = nonclient_base.rot().unwrap();
                        
                            let pos = PositionUpdate {
                                pos: pos.into(),
                                rot: Quat::from_euler(glam::EulerRot::XYZ, rot.x, rot.y, rot.z).into(),
                                vel: avatar_component.vel.clone().into(),
                            };
    
                            let mut buf = Vec::new();
                            let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                    
                            let _ = writer.write_bytes(&pos.to_bytes());
    
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u16);
                            let _ = writer.write(0u64);
                            let _ = writer.write(0u64);
    
                            let mut data = Vec::new();
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            params.write(&mut writer)?;
    
                            let mut avatar_update = CPktAvatarUpdate::default();
                            avatar_update.full_update = true;
                            avatar_update.avatar_id = Some(avatar_component.id.as_u64());
                            avatar_update.field_2 = Some(false);
                            avatar_update.name = Some(avatar_component.name.clone());
                            avatar_update.class_id = Some(params.class_id().as_u32());
                            avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            //avatar_update.flags = Some(2);
                            //avatar_update.flag_2_uuid = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.param_bytes = data.len() as u32;
                            avatar_update.params = data;
                            
                            avatar_update.update_source = 0;
                            avatar_update.move_mgr_bytes = Some(buf.len() as u32);
                            avatar_update.move_mgr_data = Some(buf);

                            self.send(&session_id, avatar_update.as_message()).await?;
                        }
                    } else {
                        if client_state_s.load_state == ClientLoadState::RequestAvatarStream {
                            client_state_s.load_state = ClientLoadState::StreamedAvatars;
                        
                            let mut connectionstate = oaPktS2XConnectionState::default();
                            connectionstate.field_1 = ClientLoadState::StreamedAvatars.into();
                            connectionstate.field_2 = 0;

                            self.send(session_id, connectionstate.as_message()).await?;
                        }

                        break;
                    }
                }
            }
        }

        // propagate events
        /*while let Some(e) = self.write().await.player_avatar_events.pop_front() {
            // update interest lists
            match e.1 {
                AvatarEvent::Spawn { pos } => {
                    self.update_interest_list(e.0, pos);
                },
                AvatarEvent::Move { pos, rot, vel } => {
                    self.update_interest_list(e.0, pos);
                },
                _ => (),
            }

            // loop trough player avatars to update interested clients
            for (_, state) in self.read().await.client_state.iter() {
                let state = state.read().await;

                if state.interest_list.contains(&e.0) {

                }
            }
        }*/

        Ok(())
    }

    fn get_subscribed_channels(&self) -> Vec<MessageChannel> {
        vec![
            MessageChannel::ClusterChannel, 
            MessageChannel::RealmChannel { 
                realm_id: self.1, 
                channel: RealmChannel::GlobalChannel 
            },
            MessageChannel::RealmChannel { 
                realm_id: self.1, 
                channel: RealmChannel::WorldChannel{
                    world_id: self.2
                },
            }
        ]
    }
}
