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

use bevy::{app::{App, Last, Plugin}, ecs::{component::Component, resource::Resource}, platform::collections::HashMap, prelude::{Commands, Entity, In, IntoSystem, Query, RemovedComponents, Res}};
use log::{debug, error, warn};
use obj_params::{GameObjectData, Player};
use protocol::{oaPktC2SConnectionState, oaPktClientServerPing, oaPktClientToClusterNode, oaPktClusterClientToCommunication, oaPktClusterClientToCommunity, oaPktClusterNodeToClient, CPkt, OaPktC2sconnectionStateState, OaPktS2xconnectionStateState, OtherlandPacket};
use tokio::sync::mpsc;
use toolkit::{types::Uuid, NativeParam};
use crate::{instance::InstanceLabel, plugins::{ControllerEntityEvent, ControllerRemoved, PlayerController}};

use crate::{error::WorldResult, instance::ZoneInstance, proto::TravelMode};

use super::ForeignResource;

type MessageHandler = Box<dyn Fn(&mut Commands, Entity, CPkt) + Send + Sync + 'static>;

#[derive(Resource)]
pub struct MessageHandlers(pub HashMap<(u8, u8), MessageHandler>);

type CommandMessageHandler = Box<dyn Fn(&mut Commands, Entity, NativeParam) -> WorldResult<()> + Send + Sync + 'static>;

pub trait CommandMessage: Send + Sync + Sized {
    fn id() -> i32;
    fn from_native_param(data: NativeParam) -> WorldResult<Self>;
}

#[derive(Resource)]
pub struct CommunityCommandMessageHandler(HashMap<i32, CommandMessageHandler>);

#[derive(Resource)]
#[allow(dead_code)]
pub struct CommunicationCommandMessageHandler(HashMap<i32, CommandMessageHandler>);

pub trait NetworkExtPriv {
    fn register_message_handler<P: OtherlandPacket + Send + Sync + 'static, T: IntoSystem<In<(Entity, P)>, (), Marker> + 'static, Marker>(&mut self, system: T);
    fn register_community_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T);
    #[allow(dead_code)]
    fn register_communication_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T);
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

        app.add_systems(Last, instance_shutdown);

        app.register_message_handler(handle_c2sconnection_state);
        app.register_message_handler(handle_client_server_ping);
        app.register_message_handler(handle_cluster_client_to_community);
        app.register_message_handler(handle_cluster_client_to_communication);
        app.register_message_handler(handle_cluster_client_to_cluster_node);
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

impl NetworkExtPriv for App {
    fn register_message_handler<P: OtherlandPacket + Send + Sync + 'static, T: IntoSystem<In<(Entity, P)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<MessageHandlers>()
            .unwrap()
            .0
            .insert(P::id(), Box::new(move |cmds: &mut Commands, ent: Entity, pkt: CPkt| {
                let pkt = pkt.into();
                cmds.run_system_with(system, (ent, pkt));
            }));
    }

    fn register_community_command_handler<C: CommandMessage + 'static, T: IntoSystem<In<(Entity, C)>, (), Marker> + 'static, Marker>(&mut self, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<CommunityCommandMessageHandler>()
            .unwrap()
            .0
            .insert(C::id(), Box::new(move |cmds: &mut Commands, ent: Entity, data: NativeParam| {
                let message = C::from_native_param(data)?;
                cmds.run_system_with(system, (ent, message));

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
                cmds.run_system_with(system, (ent, message));

                Ok(())
            }));
    }
}

pub enum WorldEvent {
    Packet { controller: Uuid, pkt: CPkt },
    TravelRequest { controller: Uuid, zone: Uuid, instance: Option<Uuid>, mode: TravelMode },
    TravelCommited { controller: Uuid },
    Close { controller: Uuid },
}

#[allow(dead_code)]
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

#[derive(Component, Default)]
pub struct CurrentState {
    pub state: ConnectionState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
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
                error!("Failed to parse client to community command: {e:?}");
            }
        } else {
            error!("Unknown client to community command: {id:#02x}");
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
                error!("Failed to parse client to communication command: {e:?}");
            }
        } else {
            error!("Unknown client to communication command: {id:#02x}");
        }
    }
}

pub fn handle_cluster_client_to_cluster_node(
    In((ent, pkt)): In<(Entity, oaPktClientToClusterNode)>,
    mut query: Query<(&PlayerController, &mut GameObjectData)>,
) {
    if let Ok((controller, mut data)) = query.get_mut(ent) {
        match pkt.field_2 {
            0x4 => { // Update quest tracker
                debug!("Quest tracker update: {pkt:#?}");

                let mut quests = data.get::<_, Vec<i32>>(Player::MyQuestTrack).cloned().unwrap_or_default();
                
                let NativeParam::Struct(args) = pkt.field_3 else {
                    warn!("Invalid quest tracker update packet");
                    controller.close();
                    return;
                };

                let Some(NativeParam::Bool(active)) = args.first() else {
                    warn!("Invalid quest tracker update packet");
                    controller.close();
                    return;
                };

                let Some(NativeParam::Int(quest_id)) = args.get(1) else {
                    warn!("Invalid quest tracker update packet");
                    controller.close();
                    return;
                };

                if *active {
                    if !quests.contains(quest_id) {
                        quests.push(*quest_id);
                    }
                } else {
                    quests.retain(|id| id != quest_id);
                }

                data.set(Player::MyQuestTrack, quests);
            },
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
                warn!("Unknown cluster node packet: {pkt:#?}");
            }
        }
    }
}