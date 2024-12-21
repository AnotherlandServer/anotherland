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

use std::sync::Arc;

use bevy::{app::{App, First, Last, Plugin, SubApp}, ecs::system::SystemId, prelude::{in_state, Commands, Component, Entity, Event, EventReader, In, IntoSystem, IntoSystemConfigs, Mut, NonSendMut, Query, Res, ResMut, Resource, World}, utils::HashMap};
use core_api::Session;
use log::{debug, warn};
use protocol::{oaPktC2SConnectionState, oaPktClientServerPing, oaPktS2XConnectionState, CPkt, CPktResourceNotify, CpktResourceNotifyResourceType, OaPktC2sconnectionStateState, OaPktS2xconnectionStateState, OtherlandPacket};
use realm_api::{RealmApi, SessionState};
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};
use toolkit::types::{AvatarId, Uuid};

use crate::{error::WorldResult, instance::{InstanceState, ZoneInstance}, plugins::EnabledInGame};

use super::{ForeignResource, ServerAction};

#[derive(Resource)]
pub struct MessageHandlers(HashMap<(u8, u8), SystemId<In<(Entity, CPkt)>, ()>>);

pub trait NetworkExtPriv {
    fn register_message_handler<P: OtherlandPacket, T: IntoSystem<In<(Entity, CPkt)>, (), Marker> + 'static, Marker>(&mut self, system: T);
}

pub trait NetworkExt {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, sender: mpsc::UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>>;
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (ctrl_removed_sender, ctrl_removed_receiver) = mpsc::channel::<ControllerRemoved>(10);
        let (ctrl_packet_sender, ctrl_packet_receiver) = mpsc::channel::<ControllerPacket>(100);

        app.insert_resource(MessageHandlers(HashMap::new()));
        app.insert_resource(ForeignResource(ctrl_removed_sender));
        app.insert_resource(ForeignResource(ctrl_removed_receiver));
        app.insert_resource(ForeignResource(ctrl_packet_sender));
        app.insert_resource(ForeignResource(ctrl_packet_receiver));

        app.add_systems(First, handle_packets.run_if(in_state(InstanceState::Running)));
        app.add_systems(Last, cleanup_player_controllers);

        app.register_message_handler::<oaPktC2SConnectionState, _, _>(handle_c2sconnection_state);
        app.register_message_handler::<oaPktClientServerPing, _, _>(handle_client_server_ping);
    }
}

fn cleanup_player_controllers(
    mut removed: ResMut<ForeignResource<Receiver<ControllerRemoved>>>,
    mut commands: Commands,
) {
    while let Ok(ControllerRemoved(ent)) = removed.try_recv() {
        commands.entity(ent).despawn();
    }
}

fn handle_packets(
    mut packets: ResMut<ForeignResource<Receiver<ControllerPacket>>>,
    message_handlers: Res<MessageHandlers>,
    mut commands: Commands,
) {
    while let Ok(ControllerPacket(ent, pkt)) = packets.try_recv() {
        if let Some(handler) = message_handlers.0.get(&pkt.get_id()) {
            commands.run_system_with_input(*handler, (ent, pkt));
        } else {
            warn!("Unknown pkt: {:#02x}:{:#02x}", pkt.get_id().0, pkt.get_id().1);
        }
    }
}

impl NetworkExtPriv for App {
    fn register_message_handler<P: OtherlandPacket, T: IntoSystem<In<(Entity, CPkt)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<MessageHandlers>()
            .unwrap()
            .0
            .insert(P::id(), system);
    }
}

impl NetworkExt for SubApp {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, sender: mpsc::UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>> {
        let instance = self.world().get_resource::<ZoneInstance>().unwrap();
        let ctrl_removed = self.world().get_resource::<ForeignResource<Sender<ControllerRemoved>>>().unwrap().0.clone();
        let ctrl_packet = self.world().get_resource::<ForeignResource<Sender<ControllerPacket>>>().unwrap().0.clone();

        let session = instance.core_api.get_session(&session).await?
            .ok_or(anyhow::Error::msg("session not found"))?;

        let state = instance.realm_api.get_session_state(*session.id()).await?
            .ok_or(anyhow::Error::msg("no active session for realm found"))?;

        // Send resource notification to client, so it can begin loading the map.
        let _ = sender.send(WorldEvent::Packet { 
            peer, 
            pkt: CPktResourceNotify {
                field_2: *instance.zone.worlddef_guid(),
                resource_type: CpktResourceNotifyResourceType::WorldDef,
                ..Default::default()
            }.into_pkt()
        });

        // Reset loading state
        let _ = sender.send(WorldEvent::Packet {
            peer,
            pkt: oaPktS2XConnectionState {
                state: OaPktS2xconnectionStateState::Offline,
                ..Default::default()
            }.into_pkt()
        });

        // Create entity
        let ent = self.world_mut().spawn((
            PlayerController {
                avatar_id: *state.avatar(),
                peer,
                session: Arc::new(session),
                state: Arc::new(state.clone()),
                sender,
                spawn_action: None, // TODO: Use this to override spawn action as result of travel or portals
            }, 
            CurrentState::default()
        )).id();

        let (ctrl_sender, mut ctrl_receiver) = mpsc::channel(10);

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
                            let _ = ctrl_packet.send(ControllerPacket(ent, pkt)).await;
                        }
                    }
                    ControllerEvent::Packet(pkt) => {
                        let _ = ctrl_packet.send(ControllerPacket(ent, pkt)).await;
                    },
                }
            }

            debug!("Stopping player controller {}", *state.avatar());
            let _ = ctrl_removed.send(ControllerRemoved(ent)).await;
        });

        Ok(ctrl_sender)
    }
}

pub struct ControllerRemoved(Entity);
pub struct ControllerPacket(Entity, CPkt);

pub enum ControllerEvent {
    Packet(CPkt)
}

pub enum WorldEvent {
    Packet { peer: Uuid, pkt: CPkt }
}

#[derive(Component, Clone)]
pub struct PlayerController {
    avatar_id: AvatarId,
    peer: Uuid,
    session: Arc<Session>,
    state: Arc<SessionState>,
    sender: UnboundedSender<WorldEvent>,
    spawn_action: Option<ServerAction>,
}

impl PlayerController {
    pub fn avatar_id(&self) -> AvatarId { self.avatar_id }

    pub fn session(&self) -> &Session { &self.session }
    pub fn state(&self) -> &SessionState { &self.state }

    pub fn take_spawn_action(&mut self) -> Option<ServerAction> {
        self.spawn_action.take()
    }

    pub fn send_packet(&self, packet: impl OtherlandPacket) {
        let _ = self.sender.send(WorldEvent::Packet {
            peer: self.peer,
            pkt: packet.into_pkt(),
        });
    }
}

#[derive(Component, Default)]
pub struct CurrentState {
    pub state: ConnectionState,
    pub version: u64,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ConnectionState {
    #[default]
    Offline,
    Transition,
    PlayerReceived,
    MapLoaded,
    PlayerLoaded,
    WaitingForInitialInterests,
    ReceivedInitialInterests,
    InitialInterestsLoaded,
    InGame,
}

impl From<OaPktC2sconnectionStateState> for ConnectionState {
    fn from(value: OaPktC2sconnectionStateState) -> Self {
        match value {
            OaPktC2sconnectionStateState::Offline => Self::Offline,
            OaPktC2sconnectionStateState::Transition => Self::Transition,
            OaPktC2sconnectionStateState::PlayerReceived => Self::PlayerReceived,
            OaPktC2sconnectionStateState::MapLoaded => Self::MapLoaded,
            OaPktC2sconnectionStateState::PlayerLoaded => Self::PlayerLoaded,
            OaPktC2sconnectionStateState::WaitingForInitialInterests => Self::WaitingForInitialInterests,
            OaPktC2sconnectionStateState::ReceivedInitialInterests => Self::ReceivedInitialInterests,
            OaPktC2sconnectionStateState::InitialInterestsLoaded => Self::InitialInterestsLoaded,
            OaPktC2sconnectionStateState::InGame => Self::InGame,    
        }
    }
}

impl From<ConnectionState> for OaPktS2xconnectionStateState {
    fn from(value: ConnectionState) -> Self {
        match value {
            ConnectionState::Offline => OaPktS2xconnectionStateState::Offline,
            ConnectionState::Transition => OaPktS2xconnectionStateState::Transition,
            ConnectionState::PlayerReceived => OaPktS2xconnectionStateState::PlayerReceived,
            ConnectionState::MapLoaded => OaPktS2xconnectionStateState::MapLoaded,
            ConnectionState::PlayerLoaded => OaPktS2xconnectionStateState::PlayerLoaded,
            ConnectionState::WaitingForInitialInterests => OaPktS2xconnectionStateState::WaitingForInitialInterests,
            ConnectionState::ReceivedInitialInterests => OaPktS2xconnectionStateState::ReceivedInitialInterests,
            ConnectionState::InitialInterestsLoaded => OaPktS2xconnectionStateState::InitialInterestsLoaded,
            ConnectionState::InGame => OaPktS2xconnectionStateState::InGame,
        }
    }
}

pub fn handle_c2sconnection_state(
    In((ent, pkt)): In<(Entity, CPkt)>,
    mut query: Query<(&mut CurrentState, &PlayerController)>,
) {
    if 
        let Ok((mut state, controller)) = query.get_mut(ent) &&
        let CPkt::oaPktC2SConnectionState(pkt) = pkt
    {
        let old_state = state.state;
        state.state = pkt.state.into();

        debug!("Connection state changed from {:?} to {:?}", old_state, state.state);
    }
}

pub fn handle_client_server_ping(
    In((ent, pkt)): In<(Entity, CPkt)>,
    query: Query<&PlayerController>,
) {
    if 
        let Ok(controller) = query.get(ent) &&
        let CPkt::oaPktClientServerPing(pkt) = pkt
    {
        controller.send_packet(*pkt);
    }
}