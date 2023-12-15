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

use std::{collections::{HashMap, HashSet, VecDeque}, sync::Arc, ops::{DerefMut, Deref}, time::{SystemTime, UNIX_EPOCH, Duration}};

use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use glam::{Vec3, Quat};
use legion::{Entity, IntoQuery, World, component, EntityStore, Schedule};
use tokio::sync::RwLock;
use async_trait::async_trait;
use atlas::{raknet::Message, CPkt, Uuid, AvatarId, PositionUpdate, Player, PlayerComponent, PlayerParam, NonClientBase, NonClientBaseComponent, TriggerComponent, StartingPointComponent, SpawnNodeComponent, oaPktS2XConnectionState, NpcOtherlandParam, PortalParam, SpawnNodeParam, StartingPointParam, StructureParam, TriggerParam, ParamClassContainer, CPktAvatarUpdate, CPktServerNotify, oaPktServerAction, CpktChatChatType, CPktChat};
use log::{debug, kv::{ToValue, Value}, error, trace, warn};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use atlas::ParamEntity;
use atlas::CPktAvatarClientNotify;
use atlas::BoundParamClass;

use super::world::{Zone, self, load_zone_from_definition, AvatarComponent, AvatarType, PlayerSpawnMode};
use crate::{db::ZoneDef, cluster::{TravelType::{DirectTravel, PortalTravel, NonPortalTravel}, SocialEvent, ChatType}, node_server::world::NetworkComponent};
use crate::{db::{WorldDef, DatabaseRecord, realm_database, Account, Session, cluster_database, Character}, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue}, util::{AnotherlandResult, AnotherlandError}};

#[derive(Clone, PartialEq, Eq)]
pub(in crate::node_server) enum ClientLoadState {
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

pub(in crate::node_server) struct ClientState {
    pub account: Account,
    pub session: Session,
    pub peer_id: Uuid,
    pub avatar_id: AvatarId,
    pub entity: Entity,
    pub load_state: ClientLoadState,
    pub zone: Uuid,
    pub interest_list: HashSet<Entity>,
    pub avatar_upload_queue: VecDeque<Entity>,
    pub avatar_despawn_queue: VecDeque<AvatarId>,
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

pub struct NodeServerOptions {
    pub zone_guid: Uuid,
    pub realm_id: u32,
}

pub struct NodeServerData {
    pub(in crate::node_server) realm_id: u32,
    pub(in crate::node_server) worlddef: WorldDef,
    pub(in crate::node_server) zone: Arc<RwLock<Zone>>,
    //zones: Arc<RwLock<HashMap<Uuid, Zone>>>,
    //world: World,
    pub(in crate::node_server) realm: MessageQueueProducer,
    pub(in crate::node_server) frontend: MessageQueueProducer,
    pub(in crate::node_server) social: MessageQueueProducer,
    pub(in crate::node_server) client_state: HashMap<Uuid, Arc<RwLock<ClientState>>>,

    //player_avatar_events: VecDeque<(AvatarId, AvatarEvent)>,
}

#[derive(Clone)]
pub struct NodeServer(Arc<RwLock<NodeServerData>>, u32, Uuid);

impl Deref for NodeServer {
    type Target = Arc<RwLock<NodeServerData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl NodeServer {
    pub(in crate::node_server) async fn send(&self, peer_id: &Uuid, message: Message) -> AnotherlandResult<()> {
        let serialized = message.to_bytes();

        debug!("Sending message of size {}", serialized.len());

        self.read().await.frontend.send(ClusterMessage::Response { 
            peer_id: peer_id.to_owned(), 
            data: serialized
        }).await
    }

    async fn handle_message(&self, state: &mut ClientState, request: Message) -> AnotherlandResult<()> {
        use Message::*;

        match request {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => {
                self.request_enter_game(state, *pkt).await
            },
            AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                self.request_server_action(state, *pkt).await
            },
            AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                self.request_c2s_connection_state(state, *pkt).await
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                //println!("Routed pkt {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                       self.request_move_manager_pos_update(state, *pkt).await
                    },
                    _ => {
                        warn!("Unhandled routed packet: {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                        Ok(())
                    },
                }
            },
            AtlasPkt(CPkt::oaPktAvatarTellBehaviorBinary(pkt)) => {
                self.request_avatar_tell_behavior_binary(state, *pkt).await
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunity(pkt)) => {
                self.request_cluster_client_to_community(state, *pkt).await
            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                self.request_avatar_update(state, *pkt).await
            },
            AtlasPkt(CPkt::CPktChat(pkt)) => {
                let (name, pos) = {
                    // Get world state
                    let instance =  self.read().await.zone.read().await.instance().clone();
                    let instance_s = instance.read().await;

                    if let Ok(entry) = instance_s.entry_ref(state.entity) {
                        let player_component = entry.get_component::<PlayerComponent>().unwrap();
                        let avatar_component = entry.get_component::<AvatarComponent>().unwrap();

                        (avatar_component.name.clone(), player_component.pos().unwrap_or(&Vec3::default()).to_owned())
                    } else {
                        panic!("Player entity not found!");
                    }
                };

                match pkt.chat_type {
                    CpktChatChatType::Say => {
                        self.send_chat_message(name, pos, ChatType::Say, pkt.message).await
                    },
                    CpktChatChatType::Shout => {
                        self.send_chat_message(name, pos, ChatType::Shout, pkt.message).await
                    },
                    // All other chat types are handled by the frontend
                    _ => Ok(()),
                }
            },
            _ => {
                debug!(client = state; "Unhandled message: {:#?}", request);
                Ok(())
            }
        }
    }

    pub(in crate::node_server) fn query_interests(pos: Vec3, radius: f32, zone: &World) -> Vec<Entity> {
        let mut interests = Vec::new();

        // query nonclient avatars
        {
            let mut query = <&NonClientBaseComponent>::query()
                .filter(!component::<TriggerComponent>() & !component::<StartingPointComponent>() & !component::<SpawnNodeComponent>());

            for chunk in query.iter_chunks(zone) {
                for (entity, nonclient_base) in chunk.into_iter_entities() {
                    //trace!("Entity dist {:#?} - {} - {}/{} - {}", entity, radius, pos, nonclient_base.pos().unwrap(), nonclient_base.pos().unwrap().distance(pos).abs());

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

    async fn send_chat_message(&self, sender: String, pos: Vec3, chat_type: ChatType, message: String) -> AnotherlandResult<()> {
        let state_s = self.read().await;
        let mut send_list = Vec::new();

        let world = {
            let zone_s = state_s.zone.read().await;
            zone_s.instance().clone()
        };

        let world_s = world.read().await;

        let range = match chat_type {
            ChatType::Say => 3900.0,
            ChatType::Shout => 8000.0,
            _ => unreachable!(),
        };

        let mut query = <(&PlayerComponent, &NetworkComponent)>::query();

        // collect avatars in range of the sayer
        for chunk in query.iter_chunks(world_s.deref()) {
            for (_, (player, net)) in chunk.into_iter_entities() {
                if player.pos().unwrap().distance(pos) <= range {
                    send_list.push(net.peer_id.clone());
                    /**/
                }
            }
        }

        // create message
        let chat_message = {
            let mut m = CPktChat::default();
            m.chat_type = match chat_type {
                ChatType::Say => CpktChatChatType::Say,
                ChatType::Shout => CpktChatChatType::Shout,
                _ => unreachable!(),
            };
            m.sender = sender;
            m.message = message;
            m.as_message().to_bytes()
        };

        // send chat message to all determined connections
        for peer_id in send_list {
            state_s.frontend.send(ClusterMessage::Response { 
                peer_id, 
                data: chat_message.clone() 
            }).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl ServerInstance for NodeServer {
    type ServerProperties = NodeServerOptions;

    fn tickrate() -> Duration { Duration::from_millis(20) }

    async fn init(properties: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        let db = realm_database().await;

        let zonedef = ZoneDef::get(db.clone(), &properties.zone_guid).await?.expect("zone not found");
        let worlddef = WorldDef::get_by_guid(db.clone(), &zonedef.worlddef_guid).await?.expect("world not found"); 
        //WorldDef::get(db.clone(), &properties.zone_guid).await?.unwrap();

        //let mut zones = HashMap::new();

        /*for zone in ZoneDef::load_for_world(db.clone(), &worlddef.guid).await?.into_iter() {
            zones.insert(zone.guid.clone(), load_zone_from_definition(db.clone(), zone).await?);
        }*/

        let zone_guid = zonedef.guid.clone();
        let zone = load_zone_from_definition(db.clone(), zonedef).await?;

        let (frontend, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: properties.realm_id, 
            channel: RealmChannel::FrontendChannel 
        }).await?;

        let (realm, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: properties.realm_id, 
            channel: RealmChannel::GlobalChannel 
        }).await?;

        let (social, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: properties.realm_id, 
            channel: RealmChannel::SocialChannel 
        }).await?;

        Ok(Box::new(Self(Arc::new(RwLock::new(NodeServerData {
            realm_id: properties.realm_id,
            worlddef: worlddef,
            zone: Arc::new(RwLock::new(zone)),
            realm,
            frontend,
            social,
            client_state: HashMap::new(),
        })), properties.realm_id, zone_guid)))
    }

    async fn close(&mut self) {

    }

    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> {
        match message {
            ClusterMessage::InvalidateSession { session_id } => { 
                let state = self.write().await.client_state.remove(&session_id);

                if let Some(state) = state {
                    let state = state.read().await;
                    self.read().await.zone.write().await.remove_avatar(&state.avatar_id).await;
                    
                    // leave social server
                    self.read().await.social.send(ClusterMessage::SocialEvent { 
                        event: SocialEvent::PeerLeave { peer_id: state.peer_id.clone() }
                    }).await?;

                    // update all other states and tell them to remove the avatar assigned to this session
                    let self = self.read().await;
                    for other_state in self.client_state.values() {
                        let mut other_state = other_state.write().await;
                        other_state.avatar_despawn_queue.push_back(state.avatar_id.clone());
                        other_state.avatar_upload_queue.retain(|id| *id != state.entity);
                        other_state.interest_list.retain(|id| *id != state.entity);
                    }
                }

                Ok(()) 
            },
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
                                    let world_state = world_state.read().await;
                                    let mut zone = world_state.zone.write().await;

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
                                    let (avatar_id, entity) = zone.spawn_avatar(world::AvatarType::Player, None, &character.name, character.data.to_entity()).await;
                                    zone.instance().write().await.entry(entity).unwrap().add_component(NetworkComponent {
                                        peer_id: peer_id.clone(),
                                    });

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
            ClusterMessage::ZoneTravelRequest { session_id, peer_id, avatar_id, current_zone, destination_zone, travel_type } => {
                // only react to travel requests if we are the destination
                if destination_zone == self.read().await.zone.read().await.zonedef().guid {
                    trace!("Create new client-state via travel");

                    let db = realm_database().await;
    
                    // Lookup session
                    let session = Session::get(cluster_database().await, &session_id).await?
                        .ok_or(AnotherlandError::app_err( "unknown session"))?;
    
                    // Lookup character
                    let mut character: Character = Character::get(db.clone(),
                        &session.character_id.ok_or(AnotherlandError::app_err("no character selected"))?).await?
                        .ok_or(AnotherlandError::app_err("character not found"))?;
                
                    trace!("Spawn character: {}", character.name);
    
                    let (entity, zone_id, interests, pos, rot) = {
                        let world_state = self.read().await;
                        let mut zone = world_state.zone.write().await;
    
                        match travel_type {
                            DirectTravel => {
                                character.data.set_pos(zone.start_pos());
                                character.data.set_rot(zone.start_rot()); 
                            },
                            PortalTravel => todo!(),
                            NonPortalTravel => todo!(),
                        }
    
                        let pos = character.data.pos().unwrap_or(&zone.start_pos()).to_owned();
                        let rot = character.data.pos().unwrap_or(&zone.start_rot()).to_owned();
                        let radius = character.data.aware_range().unwrap_or(&3900.0).to_owned();

                        character.data.set_player_node_state(0);
    
                        // Tho we spawn the avatar here, we'll only tell other clients once
                        // the player finished loading.
                        let (_, entity) = zone.spawn_avatar(world::AvatarType::Player, Some(avatar_id.clone()), &character.name, character.data.to_entity()).await;
                        zone.instance().write().await.entry(entity).unwrap().add_component(NetworkComponent {
                            peer_id: peer_id.clone(),
                        });

                        // collect initial interest list
                        let interests = Self::query_interests(pos, radius, zone.instance().read().await.deref());
                       
                        (entity, zone.zonedef().guid.clone(), interests, pos, rot)
                    };
    
                    // Create client state
                    let state = Arc::new(RwLock::new(ClientState { 
                        account: Account::get_by_id(cluster_database().await, &session.account).await?
                            .ok_or(AnotherlandError::app_err("account gone"))?, 
                        session,
                        peer_id,
                        avatar_id: avatar_id.clone(),
                        entity,
                        zone: zone_id,
                        load_state: ClientLoadState::EarlyLoadSequence,
                        interest_list: HashSet::from_iter(interests.clone().into_iter()),
                        avatar_upload_queue: VecDeque::from_iter(interests.into_iter()),
                        avatar_despawn_queue: VecDeque::new(),
                    }));
    
                    trace!("Store new client-state");
    
                    self.write().await.client_state.insert(session_id.clone(), state.clone());

                    // notify the original zone server
                    let (src_server, _) = connect_queue(MessageChannel::RealmChannel { 
                        realm_id: self.read().await.realm_id, 
                        channel: RealmChannel::NodeChannel { zone_guid: current_zone } 
                    }).await?;

                    src_server.send(ClusterMessage::ZoneTravelResponse { 
                        session_id, 
                        avatar_id,
                        destination_zone, 
                        pos, 
                        rot
                    }).await?;
                }

                Ok(())
            },
            ClusterMessage::ZoneTravelResponse { session_id, avatar_id, destination_zone, pos, rot } => {
                // load and remove session from client state map.
                let state = self.write().await.client_state.remove(&session_id);

                if let Some(state) = state {
                    trace!("Zone travel response");

                    let state = state.read().await;
                    let db: mongodb::Database = realm_database().await;

                    let target_zone = ZoneDef::get(db.clone(), &destination_zone).await?.unwrap();
                    let target_world = WorldDef::get_by_guid(db.clone(), &target_zone.worlddef_guid).await?.unwrap();

                    let params = {
                        let instance =  self.read().await.zone.read().await.instance().clone();
                        let mut instance_s = instance.write().await;

                        if let Ok(mut entry) = instance_s.entry_mut(state.entity) {
                            let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();

                            // move avatar to new zone and position
                            player_component.set_spawn_mode(PlayerSpawnMode::TravelDirect.into());
                            player_component.set_world_map_guid(&target_zone.worlddef_guid.to_string());
                            player_component.set_zone_guid(target_zone.guid);
                            player_component.set_pos(pos);
                            player_component.set_rot(rot);
                        }

                        PlayerParam::from_component(&instance_s, state.entity).unwrap()
                    };

                    // remove the avatar and store changes in the database
                    self.read().await.zone.write().await.remove_avatar(&state.avatar_id).await;

                    trace!("5");

                    //instance_s.remove(state.entity);

                    let mut character = Character::get(db.clone(), &state.session.character_id.unwrap()).await.unwrap().unwrap();
                    character.data = params.clone();
                    character.world_id = target_world.id as u32;
                    character.save(db.clone()).await?;

                    trace!("6");

                    // todo: Notify other clients, that this avatar despawned

                    // notify the frontend server, that we've let the client go
                    self.read().await.frontend.send(ClusterMessage::ZoneTravelFinished { 
                        session_id, 
                        avatar_id,
                        world_id: target_world.id, 
                        zone_id: destination_zone 
                    }).await?;
                }

                Ok(())
            },
            _ => Ok(()),
        }
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        let world_state = self.0.read().await;
        for (_session_id, client_state) in &world_state.client_state {
            let mut client_state_s = client_state.write().await;

            // remove avatars
            while let Some(avatar_id) = client_state_s.avatar_despawn_queue.pop_front() {
                let mut avatar_notify = CPktAvatarClientNotify::default();
                avatar_notify.avatar_id = avatar_id.as_u64();
                self.send(&client_state_s.peer_id, avatar_notify.as_message()).await?;
            }

            // upload avatars
            if client_state_s.load_state != ClientLoadState::EarlyLoadSequence {
                // limit to push up to 5 avatars per tick
                for _ in 0..5 {
                    if let Some(entity) = client_state_s.avatar_upload_queue.pop_front() {
                        // don't push client owned avatar, as that one is alreary transfered via the
                        // initial avatar blob
                        if entity == client_state_s.entity {
                            continue;
                        }

                        trace!(client = client_state_s.deref(); "Push entity {:#?}", entity);

                        //let zones = world_state.zones.read().await;
                        //let zone = zones.get(&client_state_s.zone).unwrap();
                        let zone = world_state.zone.read().await;
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
                                rot: rot.into(),
                                vel: avatar_component.vel.clone().into(),
                            };
    
                            let mut buf = Vec::new();
                            {
                                let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                        
                                let _ = writer.write_bytes(&pos.to_bytes());
        
                                let _ = writer.write(0u64);
                                let _ = writer.write(0u8);
                                let _ = writer.write(0u8);
                                let _ = writer.write(0u8);
                                let _ = writer.write(0u16);
                                let _ = writer.write(0u16);
                                let _ = writer.write(0u64);
                            }
    
                            let mut data = Vec::new();
                            {
                                let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                                params.write_to_client(&mut writer)?;
                            }
    
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

                            self.send(&client_state_s.peer_id, avatar_update.as_message()).await?;
                        } else {
                            let avatar_component = entry.get_component::<AvatarComponent>().unwrap();
                            let nonclient_base = entry.get_component::<NonClientBaseComponent>().unwrap();

                            trace!(client = client_state_s.deref(); "Upload non-client avatar {:#?}", avatar_component);

                            let pos = nonclient_base.pos().unwrap().clone();
                            let rot = nonclient_base.rot().unwrap();

                            // yaw (y-axis), pitch (x-axis), roll (z-axis)
                            let quat: Quat = Quat::from_euler(glam::EulerRot::YXZ, rot.y.atan2(rot.x), rot.z.atan2((rot.x * rot.x + rot.y * rot.y).sqrt()), 0.0);
                        
                            let pos = PositionUpdate {
                                pos: pos.into(),
                                rot: quat.into(),
                                vel: avatar_component.vel.clone().into(),
                            };
    
                            let mut buf = Vec::new();
                            let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                    
                            let _ = writer.write_bytes(&pos.to_bytes());
    
                            let _ = writer.write(0u64);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u16);
                            let _ = writer.write(0u16);
                            let _ = writer.write(0u64);
    
                            let mut data = Vec::new();
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            params.write_to_client(&mut writer)?;
    
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

                            //debug!("Sending params of size {} - pkt field {}", data.len(), avatar_update.param_bytes);
                            
                            avatar_update.update_source = 0;
                            avatar_update.move_mgr_bytes = Some(buf.len() as u32);
                            avatar_update.move_mgr_data = Some(buf);

                            self.send(&client_state_s.peer_id, avatar_update.as_message()).await?;
                        }
                    } else {
                        if client_state_s.load_state == ClientLoadState::RequestAvatarStream {
                            client_state_s.load_state = ClientLoadState::StreamedAvatars;
                        
                            let mut connectionstate = oaPktS2XConnectionState::default();
                            connectionstate.field_1 = ClientLoadState::StreamedAvatars.into();
                            connectionstate.field_2 = 0;

                            self.send(&client_state_s.peer_id, connectionstate.as_message()).await?;
                        }

                        break;
                    }
                }
            }

            if client_state_s.load_state == ClientLoadState::RequestSpawn {
                 // Synchronize time
                 {
                    let mut game_time_sync = CPktServerNotify::default();
                    game_time_sync.notify_type = 0;
                    game_time_sync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    
                    self.send(&client_state_s.peer_id, game_time_sync.as_message()).await?;
                }

                {
                    let mut realm_time_sync = CPktServerNotify::default();
                    realm_time_sync.notify_type = 19;
                    realm_time_sync.field_4 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                    self.send(&client_state_s.peer_id, realm_time_sync.as_message()).await?;
                }

                // Update loadstate
                {
                    client_state_s.load_state = ClientLoadState::Spawned;
                    
                    let mut connectionstate = oaPktS2XConnectionState::default();
                    connectionstate.field_1 = ClientLoadState::Spawned.into();
                    connectionstate.field_2 = 0;

                    self.send(&client_state_s.peer_id, connectionstate.as_message()).await?;
                }

                // Resend the player avatar state
                {
                    // Update and get avatar data
                    let (id, name, params, pos) = {
                        let mut param_buffer = Vec::new();
                        let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                        let world_state = self.read().await;
                        let zone = world_state.zone.read().await;
                        //let zones = world_state.zones.read().await;
                        //let zone = zones.get(&state.zone)
                        //.ok_or(AnotherlandError::app_err("zone not found"))?;

                        // update player state
                        let character_component = {
                            let mut instance = zone.instance().write().await;
                            let mut entry = instance
                                .entry(client_state_s.entity).ok_or(AnotherlandError::app_err("entity not found"))?;

                            let character_component = entry.get_component::<AvatarComponent>().unwrap().to_owned();

                            let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();
                            //player_component.set_spawn_mode(2);
                            player_component.set_client_ready(true);
                            player_component.set_player_loading(false);
                            player_component.set_player_node_state(2);
                            player_component.set_world_map_guid(&world_state.worlddef.guid.to_string());
                            player_component.set_zone(&zone.zonedef().zone.to_string());
                            player_component.set_zone_guid(client_state_s.zone.clone());

                            character_component
                        };

                        let params = PlayerParam::from_component(zone.instance().write().await.deref_mut(), client_state_s.entity)?;
                        params.write(&mut writer)?;

                        (
                            character_component.id,
                            character_component.name,
                            param_buffer,
                            PositionUpdate {
                                pos: params.pos().unwrap().to_owned().into(),
                                rot: params.rot().unwrap().to_owned().into(),
                                vel: character_component.vel.into(),
                            }.to_bytes()
                        )
                    };

                    let mut avatar_update = CPktAvatarUpdate::default();
                    avatar_update.full_update = false;
                    avatar_update.avatar_id = Some(id.as_u64());
                    avatar_update.param_bytes = params.len() as u32;
                    avatar_update.params = params;
    
                    self.send(&client_state_s.peer_id, avatar_update.as_message()).await?;
                }

                // Tell the client the avatar is ready to spawn
                {
                    let zone = world_state.zone.read().await;
                    let instance = zone.instance().read().await;

                    if let Some(player) = instance.entry_ref(client_state_s.entity).ok() {
                        let player_component = player.get_component::<PlayerComponent>().unwrap();

                        let mut action = oaPktServerAction::default();
                        action.action = "TRAVEL:NonPortalTravel|NonPortalTravelDefault".to_owned();
                        action.version = 4;
                        action.override_teleport = false;
                        action.pos = player_component.pos().unwrap().to_owned().into();
                        action.rot = player_component.rot().unwrap().to_owned().into();
                        self.send(&client_state_s.peer_id, action.as_message()).await?;
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
                channel: RealmChannel::NodeChannel { zone_guid: self.2.clone() }
            }
        ]
    }
}
