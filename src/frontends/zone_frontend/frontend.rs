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

use std::{collections::{HashSet, HashMap, VecDeque}, sync::Arc, net::{SocketAddr, IpAddr, Ipv6Addr}, time::{Duration, SystemTime, UNIX_EPOCH}, thread};

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, Message}, AvatarId, CPkt, CPktResourceNotify, CpktResourceNotifyResourceType, CPktBlob, PlayerParam, BoundParamClass, Player, NetworkVec3, CPktAvatarClientNotify, CPktStackedAvatarUpdate, NativeParam, ParamClassContainer, CPktAvatarUpdate, NonClientBase, ParamClass, oaPktS2XConnectionState, CPktServerNotify, oaPktServerAction, oaPktMoveManagerPosUpdate, CpktServerNotifyNotifyType, Uuid, MoveManagerInit, MoveManagerInitPhysicsState};
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use glam::Vec3;
use log::{debug, error, trace, warn, info};
use mongodb::change_stream::session;
use quinn::{ServerConfig, Endpoint};
use tokio::{sync::{Mutex, OnceCell, mpsc::{self, Sender}}, net::{TcpListener, TcpStream, UdpSocket}, time::{Interval, self}, select, task};
use tokio_util::{task::TaskTracker, sync::CancellationToken, udp::UdpFramed};

use crate::{cluster::{frontend::Frontend, ActorRef}, util::{AnotherlandResult, AnotherlandErrorKind}, CONF, db::{ZoneDef, realm_database, ItemContent, Character, WorldDef}, components::{Zone, ZoneRegistry, SessionHandler, SessionRef, ZoneEvent, InterestEvent, Movement}, NODE, CLUSTER_CERT};
use crate::db::DatabaseRecord;

use super::{ZoneServerListener, ZoneMessage, load_state::ClientLoadState};

pub struct ZoneFrontend {
    name: String,
    zone_def: Arc<ZoneDef>,
    zone: ActorRef<Zone>,
    //movement_manager: ActorRef<MovementManager>,
    tasks: TaskTracker,
}

impl ZoneFrontend {
    pub async fn initialize(world_def: WorldDef, zone_def: ZoneDef) -> AnotherlandResult<Self> {
        let zone = NODE.add_actor(Zone::initialize(world_def, zone_def.clone()).await?);
        //let movement_manager = NODE.add_actor(MovementManager::initialize(format!("mm_{}", zone_def.guid).as_str(), zone.clone()).await?);

        Ok(Self {
            name: format!("zone_server_{}", zone_def.guid),
            zone_def: Arc::new(zone_def),
            zone,
            //movement_manager,
            tasks: TaskTracker::new(),
        })
    }
}

#[async_trait]
impl Frontend for ZoneFrontend {
    fn name(&self) -> &str { self.name.as_str() }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        //let _ = self.zone.set(NODE.add_actor(Zone::initialize(self.zone_def.as_ref().clone()).await?));

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

        let mut zone_registry = NODE.get_remote_actor::<ZoneRegistry>("zone_registry").ok_or(AnotherlandErrorKind::ApplicationError)?;
        let mut registration_update_interval = time::interval(Duration::from_secs(1));

        let session_handler = NODE.add_actor(SessionHandler::<()>::initialize(format!("zone_{}_session_handler", self.zone_def.guid).as_str()).await);

        'accept_loop: loop {
            select! {
                res = listener.accept() => {
                    if let Some(mut connection) = res {
                        let tasks = self.tasks.clone();
                        let token = token.clone();
                        let zone = self.zone.clone();
                        let zone_def = self.zone_def.clone();
                        let session_handler = session_handler.clone();
        
                        let mut sessions = HashMap::new();
        
                        self.tasks.spawn(async move {
                            let (downstream_sender, mut downstream_receiver) = mpsc::channel(10);
                            let connection_token = CancellationToken::new();
        
                            'accept_loop: loop {
                                select! {
                                    message = connection.recv() => {
                                        match message.as_ref() {
                                            Some(ZoneMessage::EnterZone { session_id, avatar_id }) => {
                                                debug!("Session {} entering zone with avatar_id {}", session_id, avatar_id);

                                                sessions.insert(session_id.clone(), ZoneSession::spawn(
                                                    tasks.clone(), 
                                                    connection_token.clone(), 
                                                    session_id, 
                                                    avatar_id, 
                                                    zone.clone(),
                                                    zone_def.clone(),
                                                    session_handler.clone(),
                                                    downstream_sender.clone()).await);
                                            },
                                            Some(ZoneMessage::Travel { session_id }) => {
                                                if let Some(session) = sessions.get(session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneMessage::Message { session_id, .. }) => {
                                                if let Some(session) = sessions.get(session_id) {
                                                    let _ = session.send(message.unwrap()).await;
                                                }
                                            },
                                            Some(ZoneMessage::LeaveZone { session_id }) => {
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
                    zone_registry.register_zone_frontend(self.zone_def.guid.into(), listener.local_addr().unwrap()).await;
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
    zone_def: Arc<ZoneDef>,
    session_handler: ActorRef<SessionHandler<()>>,
    session_ref: SessionRef<()>,
    downstream: Sender<ZoneMessage>,
    load_state: ClientLoadState,
    zone: ActorRef<Zone>,

    interest_event_sender: mpsc::Sender<InterestEvent>,
    interest_events: Option<mpsc::Receiver<InterestEvent>>,
    interest_list: HashSet<AvatarId>,
    interest_added_queue: VecDeque<AvatarId>,
    interest_removed_queue: VecDeque<AvatarId>,
}

impl ZoneSession {
    async fn spawn(tasks: TaskTracker, token: CancellationToken, session_id: &Uuid, avatar_id: &AvatarId, 
        zone: ActorRef<Zone>, zone_def: Arc<ZoneDef>, mut session_handler: ActorRef<SessionHandler<()>>,
        downstream: Sender<ZoneMessage>) -> Sender<ZoneMessage> {

        // todo: handle errors here, as this might be a race when the session
        // invalidates during the zone enter stage.
        let session_ref = session_handler.initiate_trusted(session_id.clone(), session_id.clone()).await.unwrap();

        let (interest_event_sender, interest_event_receiver) = mpsc::channel(10);

        let session = ZoneSession {
            session_id: session_id.to_owned(),
            avatar_id: avatar_id.to_owned(),
            zone_def,
            session_handler,
            session_ref,
            zone,
            downstream,
            load_state: ClientLoadState::EarlyLoadSequence,
            interest_event_sender: interest_event_sender,
            interest_events: Some(interest_event_receiver),
            interest_list: HashSet::new(),
            interest_added_queue: VecDeque::new(),
            interest_removed_queue: VecDeque::new(),
        };

        session.run(&tasks, token)
    }

    fn run(mut self, tasks: &TaskTracker, token: CancellationToken) -> Sender<ZoneMessage> {
        let (request_sender, mut request_receiver) = mpsc::channel(10);

        tasks.spawn(async move {
            let session_ref = self.session_ref.clone();
            let mut zone_events = self.zone.subscribe().await;

            let mut interest_event_receiver = self.interest_events.take().unwrap();

            let mut update_timer = time::interval(Duration::from_millis(250));

            'net_loop: loop {
                select! {
                    request = request_receiver.recv() => {
                        if let Some(request) = request {
                            match request {
                                ZoneMessage::EnterZone { .. } => unreachable!(),
                                ZoneMessage::Travel { .. } => {
                                    if let Err(e) = self.travel_to_zone().await {
                                        error!(
                                            session = self.session_id.to_string(), 
                                            avatar = self.avatar_id.to_string(); 
                                            "Error while travelling: {:#?}", e);
                                        break 'net_loop;
                                    }
                                },
                                ZoneMessage::Message { message, ..} => {
                                    if let Err(e) = self.handle_message(Message::from_bytes(&message).unwrap().1).await {
                                        error!(
                                            session = self.session_id.to_string(), 
                                            avatar = self.avatar_id.to_string(); 
                                            "Error while handling message: {:#?}", e);
                                        break 'net_loop;
                                    }
                                }
                                ZoneMessage::LeaveZone { .. } => break 'net_loop,
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
                    Some(event) = interest_event_receiver.recv() => {
                        if let Err(e) = self.handle_interest_event(event).await {
                            error!(
                                session = self.session_id.to_string(), 
                                avatar = self.avatar_id.to_string(); 
                                "Interest event error: {:#?}", e);
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

            // despawn avatar
            self.zone.despawn_avatar(self.avatar_id.clone()).await;
            
            trace!(
                session = self.session_id.to_string(), 
                avatar = self.avatar_id.to_string(); 
                "Stopping zone frontend session");
        });

        request_sender
    }

    async fn travel_to_zone(&mut self) -> AnotherlandResult<()> {
        let session_s = self.session_ref.lock().await;

        // Spawn player character
        let mut param_buffer = Vec::new();
        let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

        let player = self.zone.spawn_player(self.avatar_id.clone(), session_s.session().character_id.unwrap(), self.interest_event_sender.clone()).await?;
        let _ = player.data.write_to_client(&mut writer)?;
        
        info!(
            session = self.session_id.to_string(), 
            avatar = self.avatar_id.to_string(); 
            "Spawning player: {}", player.name);

        let movement = self.zone.get_avatar_move_state(self.avatar_id.clone()).await.unwrap();

        let mut mm_init: MoveManagerInit = MoveManagerInit::default();
        mm_init.pos = movement.position.into();
        mm_init.rot = movement.rotation.into();
        mm_init.vel = movement.velocity.into();

        mm_init.physics_state = MoveManagerInitPhysicsState::Standing;

        let mut data = Vec::new();
        {
            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
            player.data.write_to_client(&mut writer)?;
        }

        // Update player character on client
        let mut avatar_update = CPktAvatarUpdate::default();
        avatar_update.full_update = true;
        avatar_update.avatar_id = Some(self.avatar_id.as_u64());
        avatar_update.field_2 = Some(false);
        avatar_update.name = Some(player.name);
        avatar_update.class_id = Some(PlayerParam::CLASS_ID.as_u32());
        avatar_update.field_6 = Some(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap());
        avatar_update.params = data.into();
        
        avatar_update.update_source = 0;
        avatar_update.movement = Some(mm_init.to_bytes().into());

        self.send(avatar_update.into_message()).await?;

        Ok(())
    }

    async fn handle_interest_event(&mut self, event: InterestEvent) -> AnotherlandResult<()> {
        match event {
            InterestEvent::InterestAdded { ids } => {
                self.interest_removed_queue = self.interest_removed_queue.drain(..).filter(|v| !ids.contains(v)).collect();
                self.interest_added_queue.append(&mut ids.into_iter().collect());
            },
            InterestEvent::InterestRemoved { ids } => {
                self.interest_added_queue = self.interest_added_queue.drain(..).filter(|v| !ids.contains(v)).collect();
                self.interest_removed_queue.append(&mut ids.into_iter().collect());
            }
        }

        Ok(())
    }

    async fn handle_zone_event(&mut self, event: Arc<ZoneEvent>) -> AnotherlandResult<()> {
        match event.as_ref() {
            ZoneEvent::AvatarSpawned { avatar_id, params } => {
                if *avatar_id != self.avatar_id {
                    // todo: check if we need to add the character to our interest list
                }
            },
            ZoneEvent::AvatarUpdated { avatar_id, params } => {
                if self.interest_list.contains(avatar_id) || self.avatar_id == *avatar_id {
                    let mut param_buffer = Vec::new();
                    let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                    params.write_to_client(&mut writer)?;
    
                    let mut avatar_update = CPktAvatarUpdate::default();
                    avatar_update.full_update = false;
                    avatar_update.avatar_id = Some(avatar_id.as_u64());
                    avatar_update.update_source = 0;
                    avatar_update.params = param_buffer.into();

                    let _ = self.send(avatar_update.into_message()).await?;
                }
            },
            ZoneEvent::AvatarMoved { avatar_id, movement } => {
                if self.interest_list.contains(avatar_id) {
                    let mut pos_update = oaPktMoveManagerPosUpdate::default();
                    pos_update.pos = movement.position.into();
                    pos_update.rot = movement.rotation.into();
                    pos_update.vel = movement.velocity.into();
                    pos_update.physics_state = movement.physics_state.into();
                    pos_update.mover_key = movement.mover_key;
                    pos_update.avatar_id = avatar_id.as_u64();
                    pos_update.seconds = movement.seconds;

                    let _ = self.send(pos_update.into_message()).await?;
                }
            }
            ZoneEvent::AvatarDespawned { avatar_id } => {
                if self.interest_list.remove(avatar_id) {
                    // tell client the avatar despawned
                    let mut avatar_notify = CPktAvatarClientNotify::default();
                    avatar_notify.avatar_id = avatar_id.clone().as_u64();

                    self.send(avatar_notify.into_message()).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> AnotherlandResult<()> {
        use Message::*;

        match message {
            AtlasPkt(CPkt::oaPktRequestEnterGame(_pkt)) => {
                let session_s = self.session_ref.lock().await;

                // Send resource notification 
                let mut worlddef = CPktResourceNotify::default();
                worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                worlddef.field_2 = self.zone_def.worlddef_guid.clone().into();
                worlddef.field_3 = "".to_owned();

                let _ = self.send(worlddef.into_message()).await;

                // Spawn player character
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                let player = self.zone.spawn_player(self.avatar_id.clone(), session_s.session().character_id.unwrap(), self.interest_event_sender.clone()).await?;
                let _ = player.data.write_to_client(&mut writer)?;

                let movement = self.zone.get_avatar_move_state(self.avatar_id.clone()).await.unwrap();

                let mut mm_init: MoveManagerInit = MoveManagerInit::default();
                mm_init.pos = movement.position.into();
                mm_init.rot = movement.rotation.into();
                mm_init.vel = movement.velocity.into();

                mm_init.physics_state = MoveManagerInitPhysicsState::Standing;       

                info!(
                    session = self.session_id.to_string(), 
                    avatar = self.avatar_id.to_string(); 
                    "Spawning player: {}", player.name);

                // Transfer player character to client
                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_id = self.avatar_id.as_u64();
                avatar_blob.avatar_name = player.name;
                avatar_blob.class_id = PlayerParam::CLASS_ID.as_u32();
                avatar_blob.params = param_buffer.into();
                avatar_blob.movement = mm_init.to_bytes().into();
                avatar_blob.has_guid = true;
                avatar_blob.field_7 = Some(self.session_id.clone());

                self.send(avatar_blob.into_message()).await?;
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
                    5 => ClientLoadState::RequestAvatarStream,
                    6 => ClientLoadState::StreamedAvatars,
                    7 => ClientLoadState::RequestSpawn,
                    8 => ClientLoadState::Spawned,
                    _ => {
                        warn!(
                            session = self.session_id.to_string(), 
                            avatar = self.avatar_id.to_string(); 
                            "Invalid client loadstate: {}", pkt.field_1);
                        ClientLoadState::EarlyLoadSequence
                    }
                };

                // Confirm loading state
                let mut response = pkt.clone();
                response.field_1 = self.load_state.clone().into();
                response.field_2 = pkt.field_2 + 1;

                self.send(response.into_message()).await?;
            },
            AtlasPkt(CPkt::CPktRouted(pkt)) => {
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                        debug!("Movement: {:#?}", pkt);

                        self.zone.move_player_avatar(
                            self.avatar_id.clone(), 
                            Movement {
                                position: pkt.pos.into(),
                                rotation: pkt.rot.into(),
                                velocity: pkt.vel.into(),
                                physics_state: pkt.physics_state.into(),
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
                                let (_, mut params) = self.zone.get_avatar_params(self.avatar_id.clone()).await.unwrap();
                                let db: mongodb::Database = realm_database().await;

                                match &mut params {
                                    ParamClassContainer::Player(player_params) => {
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
                                        let item = ItemContent::get(db.clone(), &item_uuid.into()).await?;
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
                                    },
                                    _ => unreachable!(),
                                }
        
                                self.zone.update_avatar(self.avatar_id.clone(), params).await;
                            },
                            _ => panic!(),
                        }
                    },
                    _ => {
                        info!("Unknown avatar behavior: {:#?}", pkt);
                        todo!();
                    }
                }
            },
            AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                if pkt.avatar_id.unwrap_or_default() == self.avatar_id.as_u64() {
                    if let Ok((_, params)) = ParamClassContainer::read(PlayerParam::CLASS_ID.as_u16(), pkt.params.as_slice()) {
                        self.zone.update_avatar(self.avatar_id.clone(), params).await;
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
        self.downstream.send(ZoneMessage::Message { 
            session_id: self.session_id.clone(),
            message: message.to_bytes(), 
        }).await.map_err(|_| AnotherlandErrorKind::IOError)?;

        Ok(())
    }

    async fn update(&mut self) -> AnotherlandResult<()> {
        // Only start transmitting interests once the client left
        // the early load sequence.
        if self.load_state != ClientLoadState::EarlyLoadSequence {
            // remove avatars we are not interested in anymore
            while let Some(avatar_id) = self.interest_removed_queue.pop_front() {
                let mut avatar_notify = CPktAvatarClientNotify::default();
                avatar_notify.avatar_id = avatar_id.as_u64();
                self.send(avatar_notify.into_message()).await?;

                // remove from interest list
                self.interest_list.remove(&avatar_id);
            }

            // limit to push up to 10 avatars per tick
            for _ in 0..10 {
                if let Some(avatar_id) = self.interest_added_queue.pop_front() {
                    if let Some((name, params)) = self.zone.get_avatar_params(avatar_id.clone()).await {
                        let movement = self.zone.get_avatar_move_state(avatar_id.clone()).await.unwrap();

                        // add to interest list, so the client will receive updates
                        // for this avatar.
                        self.interest_list.insert(avatar_id.clone());

                        let mut mm_init: MoveManagerInit = MoveManagerInit::default();
                        mm_init.pos = movement.position.into();
                        mm_init.rot = movement.rotation.into();
                        mm_init.vel = movement.velocity.into();

                        mm_init.physics_state = MoveManagerInitPhysicsState::Standing;  

                        let mut data = Vec::new();
                        {
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            params.write_to_client(&mut writer)?;
                        }

                        let mut avatar_update = CPktAvatarUpdate::default();
                        avatar_update.full_update = true;
                        avatar_update.avatar_id = Some(avatar_id.as_u64());
                        avatar_update.field_2 = Some(false);
                        avatar_update.name = Some(name);
                        avatar_update.class_id = Some(params.class_id().as_u32());
                        avatar_update.field_6 = Some(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap());
                        //avatar_update.flags = Some(2);
                        //avatar_update.flag_2_uuid = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                        avatar_update.params = data.into();
                        
                        avatar_update.update_source = 0;
                        avatar_update.movement = Some(mm_init.to_bytes().into());

                        let _ = self.send(avatar_update.into_message()).await?;
                    }
                } else if self.load_state == ClientLoadState::RequestAvatarStream {
                    // Update client loading state if we are in the initial streaming phase
                    self.load_state = ClientLoadState::StreamedAvatars;

                    let mut connectionstate = oaPktS2XConnectionState::default();
                    connectionstate.field_1 = ClientLoadState::StreamedAvatars.into();
                    connectionstate.field_2 = 0;

                    self.send(connectionstate.into_message()).await?;
                }
            }
        }

        if self.load_state == ClientLoadState::RequestSpawn {
                // Synchronize time
                {
                let mut game_time_sync = CPktServerNotify::default();
                game_time_sync.notify_type = CpktServerNotifyNotifyType::SyncGameClock;
                game_time_sync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

                self.send(game_time_sync.into_message()).await?;
            }

            {
                let mut realm_time_sync = CPktServerNotify::default();
                realm_time_sync.notify_type = CpktServerNotifyNotifyType::SyncRealmTime;
                realm_time_sync.field_4 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                self.send(realm_time_sync.into_message()).await?;
            }

            // Update loadstate
            {
                self.load_state = ClientLoadState::Spawned;
                
                let mut connectionstate = oaPktS2XConnectionState::default();
                connectionstate.field_1 = ClientLoadState::Spawned.into();
                connectionstate.field_2 = 0;

                self.send(connectionstate.into_message()).await?;
            }

            // Tell the client the avatar is ready to spawn
            {
                let mut action = oaPktServerAction::default();
                action.action = "TRAVEL:NonPortalTravel|NonPortalTravelDefault".to_owned();
                action.version = 4;
                action.override_teleport = false;
                self.send(action.into_message()).await?;
            }
        }

        Ok(())
    }
}
