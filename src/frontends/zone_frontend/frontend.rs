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

use std::{borrow::Borrow, collections::{HashMap, HashSet, VecDeque}, net::{Ipv6Addr, SocketAddr}, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use async_trait::async_trait;
use atlas::{oaPktMoveManagerPosUpdate, oaPktS2XConnectionState, oaPkt_Combat_HpUpdate, oaPkt_SplineSurfing_Acknowledge, raknet::Message, AvatarId, CPkt, CPktAvatarClientNotify, CPktAvatarUpdate, CPktBlob, CPktChat, CPktResourceNotify, CPktServerNotify, CpktChatChatType, CpktResourceNotifyResourceType, CpktServerNotifyNotifyType, MoveManagerInit, NativeParam, OaZoneConfigParams, ParamAttrib, ParamClass, ParamSet, PlayerAttribute, PlayerClass, PlayerParams, Uuid, UUID_NIL};
use bitstream_io::{ByteWriter, LittleEndian};
use glam::{Quat, Vec3};
use log::{debug, error, trace, warn, info};
use quinn::ServerConfig;
use tokio::{sync::{Mutex, mpsc::{self, Sender}, RwLock}, time, select};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{actors::{AvatarEvent, Movement, PhysicsState, PlayerSpawnMode, ProximityChatRange, ServerAction, Zone, ZoneEvent, ZoneRegistry}, cluster::{frontend::Frontend, ActorRef, CheatMessage}, components::{SessionHandler, SessionRef, ZoneFactory}, db::{realm_database, Character, ItemContent, WorldDef, ZoneDef}, util::{AnotherlandErrorKind, AnotherlandResult}, CLUSTER_CERT, NODE};
use crate::db::DatabaseRecord;

use super::{load_state::ClientLoadState, TravelType, ZoneDownstreamMessage, ZoneServerListener, ZoneUpstreamMessage};

#[derive(Clone)]
enum ZoneInstance {
    Persistent(ActorRef<Zone>),
    Instance(ActorRef<Zone>),
}

impl ZoneInstance {
    pub fn zone(&mut self) -> &mut ActorRef<Zone> {
        match self {
            Self::Persistent(zone) => zone,
            Self::Instance(zone) => zone,
        }
    }
}

pub struct ZoneFrontend {
    name: String,
    factory: ZoneFactory,
    instances: Arc<Mutex<HashMap<Uuid, ZoneInstance>>>,
    tasks: TaskTracker,
}

impl ZoneFrontend {
    pub async fn initialize(world_def: WorldDef, zone_def: ZoneDef) -> AnotherlandResult<Self> {
        Ok(Self {
            name: format!("zone_server_{}", zone_def.guid),
            factory: ZoneFactory::new(realm_database().await, world_def, zone_def).await?,
            instances: Arc::new(Mutex::new(HashMap::new())),
            tasks: TaskTracker::new(),
        })
    }
}

#[async_trait]
impl Frontend for ZoneFrontend {
    fn name(&self) -> &str { self.name.as_str() }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let cert_der = CLUSTER_CERT.serialize_der().unwrap();
        let priv_key = CLUSTER_CERT.serialize_private_key_der();

        let priv_key = rustls::PrivateKey(priv_key);
        let cert_chain = vec![rustls::Certificate(cert_der)];

        // todo: implement binding to something else than LOCALHOST
        // to support running zones distributed on multiple machines.
        let mut listener = ZoneServerListener::listen(
            ServerConfig::with_single_cert(cert_chain, priv_key).unwrap(), 
            SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 0)).await.unwrap();

        let mut zone_registry = NODE.get_remote_actor::<ZoneRegistry>("zone_registry").ok_or(AnotherlandErrorKind::Application)?;
        let mut registration_update_interval = time::interval(Duration::from_secs(1));

        let session_handler = SessionHandler::new();

        'accept_loop: loop {
            select! {
                res = listener.accept() => {
                    if let Some(mut connection) = res {
                        let tasks = self.tasks.clone();
                        let token = token.clone();
                        let factory = self.factory.clone();
                        let instances = self.instances.clone();
                        let session_handler = session_handler.clone();
        
                        let mut sessions = HashMap::new();
        
                        self.tasks.spawn(async move {
                            let (downstream_sender, mut downstream_receiver) = mpsc::channel(100);
                            let connection_token = CancellationToken::new();
        
                            'accept_loop: loop {
                                select! {
                                    message = connection.recv() => {
                                        match message.as_ref() {
                                            Some(ZoneUpstreamMessage::EnterZone { session_id, avatar_id }) => {
                                                debug!("Session {} entering zone with avatar_id {}", session_id, avatar_id);

                                                // if this zone is instanced, spin up a new zone and move the player to that
                                                // use the primary zone otherwise.
                                                let zone = if factory.config().force_generate_guid_key() {
                                                    debug!("Spinning up new instance of zone {}", factory.zone_def().guid);

                                                    ZoneInstance::Instance(factory.spawn_zone().await)
                                                } else {
                                                    let mut instances_s = instances.lock().await;

                                                    if let Some(zone) = instances_s.get(&UUID_NIL) {
                                                        zone.clone()
                                                    } else {
                                                        debug!("Spinning up persistent zone {}", factory.zone_def().guid);

                                                        let zone = ZoneInstance::Persistent(factory.spawn_zone().await);
                                                        instances_s.insert(UUID_NIL, zone.clone());

                                                        zone
                                                    }
                                                };

                                                sessions.insert(*session_id, ZoneSession::spawn(
                                                    tasks.clone(), 
                                                    connection_token.clone(), 
                                                    session_id, 
                                                    avatar_id, 
                                                    zone,
                                                    factory.clone(),
                                                    session_handler.clone(),
                                                    downstream_sender.clone()).await);
                                            },
                                            Some(ZoneUpstreamMessage::Travel { session_id, .. }) => {
                                                if let Some(session) = sessions.get(session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::Message { session_id, .. }) => {
                                                if let Some(session) = sessions.get(session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::LeaveZone { session_id }) => {
                                                debug!("Session {} leaving zone", session_id);

                                                if let Some(session) = sessions.remove(session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            None => {
                                                connection_token.cancel();
                                                downstream_receiver.close();
                                                break 'accept_loop;
                                            },
                                        }
                                    },
                                    Some(message) = downstream_receiver.recv() => {
                                        if let Err(e) = connection.send(&message).await {
                                            error!("Downstream send error: {:#?}", e);
                                            connection.close().await;
                                            downstream_receiver.close();
                                        }
                                    },
                                    _ = token.cancelled() => {
                                        connection.close().await;
                                        downstream_receiver.close();
                                    }
                                }
                            }

                            trace!("Stopping zone server <-> cluster connection loop");
        
                            connection_token.cancel();
                        });
                    } else {
                        break 'accept_loop;
                    }
                },
                _ = registration_update_interval.tick() => {
                    zone_registry.register_zone_frontend(self.factory.zone_def().guid, listener.local_addr().unwrap()).await;
                },
                _ = token.cancelled() => {
                    break 'accept_loop;
                }
            }
        }

        Ok(())
    }
}

struct ZoneSession {
    session_id: Uuid,
    avatar_id: AvatarId,
    zone_factory: ZoneFactory,
    session_handler: Arc<RwLock<SessionHandler>>,
    session_ref: SessionRef,
    downstream: Sender<ZoneDownstreamMessage>,
    load_state: ClientLoadState,
    instance: ZoneInstance,
    
    avatar_event_sender: mpsc::Sender<AvatarEvent>,
    avatar_events: Option<mpsc::Receiver<AvatarEvent>>,
    interest_list: HashSet<AvatarId>,
    interest_added_queue: VecDeque<AvatarId>,
    interest_removed_queue: VecDeque<AvatarId>,
    ignore_interest_updates: bool,

    server_actions: VecDeque<ServerAction>,

    target_avatar: Option<AvatarId>,

    dont_save_on_disconnect: bool,
}

impl ZoneSession {
    async fn spawn(tasks: TaskTracker, token: CancellationToken, session_id: &Uuid, avatar_id: &AvatarId, 
        instance: ZoneInstance, zone_factory: ZoneFactory, session_handler: Arc<RwLock<SessionHandler>>,
        downstream: Sender<ZoneDownstreamMessage>) -> Sender<ZoneUpstreamMessage> {

        // todo: handle errors here, as this might be a race when the session
        // invalidates during the zone enter stage.
        let session_ref = session_handler.write().await.initiate_trusted(*session_id, *session_id).await.unwrap();

        let (avatar_event_sender, avatar_event_receiver) = mpsc::channel(100);

        let session = ZoneSession {
            session_id: session_id.to_owned(),
            avatar_id: avatar_id.to_owned(),
            zone_factory,
            session_handler,
            session_ref,
            instance,
            downstream,
            load_state: ClientLoadState::Offline,
            avatar_event_sender,
            avatar_events: Some(avatar_event_receiver),
            interest_list: HashSet::new(),
            interest_added_queue: VecDeque::new(),
            interest_removed_queue: VecDeque::new(),
            ignore_interest_updates: false,
            server_actions: VecDeque::new(),
            target_avatar: None,
            dont_save_on_disconnect: false,
        };

        session.run(&tasks, token)
    }

    fn run(mut self, tasks: &TaskTracker, token: CancellationToken) -> Sender<ZoneUpstreamMessage> {
        let (request_sender, mut request_receiver) = mpsc::channel(100);

        tasks.spawn(async move {
            let session_ref = self.session_ref.clone();
            let mut zone_events = self.instance.zone().subscribe().await;
            let mut avatar_event_receiver = self.avatar_events.take().unwrap();
            let mut update_timer = time::interval(Duration::from_millis(250));

            'net_loop: loop {
                select! {
                    request = request_receiver.recv() => {
                        if let Some(request) = request {
                            match request {
                                ZoneUpstreamMessage::EnterZone { .. } => unreachable!(),
                                ZoneUpstreamMessage::Travel { destination, .. } => {
                                    if let Err(e) = self.travel_to_zone(destination).await {
                                        error!(
                                            session = self.session_id.to_string(), 
                                            avatar = self.avatar_id.to_string(); 
                                            "Error while travelling: {:#?}", e);
                                        break 'net_loop;
                                    }
                                },
                                ZoneUpstreamMessage::Message { message, ..} => {
                                    if let Err(e) = self.handle_message(Message::from_bytes(&message).unwrap().1).await {
                                        error!(
                                            session = self.session_id.to_string(), 
                                            avatar = self.avatar_id.to_string(); 
                                            "Error while handling message: {:#?}", e);
                                        break 'net_loop;
                                    }
                                }
                                ZoneUpstreamMessage::LeaveZone { .. } => break 'net_loop,
                            }
                            
                        } else {
                            break 'net_loop;
                        }
                    },
                    event = zone_events.recv() => {
                        match event {
                            Ok(event) => {
                                let _ = self.handle_zone_event(event).await;
                            },
                            Err(e) => {
                                error!(
                                    session = self.session_id.to_string(), 
                                    avatar = self.avatar_id.to_string(); 
                                    "Zone event error: {:#?}", e);
                                break 'net_loop;
                            }
                        }
                    },
                    Some(event) = avatar_event_receiver.recv() => {
                        if let Err(e) = self.handle_avatar_event(event).await {
                            error!(
                                session = self.session_id.to_string(), 
                                avatar = self.avatar_id.to_string(); 
                                "Avatar event error: {:#?}", e);
                            break 'net_loop;
                        }  
                    },
                    _ = update_timer.tick() => {
                        if let Err(e) = self.update().await {
                            error!(
                                session = self.session_id.to_string(), 
                                avatar = self.avatar_id.to_string(); 
                                "Update error: {:#?}", e);
                            break 'net_loop;
                        } 
                    }
                    _ = session_ref.invalidated() => {
                        warn!(
                            session = self.session_id.to_string(), 
                            avatar = self.avatar_id.to_string(); 
                            "Session invalidated!");
                        break 'net_loop;
                    },
                    _ = token.cancelled() => {
                        break 'net_loop;
                    }
                }
            }

            // despawn and save avatar
            if let Some(player) = self.instance.zone().despawn_player(self.avatar_id).await {
                if !self.dont_save_on_disconnect {
                    let session = self.session_ref.lock().await;
                    let db = realm_database().await;
    
                    if let Some(mut character) = Character::get(db.clone(), &session.session().character_id.unwrap()).await.unwrap() {
                        character.data = player;
                        character.save(db).await.unwrap();
                    } else {
                        error!("Despawned character {:?} does not exist anymore!", session.session().character_id);
                    }
                }
            }

            // stop instance
            if let ZoneInstance::Instance(zone) = self.instance {
                zone.stop();
            }
            
            trace!(
                session = self.session_id.to_string(), 
                avatar = self.avatar_id.to_string(); 
                "Stopping zone frontend session");
        });

        request_sender
    }

    async fn travel_to_zone(&mut self, destination: TravelType) -> AnotherlandResult<()> {
        let session_s = self.session_ref.lock().await;

        // Spawn player character
        let mut param_buffer = Vec::new();
        let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

        let (player, server_action) = self.instance.zone().spawn_player(
            match destination {
                TravelType::EntryPoint => PlayerSpawnMode::TravelDirect,
                TravelType::Portal { uuid } => PlayerSpawnMode::TravelPortal(uuid),
                _ => unimplemented!(),
            }, 
            self.avatar_id, 
            session_s.session().character_id.unwrap(), 
            self.avatar_event_sender.clone()).await?;

        self.server_actions.push_back(server_action);

        player.data.write_to_client(&mut writer)?;

        info!(
            session = self.session_id.to_string(), 
            avatar = self.avatar_id.to_string(); 
            "Spawning player: {}", player.name);

        let movement = self.instance.zone().get_avatar_move_state(self.avatar_id).await.unwrap();

        let mut data = Vec::new();
        {
            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
            player.data.write_to_client(&mut writer)?;
        }

        // Set loading state
        self.send(oaPktS2XConnectionState {
            field_1: ClientLoadState::Transition.into(),
            field_2: 0,
            ..Default::default()
        }.into_message()).await?;

        // Send resource notification so pathengine can initialize
        let _ = self.send(CPktResourceNotify {
            resource_type: CpktResourceNotifyResourceType::WorldDef,
            field_2: self.zone_factory.world_def().guid,
            field_3: "".to_owned(),
            ..Default::default()
        }.into_message()).await;

        // Update player character on client
        self.send(CPktAvatarUpdate {
            full_update: true,
            avatar_id: Some(self.avatar_id.as_u64()),
            field_2: Some(false),
            name: Some(player.name),
            class_id: Some(PlayerAttribute::class_id().into()),
            field_6: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()),
            params: data.into(),
            update_source: 0,
            movement: Some(MoveManagerInit {
                pos: movement.position.into(),
                rot: movement.rotation.into(),
                vel: movement.velocity.into(),
                physics: PhysicsState::Walking.into(),
                ..Default::default()
            }.to_bytes().into()),
            ..Default::default()
        }.into_message()).await?;

        Ok(())
    }

    async fn handle_avatar_event(&mut self, event: AvatarEvent) -> AnotherlandResult<()> {
        match event {
            AvatarEvent::InterestAdded { ids } => {
                if !self.ignore_interest_updates {
                    self.interest_removed_queue = self.interest_removed_queue.drain(..).filter(|v| !ids.contains(v)).collect();
                    self.interest_added_queue.append(&mut ids.into_iter().collect());
                }
            },
            AvatarEvent::InterestRemoved { ids } => {
                if !self.ignore_interest_updates {
                    self.interest_added_queue = self.interest_added_queue.drain(..).filter(|v| !ids.contains(v)).collect();
                    self.interest_removed_queue.append(&mut ids.into_iter().collect());
                }
            },
            AvatarEvent::Travel { zone, destination } => {
                self.travel(zone, destination).await?;
            },
            AvatarEvent::Message(message) => {
                self.send(message).await?;
            },
            AvatarEvent::ServerAction(action) => {
                self.server_actions.push_back(action);
            },
            AvatarEvent::ChatMessage { range, sender, message } => {
                self.send(CPktChat {
                    chat_type: match range {
                        ProximityChatRange::Say => CpktChatChatType::Say,
                        ProximityChatRange::TeamSay => CpktChatChatType::Say,
                        ProximityChatRange::Shout => CpktChatChatType::Shout,
                    },
                    message,
                    sender,
                    ..Default::default()
                }.into_message()).await?;
            }
        }

        Ok(())
    }

    async fn handle_zone_event(&mut self, event: Arc<ZoneEvent>) -> AnotherlandResult<()> {
        match event.as_ref() {
            ZoneEvent::AvatarSpawned { avatar_id, .. } => {
                if *avatar_id != self.avatar_id {
                    // todo: check if we need to add the character to our interest list
                }
            },
            ZoneEvent::AvatarUpdated { avatar_id, params } => {
                if self.interest_list.contains(avatar_id) || self.avatar_id == *avatar_id {
                    let mut param_buffer = Vec::new();
                    let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                    params.write_to_client(&mut writer)?;
    
                    self.send(CPktAvatarUpdate {
                        full_update: false,
                        avatar_id: Some(avatar_id.as_u64()),
                        update_source: 0,
                        params: param_buffer.into(),
                        ..Default::default()
                    }.into_message()).await?;
                }
            },
            ZoneEvent::AvatarMoved { avatar_id, movement } => {
                if self.interest_list.contains(avatar_id) {
                    self.send(oaPktMoveManagerPosUpdate {
                        pos: movement.position.into(),
                        rot: movement.rotation.into(),
                        vel: movement.velocity.into(),
                        physics: movement.physics_state.into(),
                        mover_key: movement.mover_key,
                        avatar_id: avatar_id.as_u64(),
                        seconds: movement.seconds,
                        ..Default::default()
                    }.into_message()).await?;
                }
            },
            ZoneEvent::AvatarDespawned { avatar_id } => {
                if self.interest_list.remove(avatar_id) {
                    // tell client the avatar despawned
                    self.send(CPktAvatarClientNotify {
                        avatar_id: avatar_id.clone().as_u64(),
                        ..Default::default()
                    }.into_message()).await?;
                }
            },
            ZoneEvent::CombatHpUpdate { avatar_id, hp } => {
                self.send(oaPkt_Combat_HpUpdate {
                    avatar_id: avatar_id.as_u64(),
                    hp: *hp,
                    ..Default::default()
                }.into_message()).await?;
            }
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> AnotherlandResult<()> {
        use Message::*;

        match message {
            AtlasPkt(CPkt::oaPktRequestEnterGame(_pkt)) => {
                let session_s = self.session_ref.lock().await;

                // Set loading state
                self.send(oaPktS2XConnectionState {
                    field_1: ClientLoadState::Transition.into(),
                    field_2: 0,
                    ..Default::default()
                }.into_message()).await?;

                // Send resource notification 
                let _ = self.send(CPktResourceNotify {
                    resource_type: CpktResourceNotifyResourceType::WorldDef,
                    field_2: self.zone_factory.world_def().guid,
                    field_3: "".to_owned(),
                    ..Default::default()
                }.into_message()).await;

                // Spawn player character
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                let (player, server_action) = self.instance.zone().spawn_player(PlayerSpawnMode::LoginNormal, self.avatar_id, session_s.session().character_id.unwrap(), self.avatar_event_sender.clone()).await?;
                self.server_actions.push_back(server_action);

                player.data.write_to_client(&mut writer)?;

                let movement = self.instance.zone().get_avatar_move_state(self.avatar_id).await.unwrap();

                info!(
                    session = self.session_id.to_string(), 
                    avatar = self.avatar_id.to_string(); 
                    "Spawning player: {}", player.name);

                self.send(CPktBlob {
                    avatar_id: self.avatar_id.as_u64(),
                    avatar_name: player.name,
                    class_id: PlayerAttribute::class_id().into(),
                    params: param_buffer.into(),
                    movement: MoveManagerInit {
                        pos: movement.position.into(),
                        rot: movement.rotation.into(),
                        vel: movement.velocity.into(),
                        physics: PhysicsState::Walking.into(),
                        mover_type: 1,
                        replica: 7,
                        ..Default::default()
                    }.to_bytes().into(),
                    has_guid: true,
                    field_7: Some(self.session_id),
                    ..Default::default()
                }.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                // Until we have to means to verify this request, we just clone the message
                // and set version to 2, accepting the action that way.
                let mut action = pkt.clone();
                action.version = 2;
                self.send(action.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                self.load_state = match pkt.field_1 {
                    0 => ClientLoadState::Offline,
                    1 => ClientLoadState::Transition,
                    2 => ClientLoadState::PlayerReceived,
                    3 => ClientLoadState::MapLoaded,
                    4 => ClientLoadState::PlayerLoaded,
                    5 => ClientLoadState::WaitingForInitialInterests,
                    6 => ClientLoadState::ReceivedInitialInterests,
                    7 => ClientLoadState::InitialInterestsLoaded,
                    8 => ClientLoadState::InGame,
                    _ => {
                        warn!(
                            session = self.session_id.to_string(), 
                            avatar = self.avatar_id.to_string(); 
                            "Invalid client loadstate: {}", pkt.field_1);
                        ClientLoadState::Offline
                    }
                };

                debug!(
                    session = self.session_id.to_string(), 
                    avatar = self.avatar_id.to_string();
                    "New connection state = {:?}", self.load_state
                );

                // Confirm loading state
                let mut response = pkt.clone();
                response.field_1 = self.load_state.clone().into();
                response.field_2 = pkt.field_2 + 1;

                self.send(response.into_message()).await?;
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {

                        self.instance.zone().move_player_avatar(
                            self.avatar_id, 
                            Movement {
                                position: pkt.pos.into(),
                                rotation: pkt.rot.into(),
                                velocity: pkt.vel.into(),
                                physics_state: pkt.physics.into(),
                                mover_key: pkt.mover_key,
                                seconds: pkt.seconds,
                            }
                        ).await;
                    },
                    _ => {
                        warn!("Unhandled routed packet: {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                    },
                }
            },
            AtlasPkt(CPkt::oaPktAvatarTellBehaviorBinary(pkt)) => {
                match pkt.field_3.as_str() {
                    "doVendorExecute" => {
                        match &pkt.field_4 {
                            NativeParam::Struct(attrib) => {
                                let (_, params) = self.instance.zone().get_avatar_params(self.avatar_id).await.unwrap();
                                let db: mongodb::Database = realm_database().await;

                                let mut player_params = params.take::<PlayerClass>().unwrap();

                                player_params.set_customization_gender(attrib[0].to_f32()?);
                                player_params.set_customization_height(attrib[1].to_f32()?);
                                player_params.set_customization_fat(attrib[2].to_f32()?);
                                player_params.set_customization_skinny(attrib[3].to_f32()?);
                                player_params.set_customization_muscular(attrib[4].to_f32()?);
                                player_params.set_customization_bust_size(attrib[5].to_f32()?);
                                player_params.set_race(attrib[6].to_i32()?);
                                player_params.set_customization_brow_angle(attrib[7].to_f32()?);
                                player_params.set_customization_eye_brow_pos(attrib[8].to_f32()?);
                                player_params.set_customization_eye_pos_spacing(attrib[9].to_f32()?);
                                player_params.set_customization_eye_pos(attrib[10].to_f32()?);
                                player_params.set_customization_eye_size_length(attrib[11].to_f32()?);
                                player_params.set_customization_eye_size_width(attrib[12].to_f32()?);
                                player_params.set_customization_eyes_pretty(attrib[13].to_f32()?);
                                player_params.set_customization_mouth_pos(attrib[14].to_f32()?);
                                player_params.set_customization_mouth_width(attrib[15].to_f32()?);
                                player_params.set_customization_mouth_lower_lip_thic(attrib[16].to_f32()?);
                                player_params.set_customization_mouth_upper_lip_thic(attrib[17].to_f32()?);
                                player_params.set_customization_mouth_expression(attrib[18].to_f32()?);
                                player_params.set_customization_nose_pos_length(attrib[19].to_f32()?);
                                player_params.set_customization_nose_pos_width(attrib[20].to_f32()?);
                                player_params.set_customization_nose_portude(attrib[21].to_f32()?);
                                player_params.set_customization_ear_size(attrib[22].to_f32()?);
                                player_params.set_customization_ear_elf(attrib[23].to_f32()?);
                                player_params.set_customization_cheek_bone(attrib[24].to_f32()?);
                                player_params.set_customization_cheek(attrib[25].to_f32()?);
                                player_params.set_customization_chin_portude(attrib[26].to_f32()?);
                                player_params.set_customization_jaw_chubby(attrib[27].to_f32()?);
                                debug!("Attrib 28: {}", attrib[28].to_string()?);
                                debug!("Attrib 29: {:#?}", attrib[29]);

                                let mut visible_items = Vec::new();
                                for a in attrib[30..].iter() {
                                    let item_uuid = a.to_uuid()?;
                                    debug!("Load item {}", item_uuid.to_string());
                                let db: mongodb::Database = realm_database().await;
                                let item = ItemContent::get(db.clone(), &item_uuid).await?;
                                    visible_items.push(item.unwrap().id as i32);
                                }
    
                                if !visible_items.is_empty() {
                                    debug!("set visible item info");
                                    player_params.set_visible_item_info(visible_items);
                                } else {
                                    debug!("received empty visible item info after metamorph");
                                }

                                // Save changes
                                debug!("Save avatar change");
   
                                let mut character = Character::get(db.clone(), self.session_ref.lock().await.session().character_id.as_ref().unwrap()).await.unwrap().unwrap();
                                character.data = player_params.clone();
                                character.save(db.clone()).await?;
        
                                self.instance.zone().update_avatar(self.avatar_id, player_params.into_set().into_box()).await;
                            },
                            _ => panic!(),
                        }
                    },
                    _ => {
                        warn!("Unknown avatar behavior: {:#?}", pkt);
                    }
                }
            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                if pkt.avatar_id.unwrap_or_default() == self.avatar_id.as_u64() {
                    if let Ok((_, params)) = ParamSet::<PlayerAttribute>::read(pkt.params.as_slice()) {
                        self.instance.zone().update_avatar(self.avatar_id, params.into_box()).await;
                    } else {
                        error!(
                            session = self.session_id.to_string(), 
                            avatar = self.avatar_id.to_string(); 
                            "Client sent invalid param update!");
                    }
                } else {
                    error!(
                        session = self.session_id.to_string(), 
                        avatar = self.avatar_id.to_string(); 
                        "Client tried to update unowned avatar #{}", pkt.avatar_id.unwrap_or_default());
                }
            },
            AtlasPkt(CPkt::CPktTargetRequest(pkt)) => {
                if pkt.avatar_id == self.avatar_id.as_u64() {
                    if pkt.target_avatar_id != 0 {
                        self.target_avatar = Some(pkt.avatar_id.into());
                    } else {
                        self.target_avatar = None;
                    }

                    debug!(
                        session = self.session_id.to_string(), 
                        avatar = self.avatar_id.to_string(); 
                        "Selected avatar: {:?}", self.target_avatar);
                }
            },
            AtlasPkt(CPkt::oaPktDialogList(pkt)) => {
                debug!("Dialog List: {:#?}", pkt);
            },
            AtlasPkt(CPkt::CPktRequestAvatarBehaviors(pkt)) => {
                debug!("Request behavior: {:#?}", pkt);

                self.instance.zone().request_behavior(pkt.avatar_id.into(), pkt.behaviour, pkt.data).await;
            },
            AtlasPkt(CPkt::oaPktAvatarTellBehavior(pkt)) => {
                if pkt.instigator != self.avatar_id.as_u64() {
                    warn!("Client tried to instigate behavior on behalf of other avatar: {:#?}", pkt);
                } else {
                    self.instance.zone().tell_behavior(pkt.instigator.into(), pkt.target.into(), pkt.behavior).await;
                }
            },
            AtlasPkt(CPkt::CPktChat(pkt)) => {
                self.instance.zone().proximity_chat(match pkt.chat_type {
                    CpktChatChatType::Say => ProximityChatRange::Say,
                    CpktChatChatType::Shout => ProximityChatRange::Shout,
                    _ => unreachable!(),
                }, self.avatar_id, pkt.message).await;
            },
            AtlasPkt(CPkt::oaPktCheatingClusterNode(pkt)) => {
                debug!("{:#?}", pkt);

                let command = CheatMessage::from_native(pkt.command.clone())?;
                debug!("{:#?}", command);

                match command {
                    CheatMessage::InstantKill { target, .. } => {
                        if target == self.avatar_id || self.session_ref.lock().await.account().is_gm {
                            self.instance.zone().kill_avatar(target).await;
                        }
                    }
                }
            },
            _ => {
                debug!(
                    session = self.session_id.to_string(), 
                    avatar = self.avatar_id.to_string(); 
                    "Unhandled message: {:#?}", message);
            }
        }

        Ok(())
    }

    async fn send(&self, message: Message) -> AnotherlandResult<()> {
        self.downstream.send(ZoneDownstreamMessage::Message { 
            session_id: self.session_id,
            message: message.to_bytes(), 
        }).await.map_err(|_| AnotherlandErrorKind::IO)?;

        Ok(())
    }

    async fn travel(&mut self, zone: Uuid, destination: TravelType) -> AnotherlandResult<()> {
        debug!("Initiating travel for avatar {}. Godspeed!", self.avatar_id);

        // stop accepting interest updates
        self.ignore_interest_updates = true;

        // disable save on disconnect, to avoid racing the destination zone frontend
        self.dont_save_on_disconnect = true;

        // unload all interests
        let interests: Vec<_> = self.interest_list.drain().collect();
        for avatar_id in interests {
            self.send(CPktAvatarClientNotify {
                avatar_id: avatar_id.as_u64(),
                ..Default::default()
            }.into_message()).await?;
        }

        // tell frontend to initiate travel
        self.downstream.send(ZoneDownstreamMessage::RequestTravel { 
            session_id: self.session_id, 
            zone, 
            travel: destination
        })
        .await
        .map_err(|_| AnotherlandErrorKind::IO)?;

        Ok(())
    }

    async fn update(&mut self) -> AnotherlandResult<()> {
        // Only start transmitting interests once the client left
        // the early load sequence.
        if self.load_state >= ClientLoadState::WaitingForInitialInterests {
            // remove avatars we are not interested in anymore
            while let Some(avatar_id) = self.interest_removed_queue.pop_front() {
                self.send(CPktAvatarClientNotify {
                    avatar_id: avatar_id.as_u64(),
                    ..Default::default()
                }.into_message()).await?;

                // remove from interest list
                self.interest_list.remove(&avatar_id);
            }

            // limit to push up to 10 avatars per tick
            for _ in 0..10 {
                if let Some(avatar_id) = self.interest_added_queue.pop_front() {
                    if let Some((name, params)) = self.instance.zone().get_subjective_avatar_params(self.avatar_id, avatar_id).await {
                        let movement = self.instance.zone().get_avatar_move_state(avatar_id).await.unwrap();

                        // add to interest list, so the client will receive updates
                        // for this avatar.
                        self.interest_list.insert(avatar_id);

                        let mut data = Vec::new();
                        {
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            params.write_to_client(&mut writer)?;
                        }

                        self.send(CPktAvatarUpdate {
                            full_update: true,
                            avatar_id: Some(avatar_id.as_u64()),
                            field_2: Some(false),
                            name: Some(name),
                            class_id: Some(params.class_id().into()),
                            field_6: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()),
                            params: data.into(),
                            update_source: 0,
                            movement: Some(MoveManagerInit {
                                pos: movement.position.into(),
                                rot: movement.rotation.into(),
                                vel: movement.velocity.into(),
                                physics: PhysicsState::Walking.into(),
                                ..Default::default()
                            }.to_bytes().into()),
                            ..Default::default()
                        }.into_message()).await?;
                    }
                } else if self.load_state == ClientLoadState::WaitingForInitialInterests {
                    // Update client loading state if we are in the initial streaming phase
                    self.load_state = ClientLoadState::ReceivedInitialInterests;

                    self.send(oaPktS2XConnectionState {
                        field_1: ClientLoadState::ReceivedInitialInterests.into(),
                        field_2: 0,
                        ..Default::default()
                    }.into_message()).await?;
                }
            }
        }

        if self.load_state == ClientLoadState::InitialInterestsLoaded {
            // Synchronize time
            {
                self.send(CPktServerNotify {
                    notify_type: CpktServerNotifyNotifyType::SyncGameClock,
                    field_2: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
                    ..Default::default()
                }.into_message()).await?;
            }

            {
                self.send(CPktServerNotify {
                    notify_type: CpktServerNotifyNotifyType::SyncRealmTime,
                    field_4: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
                    ..Default::default()
                }.into_message()).await?;
            }

            // Update loadstate
            {
                self.load_state = ClientLoadState::InGame;
                
                self.send(oaPktS2XConnectionState {
                    field_1: ClientLoadState::InGame.into(),
                    field_2: 0,
                    ..Default::default()
                }.into_message()).await?;
            }
        }

        if self.load_state == ClientLoadState::InGame {
            while let Some(action) = self.server_actions.pop_front() {
                self.send(action.into_message()).await?;
            }
        }

        Ok(())
    }
}
