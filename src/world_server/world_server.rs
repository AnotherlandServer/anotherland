use std::{collections::{HashMap, HashSet, VecDeque}, cell::RefCell, sync::Arc, ops::{DerefMut, Deref}};

use bitstream_io::{ByteWriter, LittleEndian};
use glam::Vec3;
use legion::Entity;
use tokio::sync::{RwLock, Mutex};
use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, RakNetRequest, Message}, CPkt, CPktResourceNotify, CpktResourceNotifyResourceType, Uuid, AvatarId, PositionUpdate, ParamClass, CPktBlob, Player, PlayerComponent, PlayerParam, BoundParamClass};
use log::{debug, info, kv::{ToValue, Value}, error, trace};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use atlas::ParamEntity;

use crate::{world::{Zone, self, load_zone_from_definition, CharacterComponent}, db::ZoneDef};
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
    //avatar: Arc<RwLock<PlayerAvatar>>,
    zone: Uuid,
    interest_list: HashSet<AvatarId>,
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

                    /*let avatar = self.clone().read().await
                        .world.get_zone(&state.zone).unwrap()
                        .get_avatar(&state.avatar_id).unwrap();
                    let mut avatar_s = avatar.write().await;*/

                    let world_state = self.read().await;
                    let zones = world_state.zones.read().await;
                    let zone = zones.get(&state.zone)
                    .ok_or(AnotherlandError::app_err("zone not found"))?;

                    // update player state
                    let character_component = {
                        let mut instance = zone.instance().write().await;
                        let mut entry = instance
                            .entry(state.entity).ok_or(AnotherlandError::app_err("entity not found"))?;

                        let character_component = entry.get_component::<CharacterComponent>().unwrap().to_owned();

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

                    /*{
                        match avatar_s.deref_mut() {
                            Avatar::Player(player) =>  {
                                let params = player.player_param_mut();
                                params.set_spawn_mode(2);
                                params.set_client_ready(false);
                                params.set_player_loading(true);
                                params.set_player_node_state(2);
                                params.set_world_map_guid(self.read().await.worlddef.guid.clone());
                                params.set_zone(self.read().await.world.get_zone(&state.zone).unwrap().zonedef.zone.clone());
                                params.set_zone_guid(state.zone.clone());

                                params.write(&mut writer)?;

                                (
                                    player.name().clone(), 
                                    param_buffer,
                                    PositionUpdate {
                                        pos: player.position().into(),
                                        rot: player.rotation().into(),
                                        vel: player.velocity().into(),
                                    }.to_bytes()
                                )
                            },
                            _ => panic!("Client avatar is not of type player"),
                        }
                    }*/
                };

                // Transfer character to client
                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_id = state.avatar_id.as_u64();
                avatar_blob.avatar_name = name;
                avatar_blob.class_id = 77;
                avatar_blob.param_bytes = params.len() as u32;
                avatar_blob.params = params;
                avatar_blob.movement_bytes = pos.len() as u32;
                avatar_blob.movement = pos;
                avatar_blob.has_guid = true;
                avatar_blob.field_9 = Some(state.session.id.clone());

                self.send(&state.peer_id, avatar_blob.as_message()).await?;

                Ok(())
            },

            _ => {
                debug!("Unhandled message: {:#?}", request);
                Ok(())
            }
        }
    }

    async fn update_interest_list(&self, id: AvatarId, pos: Vec3) {
        // loop trough player avatars to update interested clients
        for (_, state) in self.read().await.client_state.iter() {
            let state = state.read().await;

            // are we within the interest area?
            

            if state.interest_list.contains(&id) {

            }
        }
    }

    async fn remove_from_interest_list(&self, state: &mut ClientState, id: AvatarId) {

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

                                // Lookup session
                                let session = Session::get(cluster_database().await, &session_id).await?
                                    .ok_or(AnotherlandError::app_err( "unknown session"))?;

                                // Lookup character
                                let character = Character::get(realm_database().await,
                                    &session.character_id.ok_or(AnotherlandError::app_err("no character selected"))?).await?
                                    .ok_or(AnotherlandError::app_err("character not found"))?;
                            
                                trace!("Spawn character: {}", character.name);

                                let (avatar_id, entity, zone_id) = {
                                    trace!("Character: {:#?}", character.data.as_anyclass().as_hashmap());

                                    let mut world_state = world_state.read().await;
                                    let mut zones = world_state.zones.write().await;
                                    let zone = zones.get_mut(
                                        character.data.zone_guid().ok_or(AnotherlandError::app_err("character zone not found"))?
                                        ).ok_or(AnotherlandError::app_err("zone not found"))?;

                                    // Tho we spawn the avatar here, we'll only tell other clients once
                                    // the player finished loading.
                                    let (avatar_id, entity) = zone.spawn_avatar(world::AvatarType::Player, character.data.to_entity()).await;
                                    zone.instance().write().await.entry(entity).unwrap().add_component(CharacterComponent { 
                                        name: character.name.clone(),
                                        vel: Vec3::default(),
                                    });
                                   
                                    (avatar_id, entity, zone.zonedef().guid.clone())
                                };

                                // Create client state
                                let state = Arc::new(RwLock::new(ClientState { 
                                    account: Account::get_by_id(cluster_database().await, &session.account).await?
                                        .ok_or(AnotherlandError::app_err("account gone"))?, 
                                    session,
                                    peer_id,
                                    avatar_id,
                                    entity,
                                    //avatar_id,
                                    //avatar,
                                    zone: zone_id,
                                    interest_list: HashSet::new(),
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
