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

use std::{collections::{HashMap, HashSet, VecDeque}, net::{Ipv6Addr, SocketAddr}, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use async_trait::async_trait;
use atlas::{oaPktS2XConnectionState, oaQuestCondition, oaQuestTemplate, raknet::Message, AvatarId, CPkt, CPktAvatarClientNotify, CPktAvatarUpdate, CPktBlob, CPktResourceNotify, CPktServerNotify, CPktStream_165_2, CpktChatChatType, CpktResourceNotifyResourceType, CpktServerNotifyNotifyType, MoveManagerInit, OaPktQuestRequestRequest, OaZoneConfigParams, ParamAttrib, ParamClass, ParamSet, PlayerAttribute, PlayerParams, Uuid, UUID_NIL};
use bitstream_io::{ByteWriter, LittleEndian};
use chrono::{Local, TimeZone};
use log::{debug, error, trace, warn, info};
use quinn::ServerConfig;
use tokio::{sync::{Mutex, mpsc::{self, Sender}, RwLock}, time, select};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{actors::{AvatarEvent, AvatarState, Movement, PhysicsState, PlayerSpawnMode, ProximityChatRange, ServerAction, Zone, ZoneRegistry}, cluster::{frontend::Frontend, ActorRef, CheatMessage}, components::{SessionHandler, SessionRef, ZoneFactory}, db::{realm_database, Character, WorldDef, ZoneDef}, scripting::quest::lookup_quest_info, util::{AnotherlandError, AnotherlandErrorKind, AnotherlandResult}, CLUSTER_CERT, NODE};
use crate::db::DatabaseRecord;

use super::{load_state::ClientLoadState, ApiCommand, ApiResult, TravelType, ZoneDownstreamMessage, ZoneServerListener, ZoneUpstreamMessage};

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
        info!("Initializing zone server: {}", zone_def.zone);

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
        let sessions = Arc::new(RwLock::new(HashMap::new()));

        'accept_loop: loop {
            select! {
                res = listener.accept() => {
                    if let Some(mut connection) = res {
                        let tasks = self.tasks.clone();
                        let token = token.clone();
                        let factory = self.factory.clone();
                        let instances = self.instances.clone();
                        let session_handler = session_handler.clone();
                        let sessions = sessions.clone();

                        self.tasks.spawn(async move {
                            let (downstream_sender, mut downstream_receiver) = mpsc::channel(100);
                            let connection_token = CancellationToken::new();
                            let mut local_sessions = HashSet::new();
        
                            'protocol_loop: loop {
                                select! {
                                    message = connection.recv() => {
                                        match message {
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

                                                local_sessions.insert(session_id);

                                                sessions.write().await.insert(session_id, ZoneSession::spawn(
                                                    tasks.clone(), 
                                                    connection_token.clone(), 
                                                    &session_id, 
                                                    &avatar_id, 
                                                    zone,
                                                    factory.clone(),
                                                    session_handler.clone(),
                                                    downstream_sender.clone()).await);
                                            },
                                            Some(ZoneUpstreamMessage::Travel { session_id, .. }) => {
                                                let sessions_s = sessions.read().await;

                                                if let Some(session) = sessions_s.get(&session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::Message { session_id, .. }) => {
                                                let sessions_s = sessions.read().await;

                                                if let Some(session) = sessions_s.get(&session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::LeaveZone { session_id }) => {
                                                debug!("Session {} leaving zone", session_id);

                                                let mut sessions_s = sessions.write().await;

                                                if let Some(session) = sessions_s.remove(&session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }

                                                local_sessions.remove(&session_id);
                                            },
                                            Some(ZoneUpstreamMessage::IngameCommand { session_id, .. }) => {
                                                let sessions_s = sessions.read().await;

                                                if let Some(session) = sessions_s.get(&session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::ApiCommand(command)) => {
                                                debug!("Api command: {:?}", command);

                                                let sessions_s = sessions.read().await;

                                                debug!("Session id: {:?}", command.session_id());
                                                debug!("Session: {:#?}", sessions_s.keys());

                                                match command.session_id() {
                                                    Some(id) => {
                                                        if let Some(session) = sessions_s.get(&id) {
                                                            let _ = session.send(ZoneUpstreamMessage::SessionApiCommand { 
                                                                downstream: downstream_sender.clone(), 
                                                                command 
                                                            }).await;
                                                        } else {
                                                            let _ = connection.send(&ZoneDownstreamMessage::ApiResult(
                                                                ApiResult::Error("session not connected to zone server".to_string())
                                                            )).await;
                                                        }
                                                    },
                                                    None => unimplemented!(),
                                                }
                                            },
                                            Some(ZoneUpstreamMessage::SessionApiCommand { .. }) => unreachable!(),
                                            None => {
                                                connection_token.cancel();
                                                downstream_receiver.close();
                                                break 'protocol_loop;
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

                            let mut sessions_s = sessions.write().await;
                            for session in local_sessions {
                                sessions_s.remove(&session);
                            }
        
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
    
    avatar_event_sender: mpsc::UnboundedSender<AvatarEvent>,
    avatar_events: Option<mpsc::UnboundedReceiver<AvatarEvent>>,
    interest_list: HashSet<AvatarId>,
    interest_added_queue: VecDeque<AvatarId>,
    interest_removed_queue: VecDeque<AvatarId>,
    ignore_interest_updates: bool,

    server_actions: VecDeque<ServerAction>,

    dont_save_on_disconnect: bool,
}

impl ZoneSession {
    async fn spawn(tasks: TaskTracker, token: CancellationToken, session_id: &Uuid, avatar_id: &AvatarId, 
        instance: ZoneInstance, zone_factory: ZoneFactory, session_handler: Arc<RwLock<SessionHandler>>,
        downstream: Sender<ZoneDownstreamMessage>) -> Sender<ZoneUpstreamMessage> {

        // todo: handle errors here, as this might be a race when the session
        // invalidates during the zone enter stage.
        let session_ref = session_handler.write().await.initiate_trusted(*session_id, *session_id).await.unwrap();

        let (avatar_event_sender, avatar_event_receiver) = mpsc::unbounded_channel();

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
            dont_save_on_disconnect: false,
        };

        session.run(&tasks, token)
    }

    fn run(mut self, tasks: &TaskTracker, token: CancellationToken) -> Sender<ZoneUpstreamMessage> {
        let (request_sender, mut request_receiver) = mpsc::channel(100);

        tasks.spawn(async move {
            let session_ref = self.session_ref.clone();
            //let mut zone_events = self.instance.zone().subscribe().await;
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
                                ZoneUpstreamMessage::Message { message, .. } => {
                                    if let Err(e) = self.handle_message(Message::from_bytes(&message).unwrap().1).await {
                                        error!(
                                            session = self.session_id.to_string(), 
                                            avatar = self.avatar_id.to_string(); 
                                            "Error while handling message: {:#?}", e);
                                        break 'net_loop;
                                    }
                                }
                                ZoneUpstreamMessage::LeaveZone { .. } => break 'net_loop,
                                ZoneUpstreamMessage::IngameCommand { command, .. } => {
                                    self.handle_ingame_command(command).await;
                                },
                                ZoneUpstreamMessage::ApiCommand(_) => unimplemented!("plain api commands are not meant to be executed in this context!"),
                                ZoneUpstreamMessage::SessionApiCommand { downstream, command } => {
                                    debug!("Session api command: {:?}", command);

                                    if let Err(e) = self.execute_api_command(&command, downstream.clone()).await {
                                        let _ = downstream.send(
                                            ZoneDownstreamMessage::ApiResult(crate::frontends::ApiResult::Error(e.to_string()))
                                        ).await;
                                    }
                                },
                            }
                            
                        } else {
                            break 'net_loop;
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
                        character.bling = Some(player.bling());
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
        let (name, player, server_action) = self.instance.zone().spawn_player(
            match destination {
                TravelType::EntryPoint => PlayerSpawnMode::TravelDirect,
                TravelType::Portal { uuid } => PlayerSpawnMode::TravelPortal(uuid),
                _ => unimplemented!(),
            }, 
            self.avatar_id, 
            session_s.session().character_id.unwrap(), 
            self.avatar_event_sender.clone()).await?;

        info!(
            session = self.session_id.to_string(), 
            avatar = self.avatar_id.to_string(); 
            "Spawning player: {}", name);

        // Set loading state
        self.send(oaPktS2XConnectionState {
            field_1: ClientLoadState::Transition.into(),
            field_2: 0,
            ..Default::default()
        }.into_message()).await?;

        let mut data = Vec::new();
        {
            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
            player.write_to_client(&mut writer)?;
        }

        if matches!(server_action, ServerAction::LocalPortal(_, _)) {
            // perform maplocal spawn
            self.send(server_action.into_message()).await?;
        } else {
            // perform complete spawn
            self.server_actions.push_back(server_action);
    
            // Send resource notification so pathengine can initialize
            let _ = self.send(CPktResourceNotify {
                resource_type: CpktResourceNotifyResourceType::WorldDef,
                field_2: self.zone_factory.world_def().guid,
                field_3: "".to_owned(),
                ..Default::default()
            }.into_message()).await;
        }

        Ok(())
    }

    async fn handle_avatar_event(&mut self, event: AvatarEvent) -> AnotherlandResult<()> {
        match event {
            AvatarEvent::InterestAdded(ids) => {
                if !self.ignore_interest_updates {
                    self.interest_removed_queue = self.interest_removed_queue.drain(..).filter(|v| !ids.contains(v)).collect();
                    self.interest_added_queue.append(&mut ids.into_iter().collect());
                }
            },
            AvatarEvent::InterestRemoved(ids) => {
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

                let (name, player, server_action) = self.instance.zone().spawn_player(PlayerSpawnMode::LoginNormal, self.avatar_id, session_s.session().character_id.unwrap(), self.avatar_event_sender.clone()).await?;
                self.server_actions.push_back(server_action);

                player.write_to_client(&mut writer)?;

                let movement = self.instance.zone().get_avatar_move_state(self.avatar_id).await.unwrap();

                info!(
                    session = self.session_id.to_string(), 
                    avatar = self.avatar_id.to_string(); 
                    "Spawning player: {}", name);

                self.send(CPktBlob {
                    avatar_id: self.avatar_id,
                    avatar_name: name,
                    class_id: PlayerAttribute::class_id().into(),
                    params: param_buffer.into(),
                    movement: MoveManagerInit {
                        pos: movement.position.into(),
                        rot: movement.rotation.into(),
                        vel: movement.velocity.into(),
                        physics: PhysicsState::Walking.into(),
                        mover_type: 1,
                        mover_replication_policy: 7,
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
                match pkt.field_4 {
                    Some(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
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
                        warn!("Unhandled routed packet: {:#?}", pkt.field_4);
                    },
                }
            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                if pkt.avatar_id.unwrap_or_default() == self.avatar_id {
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
                if pkt.avatar_id == self.avatar_id {
                    self.instance.zone().update_player_target(self.avatar_id, pkt.target_avatar_id.into()).await;
                }
            },
            AtlasPkt(CPkt::oaPktDialogList(pkt)) => {
                if pkt.instigator == self.avatar_id {
                    self.instance.zone().handle_message(pkt.into_message()).await;
                }
            },
            AtlasPkt(CPkt::oaPktDialogChoice(pkt)) => {
                if pkt.instigator == self.avatar_id {
                    self.instance.zone().handle_message(pkt.into_message()).await;
                }
            },
            AtlasPkt(CPkt::CPktRequestAvatarBehaviors(pkt)) => {
                debug!("Request behavior: {:#?}", pkt);

                self.instance.zone().request_behavior(pkt.avatar_id.into(), pkt.behaviour, pkt.data).await;
            },
            AtlasPkt(CPkt::oaPktAvatarTellBehavior(pkt)) => {
                if pkt.instigator != self.avatar_id {
                    warn!("Client tried to instigate behavior on behalf of other avatar: {:#?}", pkt);
                } else {
                    self.instance.zone().tell_behavior(pkt.instigator.into(), pkt.target.into(), pkt.behavior).await;
                }
            },
            AtlasPkt(CPkt::oaPktAvatarTellBehaviorBinary(pkt)) => {
                if pkt.instigator != self.avatar_id {
                    warn!("Client tried to instigate behavior on behalf of other avatar: {:#?}", pkt);
                } else {
                    self.instance.zone().tell_behavior_binary(pkt.instigator.into(), pkt.target.into(), pkt.behavior, pkt.data).await;
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
            AtlasPkt(CPkt::oaPktShopCartBuyRequest(pkt)) => {
                for cart_entry in pkt.shopping_cart {
                    self.instance.zone().item_purchase_request(self.avatar_id, cart_entry.id, cart_entry.count).await;
                }
            },
            AtlasPkt(CPkt::oaPktAccountBankRequest(_)) => {
                self.instance.zone().transfer_bling(self.avatar_id, 1000).await;
                self.instance.zone().transfer_game_cash(self.avatar_id, 1000).await;
            },
            AtlasPkt(CPkt::oaPktQuestRequest(pkt)) => {
                debug!("{:#?}", pkt);
                
                if pkt.player.is_none() && pkt.request == OaPktQuestRequestRequest::Request {
                    if let Some(quest) = lookup_quest_info(pkt.quest_id) {
                        self.send(CPktStream_165_2 {
                            field_1: oaQuestTemplate { 
                                quest_id: quest.id, 
                                level: quest.level, 
                                world_guid: quest.world, 
                                exp_reward: quest.exp_reward.unwrap_or_default(), 
                                bit_reward: quest.bit_reward.unwrap_or_default(), 
                                ..Default::default()
                            },
                            
                            conditions: quest.conditions.len() as u32,
                            field_3: quest.conditions.iter().map(|condition| {
                                oaQuestCondition { 
                                    quest_id: quest.id, 
                                    condition_id: condition.id, 
                                    required_count: condition.required_count, 
                                    greater_than_one: 2, 
                                    ..Default::default()
                                }
                            }).collect(),
                            ..Default::default()
                        }.into_message()).await?;
                    } else {
                        warn!("Quest {} not found!", pkt.quest_id);

                        self.send(CPktStream_165_2 {
                            field_1: oaQuestTemplate { 
                                quest_id: pkt.quest_id, 
                                level: 0, 
                                world_guid: UUID_NIL, 
                                ..Default::default()
                            },
                            ..Default::default()
                        }.into_message()).await?;
                    }
                } else if pkt.player == self.avatar_id {
                    self.instance.zone().handle_message(pkt.into_message()).await;
                }
            },
            AtlasPkt(CPkt::oaPktQuestDebugRequest(pkt)) => {
                self.instance.zone().handle_message(pkt.into_message()).await;
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

    async fn handle_ingame_command(&mut self, command: String) {
        self.instance.zone().exec_command(self.avatar_id, command).await;
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
                avatar_id,
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
                    avatar_id,
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

                        debug!("Send interest: {}", name);

                        self.send(CPktAvatarUpdate {
                            full_update: true,
                            avatar_id: Some(avatar_id),
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
            // Update loadstate
            {
                self.load_state = ClientLoadState::InGame;
                
                self.send(oaPktS2XConnectionState {
                    field_1: ClientLoadState::InGame.into(),
                    field_2: 0,
                    ..Default::default()
                }.into_message()).await?;
            }
            
            self.send(CPktServerNotify {
                notify_type: CpktServerNotifyNotifyType::SyncGameClock,
                game_clock: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()),
                ..Default::default()
            }.into_message()).await?;

            debug!("Send realm time");

            let otherland_epoch = Local.with_ymd_and_hms(2009, 1, 1, 0, 0, 0).unwrap();

            self.send(CPktServerNotify {
                notify_type: CpktServerNotifyNotifyType::SyncRealmTime,
                realm_time: Some(
                    Local::now().signed_duration_since(otherland_epoch).to_std().unwrap().as_millis() as i64
                ),
                ..Default::default()
            }.into_message()).await?;

            // notify that player is ready
            self.instance.zone().notify_player_ready(self.avatar_id).await;
        }

        if self.load_state == ClientLoadState::InGame {
            while let Some(action) = self.server_actions.pop_front() {
                self.send(action.into_message()).await?;
            }
        }

        Ok(())
    }

    async fn execute_api_command(&mut self, command: &ApiCommand, downstream: Sender<ZoneDownstreamMessage>) -> AnotherlandResult<()> {
        match command {
            ApiCommand::GetPlayerAvatarId { .. } => {
                debug!("Get player avatar id...");
                let _ = downstream.send(ZoneDownstreamMessage::ApiResult(
                    ApiResult::PlayerAvatar(self.avatar_id)
                )).await;
                Ok(())
            },
            ApiCommand::GetPlayerInterestList { .. } => {
                let _ = downstream.send(ZoneDownstreamMessage::ApiResult(
                    ApiResult::PlayerInterestList(self.interest_list.iter().cloned().collect())
                )).await;
                Ok(())
            },
            ApiCommand::GetAvatar { avatar_id, .. } => {
                if let Some(avatar) = self.instance.zone().get_avatar(*avatar_id).await {
                    let _ = downstream.send(ZoneDownstreamMessage::ApiResult(
                        ApiResult::Avatar(avatar)
                    )).await;
                    Ok(())
                } else {
                    Err(AnotherlandError::app_err("unknown avatar"))
                }
            },
            ApiCommand::GetSelectedAvatar { avatar_id, .. } => {
                let avatar_id = self.instance.zone().get_target_avatar(avatar_id.unwrap_or(self.avatar_id)).await;
                let _ = downstream.send(ZoneDownstreamMessage::ApiResult(
                    ApiResult::AvatarId(avatar_id)
                )).await;
                Ok(())
            }
            ApiCommand::UpdateAvatarParams { avatar_id, params, .. } => {
                self.instance.zone().update_avatar_named_params(*avatar_id, params.clone().into_iter().collect()).await;
                let _ = downstream.send(ZoneDownstreamMessage::ApiResult(ApiResult::Ok)).await;
                Ok(())
            }
        }
    }
}
