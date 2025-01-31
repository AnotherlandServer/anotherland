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

use std::{net::Shutdown, sync::Arc};

use bevy::{app::{App, First, Last, Plugin, SubApp}, ecs::system::SystemId, prelude::{in_state, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, In, IntoSystem, IntoSystemConfigs, Mut, NonSendMut, Query, RemovedComponents, Res, ResMut, Resource, With, World}, utils::HashMap};
use core_api::Session;
use log::{debug, error, warn};
use obj_params::{GameObjectData, Player};
use protocol::{oaPktC2SConnectionState, oaPktClientServerPing, oaPktClientToClusterNode, oaPktClusterClientToCommunication, oaPktClusterClientToCommunity, oaPktClusterNodeToClient, oaPktS2XConnectionState, oaPktServerAction, CPkt, CPktGameMsg, CPktResourceNotify, CpktGameMsgMsgType, CpktResourceNotifyResourceType, OaPktC2sconnectionStateState, OaPktS2xconnectionStateState, OtherlandPacket};
use realm_api::{proto::Destination, RealmApi, SessionState};
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};
use toolkit::{types::{AvatarId, Uuid}, NativeParam};
use crate::{instance::{InstanceLabel, InstanceShutdown}, proto::TravelRejectReason};

use crate::{error::WorldResult, instance::{InstanceState, ZoneInstance}, proto::TravelMode};

use super::{ForeignResource, ServerAction, Travelling};

type MessageHandler = Box<dyn Fn(&mut Commands, Entity, CPkt) + Send + Sync + 'static>;

#[derive(Resource)]
pub struct MessageHandlers(HashMap<(u8, u8), MessageHandler>);

type CommandMessageHandler = Box<dyn Fn(&mut Commands, Entity, NativeParam) -> WorldResult<()> + Send + Sync + 'static>;

pub trait CommandMessage: Send + Sync + Sized {
    fn id() -> i32;
    fn from_native_param(data: NativeParam) -> WorldResult<Self>;
}

#[derive(Resource)]
pub struct CommunityCommandMessageHandler(HashMap<i32, CommandMessageHandler>);

#[derive(Resource)]
pub struct CommunicationCommandMessageHandler(HashMap<i32, CommandMessageHandler>);

pub trait NetworkExtPriv {
    fn register_message_handler<P: OtherlandPacket + Send + Sync + 'static, T: IntoSystem<In<(Entity, P)>, (), Marker> + 'static, Marker>(&mut self, system: T);
    fn register_community_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T);
    fn register_communication_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T);
}

pub trait NetworkExt {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, travel_mode: TravelMode, sender: mpsc::UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>>;
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (ctrl_removed_sender, ctrl_removed_receiver) = mpsc::channel::<ControllerRemoved>(10);
        let (ctrl_event_sender, ctrl_event_receiver) = mpsc::channel::<ControllerEntityEvent>(100);

        app.insert_resource(MessageHandlers(HashMap::new()));
        app.insert_resource(CommunityCommandMessageHandler(HashMap::new()));
        app.insert_resource(CommunicationCommandMessageHandler(HashMap::new()));
        app.insert_resource(CommunityCommandMessageHandler(HashMap::new()));
        app.insert_resource(ForeignResource(ctrl_removed_sender));
        app.insert_resource(ForeignResource(ctrl_removed_receiver));
        app.insert_resource(ForeignResource(ctrl_event_sender));
        app.insert_resource(ForeignResource(ctrl_event_receiver));

        app.add_systems(First, handle_controller_events.run_if(in_state(InstanceState::Running)));
        app.add_systems(Last, (
            cleanup_player_controllers,
            instance_shutdown
        ).chain());
        app.add_systems(InstanceShutdown, close_connections);

        app.register_message_handler(handle_c2sconnection_state);
        app.register_message_handler(handle_client_server_ping);
        app.register_message_handler(handle_cluster_client_to_community);
        app.register_message_handler(handle_cluster_client_to_communication);
        app.register_message_handler(handle_cluster_client_to_cluster_node);
    }
}

fn close_connections(
    controller: Query<&PlayerController>,
) {
    for controller in controller.iter() {
        controller.close();
    }
}

fn cleanup_player_controllers(
    mut removed: ResMut<ForeignResource<Receiver<ControllerRemoved>>>,
    travelling: Query<(Entity, &PlayerController), With<Travelling>>,
    mut commands: Commands,
) {
    while let Ok(ControllerRemoved(ent)) = removed.try_recv() {
        if let Some(ent) = commands.get_entity(ent) {
            ent.despawn_recursive();
        }
    }

    for (ent, controller) in travelling.iter() {
        debug!("Committing travel of peer: {}", controller.id);

        let _ = controller.sender.send(WorldEvent::TravelCommited { controller: controller.id });
        commands.entity(ent).despawn_recursive();
    }
}

fn instance_shutdown(
    mut removed: RemovedComponents<PlayerController>,
    players: Query<&PlayerController>,
    instance: Res<ZoneInstance>,
) {
    if !removed.is_empty() {
        removed.clear();

        if instance.config.force_generate_guid_key && players.is_empty() {
            debug!("Last player left zone...");

            let manager = instance.manager.clone();
            let label = InstanceLabel::new(*instance.zone.guid(), instance.instance_id);
            instance.spawn_task(async move {
                manager.request_unregister_instance(label).await;
            });
        }
    }
}

fn handle_controller_events(
    mut packets: ResMut<ForeignResource<Receiver<ControllerEntityEvent>>>,
    message_handlers: Res<MessageHandlers>,
    mut commands: Commands,
) {
    while let Ok(ControllerEntityEvent(ent, ev)) = packets.try_recv() {
        match ev {
            ControllerEvent::Packet(pkt) => {
                if let Some(handler) = message_handlers.0.get(&pkt.get_id()) {
                    handler(&mut commands, ent, pkt)
                    //commands.run_system_with_input(*handler, (ent, pkt));
                } else {
                    warn!("Unknown pkt: {:#02x}:{:#02x}", pkt.get_id().0, pkt.get_id().1);
                }
            },
            ControllerEvent::TravelAccepted => {
                commands.entity(ent).insert(Travelling);
            },
            ControllerEvent::TravelRejected(travel_reject_reason) => todo!(),
        }
    }
}

impl NetworkExtPriv for App {
    fn register_message_handler<P: OtherlandPacket + Send + Sync + 'static, T: IntoSystem<In<(Entity, P)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<MessageHandlers>()
            .unwrap()
            .0
            .insert(P::id(), Box::new(move |cmds: &mut Commands, ent: Entity, pkt: CPkt| {
                let pkt = pkt.into();
                cmds.run_system_with_input(system, (ent, pkt));
            }));
    }

    fn register_community_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<CommunityCommandMessageHandler>()
            .unwrap()
            .0
            .insert(C::id(), Box::new(move |cmds: &mut Commands, ent: Entity, data: NativeParam| {
                let message = C::from_native_param(data)?;
                cmds.run_system_with_input(system, (ent, message));

                Ok(())
            }));
    }

    fn register_communication_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<CommunicationCommandMessageHandler>()
            .unwrap()
            .0
            .insert(C::id(), Box::new(move |cmds: &mut Commands, ent: Entity, data: NativeParam| {
                let message = C::from_native_param(data)?;
                cmds.run_system_with_input(system, (ent, message));

                Ok(())
            }));
    }
}

impl NetworkExt for SubApp {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, travel_mode: TravelMode, sender: mpsc::UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>> {
        let instance = self.world().get_resource::<ZoneInstance>().unwrap();
        let ctrl_removed = self.world().get_resource::<ForeignResource<Sender<ControllerRemoved>>>().unwrap().0.clone();
        let ctrl_event = self.world().get_resource::<ForeignResource<Sender<ControllerEntityEvent>>>().unwrap().0.clone();

        let session = instance.core_api.get_session(&session).await?
            .ok_or(anyhow::Error::msg("session not found"))?;

        let state = instance.realm_api.get_session_state(*session.id()).await?
            .ok_or(anyhow::Error::msg("no active session for realm found"))?;

        // Send resource notification to client, so it can begin loading the map.
        let _ = sender.send(WorldEvent::Packet { 
            controller: peer, 
            pkt: CPktResourceNotify {
                field_2: *instance.zone.worlddef_guid(),
                resource_type: CpktResourceNotifyResourceType::WorldDef,
                ..Default::default()
            }.into_pkt()
        });

        // Reset loading state
        let _ = sender.send(WorldEvent::Packet {
            controller: peer,
            pkt: oaPktS2XConnectionState {
                state: OaPktS2xconnectionStateState::Offline,
                ..Default::default()
            }.into_pkt()
        });

        let state = Arc::new(state);

        // Create entity
        let ent = self.world_mut().spawn((
            PlayerController {
                avatar_id: *state.avatar(),
                id: peer,
                session: Arc::new(session),
                state: state.clone(),
                sender,
                travel_mode,
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
                            let _ = ctrl_event.send(ControllerEntityEvent(ent, ControllerEvent::Packet(pkt))).await;
                        }
                    },
                    _ => { let _ = ctrl_event.send(ControllerEntityEvent(ent, evt)).await; }
                }
            }

            debug!("Stopping player controller {}", *state.avatar());
            let _ = ctrl_removed.send(ControllerRemoved(ent)).await;
        });

        Ok(ctrl_sender)
    }
}

pub struct ControllerRemoved(Entity);
pub struct ControllerEntityEvent(Entity, ControllerEvent);

pub enum ControllerEvent {
    Packet(CPkt),
    TravelAccepted,
    TravelRejected(TravelRejectReason),
}

pub enum WorldEvent {
    Packet { controller: Uuid, pkt: CPkt },
    TravelRequest { controller: Uuid, zone: Uuid, instance: Option<Uuid>, mode: TravelMode },
    TravelCommited { controller: Uuid },
    Close { controller: Uuid },
}

pub enum MessageType {
    Normal,
    Combat,
    Console,
    Clan,
    Party,
    Xp,
    Loot,
    Quest,
    PopUp,
    IllegalZone,
}

#[derive(Component, Clone)]
pub struct PlayerController {
    avatar_id: AvatarId,
    id: Uuid,
    session: Arc<Session>,
    state: Arc<SessionState>,
    sender: UnboundedSender<WorldEvent>,
    travel_mode: TravelMode,
}

impl PlayerController {
    pub fn avatar_id(&self) -> AvatarId { self.avatar_id }

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

    pub fn send_message(&self, ty: MessageType, message: String) {
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
                message,
                ..Default::default()
            }.into_pkt()
        });
    }
    
    fn close(&self) {
        let _ = self.sender.send(WorldEvent::Close { controller: self.id });
    }
}

#[derive(Component, Default)]
pub struct CurrentState {
    pub state: ConnectionState,
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
    In((ent, pkt)): In<(Entity, oaPktC2SConnectionState)>,
    mut query: Query<&mut CurrentState>,
) {
    if let Ok(mut state) = query.get_mut(ent) {
        let old_state = state.state;
        state.state = pkt.state.into();

        debug!("Connection state changed from {:?} to {:?}", old_state, state.state);
    }
}

pub fn handle_client_server_ping(
    In((ent, pkt)): In<(Entity, oaPktClientServerPing)>,
    query: Query<&PlayerController>,
) {
    if let Ok(controller) = query.get(ent) {
        controller.send_packet(pkt);
    }
}

pub fn handle_cluster_client_to_community(
    In((ent, pkt)): In<(Entity, oaPktClusterClientToCommunity)>,
    res: Res<CommunityCommandMessageHandler>,
    mut commands: Commands,
) {
    if 
        let NativeParam::Struct(params) = &pkt.field_3 &&
        let Some(NativeParam::Int(id)) = params.first()
    {
        if let Some(handler) = res.0.get(id) {
            if let Err(e) = handler(&mut commands, ent, pkt.field_3) {
                error!("Failed to parse client to community command: {:?}", e);
            }
        } else {
            error!("Unknown client to community command: {:#02x}", id);
        }
    }
}

pub fn handle_cluster_client_to_communication(
    In((ent, pkt)): In<(Entity, oaPktClusterClientToCommunication)>,
    res: Res<CommunityCommandMessageHandler>,
    mut commands: Commands,
) {
    if 
        let NativeParam::Struct(params) = &pkt.field_3 &&
        let Some(NativeParam::Int(id)) = params.first()
    {
        if let Some(handler) = res.0.get(id) {
            if let Err(e) = handler(&mut commands, ent, pkt.field_3) {
                error!("Failed to parse client to communication command: {:?}", e);
            }
        } else {
            error!("Unknown client to communication command: {:#02x}", id);
        }
    }
}

pub fn handle_cluster_client_to_cluster_node(
    In((ent, pkt)): In<(Entity, oaPktClientToClusterNode)>,
    mut query: Query<(&PlayerController, &mut GameObjectData)>,
) {
    if let Ok((controller, mut data)) = query.get_mut(ent) {
        match pkt.field_2 {
            0x5 => { // Check svr request
                controller.send_packet(oaPktClusterNodeToClient {
                    field_1: Uuid::new(),
                    field_3: NativeParam::Struct(vec![
                        NativeParam::Int(0xa8)
                    ]),
                    ..Default::default()
                });
            },
            0x6 => { // GHO_InitClientInfo
                data.set(Player::ClientReady, true);
                data.set(Player::PlayerLoading, false);
            }
            _ => {
                warn!("Unknown cluster node packet: {:#?}", pkt);
            }
        }
    }
}