// Copyright (C) 2025 AnotherlandServer
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

use std::sync::Arc;

use bevy::{app::SubApp, ecs::{component::Component, entity::Entity, message::{Message, MessageMutator}, query::With, system::{Commands, Query, Res, ResMut}}};
use core_api::Session;
use log::{debug, warn};
use protocol::{CPkt, CPktGameMsg, CPktResourceNotify, CpktGameMsgMsgType, CpktResourceNotifyResourceType, OaPktS2xconnectionStateState, OtherlandPacket, oaPktS2XConnectionState};
use realm_api::SessionState;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};
use toolkit::types::{AvatarId, Uuid};

use crate::{error::WorldResult, instance::ZoneInstance, plugins::{AvatarLoader, AvatarLoaderParams, AvatarStorageId, ComponentLoaderCommandsTrait, CurrentState, DespawnAvatar, DynamicInstance, ForeignResource, MessageHandlers, MessageType, SpawnState, Travelling, WorldEvent, player::loader::disconnect_player_error_handler}, proto::{TravelMode, TravelRejectReason}};

#[derive(Component, Clone)]
pub struct PlayerController {
    avatar_id: AvatarId,
    id: Uuid,
    character_id: Uuid,
    session: Arc<Session>,
    state: Arc<SessionState>,
    sender: UnboundedSender<WorldEvent>,
    travel_mode: TravelMode,
}

impl PlayerController {
    pub fn avatar_id(&self) -> AvatarId { self.avatar_id }
    pub fn character_id(&self) -> Uuid { self.character_id }

    pub fn session(&self) -> Arc<Session> { self.session.clone() }
    pub fn state(&self) -> Arc<SessionState> { self.state.clone() }

    pub fn travel_mode(&self) -> TravelMode {
        self.travel_mode
    }

    pub fn send_packet(&self, packet: impl OtherlandPacket) {
        let _ = self.sender.send(WorldEvent::Packet {
            controller: self.id,
            pkt: packet.into_pkt(),
        });
    }

    pub fn request_travel(&self, zone: Uuid, instance: Option<Uuid>, mode: TravelMode) {
        let _ = self.sender.send(WorldEvent::TravelRequest { 
            controller: self.id, 
            zone, 
            instance, 
            mode 
        });
    }

    pub fn send_message(&self, ty: MessageType, message: impl ToString) {
        let _ = self.sender.send(WorldEvent::Packet {
            controller: self.id,
            pkt: CPktGameMsg {
                msg_type: match ty {
                    MessageType::Normal => CpktGameMsgMsgType::Normal,
                    MessageType::Combat => CpktGameMsgMsgType::Combat,
                    MessageType::Console => CpktGameMsgMsgType::Console,
                    MessageType::Clan => CpktGameMsgMsgType::Clan,
                    MessageType::Party => CpktGameMsgMsgType::Party,
                    MessageType::Xp => CpktGameMsgMsgType::Xp,
                    MessageType::Loot => CpktGameMsgMsgType::Loot,
                    MessageType::Quest => CpktGameMsgMsgType::Quest,
                    MessageType::PopUp => CpktGameMsgMsgType::PopUp,
                    MessageType::IllegalZone => CpktGameMsgMsgType::IllegalZone,
                },
                message: message.to_string(),
                ..Default::default()
            }.into_pkt()
        });
    }
    
    pub fn close(&self) {
        let _ = self.sender.send(WorldEvent::Close { controller: self.id });
    }
}

pub enum ControllerEvent {
    Packet(CPkt),
    TravelAccepted,
    TravelRejected(TravelRejectReason),
}

pub struct ControllerRemoved(pub Entity);
pub struct ControllerEntityEvent(pub Entity, pub ControllerEvent);

#[derive(Message)]
pub struct PlayerJoinRequested {
    pub peer: Uuid,
    pub session: Arc<Session>,
    pub session_state: Arc<SessionState>,
    pub travel_mode: TravelMode,
    pub sender: UnboundedSender<WorldEvent>,
    pub ctrl_receiver: Option<Receiver<ControllerEvent>>,
}

pub trait PlayerControllerSubAppExt {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, travel_mode: TravelMode, sender: UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>>;
}


impl PlayerControllerSubAppExt for SubApp {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, travel_mode: TravelMode, sender: UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>> {
        let instance = self.world().get_resource::<ZoneInstance>().unwrap();

        let session = instance.core_api.get_session(&session).await?
            .ok_or(anyhow::Error::msg("session not found"))?;

        let state = instance.realm_api.get_session_state(*session.id()).await?
            .ok_or(anyhow::Error::msg("no active session for realm found"))?;

        let (ctrl_sender, ctrl_receiver) = mpsc::channel(10);

        self.world_mut().write_message(PlayerJoinRequested {
            peer,
            session: Arc::new(session),
            session_state: Arc::new(state),
            travel_mode,
            sender,
            ctrl_receiver: Some(ctrl_receiver),
        });

        Ok(ctrl_sender)
    }
}

pub fn process_join_requests(
    mut requests: MessageMutator<PlayerJoinRequested>,
    instance: Res<ZoneInstance>,
    ctrl_removed: ResMut<ForeignResource<Sender<ControllerRemoved>>>,
    ctrl_event: ResMut<ForeignResource<Sender<ControllerEntityEvent>>>,
    mut commands: Commands,
) {
    for PlayerJoinRequested { peer, session, session_state, travel_mode, sender, ctrl_receiver } in requests.read() {

        // Send resource notification to client, so it can begin loading the map.
        let _ = sender.send(WorldEvent::Packet { 
            controller: *peer, 
            pkt: CPktResourceNotify {
                field_2: *instance.zone.worlddef_guid(),
                resource_type: CpktResourceNotifyResourceType::WorldDef,
                ..Default::default()
            }.into_pkt()
        });

        // Reset loading state
        let _ = sender.send(WorldEvent::Packet {
            controller: *peer,
            pkt: oaPktS2XConnectionState {
                state: OaPktS2xconnectionStateState::Offline,
                ..Default::default()
            }.into_pkt()
        });

        

        // Create entity
        let ent = commands
            .spawn((
                PlayerController {
                    avatar_id: *session_state.avatar(),
                    id: *peer,
                    character_id: *session_state.character(),
                    session: session.clone(),
                    state: session_state.clone(),
                    sender: sender.clone(),
                    travel_mode: *travel_mode,
                },
                CurrentState::default(),
                SpawnState::Alive,
                DynamicInstance,
            ))
            .load_component_with_error_handler::<AvatarLoader>(
                AvatarLoaderParams {
                    id: AvatarStorageId::PlayerCharacter(*session_state.character()),
                    realm_api: instance.realm_api.clone(),
                },
                disconnect_player_error_handler
            )
            .id();

        let mut ctrl_receiver = ctrl_receiver.take().unwrap();
        let ctrl_event = ctrl_event.clone();
        let ctrl_removed = ctrl_removed.clone();
        let session_state = session_state.clone();

        // Start receive loop for the client, to feed messages
        // into the world event loop.
        tokio::spawn(async move {
            while let Some(evt) = ctrl_receiver.recv().await {
                match evt {
                    ControllerEvent::Packet(CPkt::CPktRouted(pkt)) => {
                        // I think CPktRouted packets are sent from the client
                        // to tell the server, that this has to be forwarded to a 
                        // different server behind the cluster server.
                        // I don't know yet what the desitnations mean, so we just unpack
                        // and forward it to the world server to be handled there. 
                        // In the future we might forward those packets directly to a responsible node.
                        if let Some(pkt) = pkt.field_4 {
                            let _ = ctrl_event.send(ControllerEntityEvent(ent, ControllerEvent::Packet(pkt))).await;
                        }
                    },
                    _ => { let _ = ctrl_event.send(ControllerEntityEvent(ent, evt)).await; }
                }
            }

            debug!("Stopping player controller {}", *session_state.avatar());
            let _ = ctrl_removed.send(ControllerRemoved(ent)).await;
        });
    }
}

pub fn close_connections(
    controller: Query<&PlayerController>,
) {
    for controller in controller.iter() {
        controller.close();
    }
}

pub fn cleanup_player_controllers(
    mut removed: ResMut<ForeignResource<Receiver<ControllerRemoved>>>,
    travelling: Query<(Entity, &PlayerController), With<Travelling>>,
    mut commands: Commands,
) {
    while let Ok(ControllerRemoved(ent)) = removed.try_recv() {
        debug!("Controller disconnected. Despawn player character...");
        commands.write_message(DespawnAvatar(ent));
    }

    for (ent, controller) in travelling.iter() {
        debug!("Committing travel of peer: {}", controller.id);

        let _ = controller.sender.send(WorldEvent::TravelCommited { controller: controller.id });
        commands.write_message(DespawnAvatar(ent));
    }
}

pub fn handle_controller_events(
    mut packets: ResMut<ForeignResource<Receiver<ControllerEntityEvent>>>,
    message_handlers: Res<MessageHandlers>,
    mut commands: Commands,
) {
    while let Ok(ControllerEntityEvent(ent, ev)) = packets.try_recv() {
        match ev {
            ControllerEvent::Packet(pkt) => {
                if let Some(handler) = message_handlers.0.get(&pkt.get_id()) {
                    handler(&mut commands, ent, pkt)
                    //commands.run_system_with(*handler, (ent, pkt));
                } else {
                    warn!("Unknown pkt: {:#02x}:{:#02x}", pkt.get_id().0, pkt.get_id().1);
                }
            },
            ControllerEvent::TravelAccepted => {
                commands.entity(ent).insert(Travelling);
            },
            ControllerEvent::TravelRejected(_travel_reject_reason) => todo!(),
        }
    }
}