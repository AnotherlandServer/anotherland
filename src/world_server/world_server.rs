use std::{collections::{HashMap, HashSet, VecDeque}, cell::RefCell, sync::Arc, ops::{DerefMut, Deref}};

use tokio::sync::{RwLock, Mutex};
use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, RakNetRequest, Message}, CPkt, CPktResourceNotify, CpktResourceNotifyResourceType, Uuid, AvatarId};
use log::{debug, info, kv::{ToValue, Value}, error, trace};
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{db::{WorldDef, DatabaseRecord, realm_database, Account, Session, cluster_database, Character}, world::{World, AvatarEvent, PlayerAvatar}, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue}, util::{AnotherlandResult, AnotherlandError, AnotherlandErrorKind}};

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

#[derive(Clone)]
struct ClientState {
    account: Account,
    session: Session,
    peer_id: Uuid,
    avatar_id: AvatarId,
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
        state.serialize_field("avatar_id", &self.avatar_id)?;
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
    world: World,
    frontend: MessageQueueProducer,
    client_state: HashMap<Uuid, Arc<RwLock<ClientState>>>,
    player_avatar_events: VecDeque<AvatarEvent>,
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

                Ok(())

                // Lookup character
                /*let mut character = match Character::get(self.realm_db.clone(), &state.session.character_id.unwrap()).await? {
                    Some(character) => Ok(character),
                    None => Err(AnotherlandError::new(ApplicationError, "selected character not found"))
                }?;

                // Set spawn mode to Login_Normal
                character.data.set_spawn_mode(2);
                character.data.set_client_ready(true);
                character.data.set_player_loading(true);
                character.data.set_player_node_state(2);
                character.data.set_world_map_guid(self.worlddef.guid.clone());

                state.zone_guid = character.data.zone_guid().map(|v| v.to_owned());

                // Spawn avatar
                let zone = self.world.get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap();
                let avatar_id = zone.spawn_avatar(PlayerAvatar::new(character.clone()).into());
                let avatar = zone.get_avatar(&avatar_id).unwrap();

                state.avatar_id = Some(avatar_id.clone());

                // serialize data
                let mut params = Vec::new();
                let mut writer = ByteWriter::endian(&mut params, LittleEndian);
                character.data.write(&mut writer)?;
                let pos = PositionUpdate {
                    pos: avatar.position().into(),
                    rot: avatar.rotation().into(),
                    vel: avatar.velocity().into(),
                }.to_bytes();

                // Transfer character to client
                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_id = avatar_id.as_u64(); // local client always uses 1 for their avatar id
                avatar_blob.avatar_name = character.name.clone();
                avatar_blob.class_id = 77;
                avatar_blob.param_bytes = params.len() as u32;
                avatar_blob.params = params;
                avatar_blob.movement_bytes = pos.len() as u32;
                avatar_blob.movement = pos;
                avatar_blob.has_guid = true;
                avatar_blob.field_9 = Some(state.session.id.clone());

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatar_blob.as_message()).await?;*/
            },

            _ => {
                debug!("Unhandled message: {:#?}", request);
                Ok(())
            }
        }
    }
}

#[async_trait]
impl ServerInstance for WorldServer {
    type ServerProperties = WorldServerOptions;

    async fn init(properties: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        let db = realm_database().await;

        let worlddef = WorldDef::get(db, &properties.world_id).await?.unwrap();
        let world = World::load_from_id(properties.world_id).await?;

        let (frontend, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: properties.realm_id, 
            channel: RealmChannel::FrontendChannel 
        }).await?;

        Ok(Box::new(Self(Arc::new(RwLock::new(WorldServerData {
            realm_id: properties.realm_id,
            worlddef,
            world,
            frontend,
            client_state: HashMap::new(),
            player_avatar_events: VecDeque::new(),
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

                                // Spawn character in world
                                let avatar_id = {
                                    let mut world_state = world_state.write().await;
                                    let zone = world_state.world.get_zone_mut(
                                        character.data.zone_guid().ok_or(AnotherlandError::app_err("character zone not found"))?
                                        ).ok_or(AnotherlandError::app_err("zone not found"))?;

                                    // Tho we spawn the avatar here, we'll only tell other clients once
                                    // the player finished loading.
                                    zone.spawn_avatar(PlayerAvatar::new(character).into())
                                };

                                // Create client state
                                let state = Arc::new(RwLock::new(ClientState { 
                                    account: Account::get_by_id(cluster_database().await, &session.account).await?
                                        .ok_or(AnotherlandError::app_err("account gone"))?, 
                                    session,
                                    peer_id,
                                    avatar_id,
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