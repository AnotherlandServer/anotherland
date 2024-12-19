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

use bevy::{app::{First, Last, Plugin, SubApp}, ecs::system::SystemId, log::debug, prelude::{in_state, Commands, Component, Entity, Event, EventReader, In, IntoSystem, IntoSystemConfigs, Mut, NonSendMut, Res, ResMut, Resource, World}, utils::HashMap};
use log::warn;
use protocol::CPkt;
use realm_api::RealmApi;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};
use toolkit::types::{AvatarId, Uuid};

use crate::{error::WorldResult, instance::{InstanceState, ZoneInstance}};

use super::Shared;

#[derive(Resource)]
pub struct MessageHandlers(HashMap<(u8, u8), SystemId<In<(Entity, CPkt)>, ()>>);

pub trait NetworkExtPriv {
    fn register_message_handler<T: IntoSystem<In<(Entity, CPkt)>, (), Marker> + 'static, Marker>(&mut self, id: (u8, u8), system: T);
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
        app.insert_resource(Shared(ctrl_removed_sender));
        app.insert_resource(Shared(ctrl_removed_receiver));
        app.insert_resource(Shared(ctrl_packet_sender));
        app.insert_resource(Shared(ctrl_packet_receiver));

        app.add_systems(First, handle_packets.run_if(in_state(InstanceState::Running)));
        app.add_systems(Last, cleanup_player_controllers);
    }
}

fn cleanup_player_controllers(
    mut removed: ResMut<Shared<Receiver<ControllerRemoved>>>,
    mut commands: Commands,
) {
    while let Ok(ControllerRemoved(ent)) = removed.try_recv() {
        commands.entity(ent).despawn();
    }
}

fn handle_packets(
    mut packets: ResMut<Shared<Receiver<ControllerPacket>>>,
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

impl NetworkExtPriv for SubApp {
    fn register_message_handler<T: IntoSystem<In<(Entity, CPkt)>, (), Marker> + 'static, Marker>(&mut self, id: (u8, u8), system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut().get_resource_mut::<MessageHandlers>()
            .unwrap()
            .0
            .insert(id, system);
    }
}

impl NetworkExt for SubApp {
    async fn create_player_controller(&mut self, peer: Uuid, session: Uuid, sender: mpsc::UnboundedSender<WorldEvent>) -> WorldResult<Sender<ControllerEvent>> {
        let instance = self.world().get_resource::<ZoneInstance>().unwrap();
        let ctrl_removed = self.world().get_non_send_resource::<Sender<ControllerRemoved>>().unwrap().clone();
        let ctrl_packet = self.world().get_non_send_resource::<Sender<ControllerPacket>>().unwrap().clone();

        let state = instance.realm_api.get_session_state(session).await?
            .ok_or(anyhow::Error::msg("no active session for realm found"))?;

        let ent = self.world_mut().spawn(PlayerController {
            avatar_id: *state.avatar(),
            peer,
            sender,
        }).id();

        let (ctrl_sender, mut ctrl_receiver) = mpsc::channel(10);

        // Start receive loop for the client, to feed messages
        // into the world event loop.
        tokio::spawn(async move {
            while let Some(evt) = ctrl_receiver.recv().await {
                match evt {
                    ControllerEvent::Packet(cpkt) => {
                        let _ = ctrl_packet.send(ControllerPacket(ent, cpkt)).await;
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

#[derive(Component)]
pub struct PlayerController {
    avatar_id: AvatarId,
    peer: Uuid,
    sender: UnboundedSender<WorldEvent>,
}

impl PlayerController {
    pub fn send_packet(&self, packet: CPkt) {
        let _ = self.sender.send(WorldEvent::Packet {
            peer: self.peer,
            pkt: packet
        });
    }
}
