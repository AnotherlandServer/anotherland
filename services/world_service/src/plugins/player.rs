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

use bevy::{app::{First, Plugin, Update}, math::{Quat, Vec3, VectorSpace}, prelude::{Added, Changed, Commands, Entity, IntoSystemConfigs, Query, Res, ResMut}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;
use obj_params::{GameObjectData, ParamWriter, Player};
use protocol::{oaPktS2XConnectionState, CPkt, CPktBlob, MoveManagerInit, OaPktS2xconnectionStateState, OtherlandPacket, Physics, PhysicsState};
use realm_api::Character;
use scripting::LuaRuntime;
use tokio::sync::mpsc::{self, Receiver, Sender};
use toolkit::OtherlandQuatExt;

use crate::{instance::ZoneInstance, plugins::ForeignResource};

use super::{init_gameobjects, AvatarIdManager, AvatarInfo, ConnectionState, CurrentState, EnabledInGame, Movement, PlayerController, ServerAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (character_sender, character_receiver) = mpsc::channel::<(Entity, Character)>(10);

        app.insert_resource(ForeignResource(character_sender));
        app.insert_resource(ForeignResource(character_receiver));

        app.add_systems(First, (
            request_player_characters, 
            insert_player_characters.before(init_gameobjects)
        ));

        app.add_systems(Update, spawn_player);
    }
}

fn request_player_characters(
    query: Query<(Entity, &PlayerController), Added<PlayerController>>,
    instance: Res<ZoneInstance>,
    sender: Res<ForeignResource<Sender<(Entity, Character)>>>,
) {
    for (entity, controller) in query.iter() {
        let realm_api = instance.realm_api.clone();
        let sender = sender.clone();
    
        let state = controller.state().clone();
    
        instance.handle.spawn(async move {
            if let Ok(Some(character)) = realm_api.get_character(state.character()).await {
                let _ = sender.send((entity, character)).await;
            }
        });
    }
}

fn insert_player_characters(
    mut receiver: ResMut<ForeignResource<Receiver<(Entity, Character)>>>,
    mut runtime: ResMut<LuaRuntime>,
    controller: Query<&PlayerController>,
    instance: Res<ZoneInstance>,
    mut avatar_id_manager: ResMut<AvatarIdManager>,
    mut commands: Commands,
) {
    while let Ok((entity, mut character)) = receiver.try_recv() {
        if let Ok(controller) = controller.get(entity) {
            let avatar_entry = avatar_id_manager.avatar_entry(controller.avatar_id());
            avatar_entry.insert(entity);

            debug!("Starting spawning sequence for character: {}", character.name());

            let mut serialized = Vec::new();
            let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

            // Update zone info in character data
            character.data_mut().set(Player::ZoneGuid, instance.zone.guid().to_string());
            character.data_mut().set(Player::InstanceZoneKey, instance.instance_id.map(|v| v.to_string()).unwrap_or_default());

            // TODO: Save character data too!

            character.data().write_to_privileged_client(&mut writer).unwrap();

            let movement = Movement {
                position: character.data().get::<_, (u32, Vec3)>(Player::Pos).unwrap().1,
                rotation: Quat::from_unit_vector(*character.data().get::<_, Vec3>(Player::Rot).unwrap()),
                velocity: Vec3::ZERO,
                mode: PhysicsState::Walking,
                mover_type: 1,
                mover_replication_policy: 7,
                version: 0,
            };

            // Send character to client, so it begins loading the level
            controller.send_packet(CPktBlob {
                avatar_id: controller.avatar_id(),
                avatar_name: character.name().to_owned(),
                class_id: character.data().class().id() as u32,
                params: serialized.into(),
                movement:  MoveManagerInit {
                    pos: movement.position.into(),
                    rot: movement.rotation.into(),
                    vel: movement.velocity.into(),
                    physics: Physics {
                        state: movement.mode,
                    },
                    mover_type: movement.mover_type,
                    mover_replication_policy: movement.mover_replication_policy,
                    version: movement.version,
                    ..Default::default()
                }.to_bytes().into(),
                has_guid: true,
                field_7: Some(*controller.session().id()),
                ..Default::default()
            });

            // Insert character into world
            commands.entity(entity)
                .insert((
                    AvatarInfo {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    movement
                ));
        }
    }
}

fn spawn_player(
    mut query: Query<(Entity, &AvatarInfo, &Movement, &mut PlayerController, &mut CurrentState), Changed<CurrentState>>, 
    mut commands: Commands,
) {
    for (ent, info, movement, mut controller, mut state) in query.iter_mut() {
        if matches!(state.state, ConnectionState::InitialInterestsLoaded) {
            debug!("Spawning player: {}", info.name);

            state.state = ConnectionState::InGame;
            state.version += 1;

            controller.send_packet(oaPktS2XConnectionState {
                state: OaPktS2xconnectionStateState::InGame,
                ..Default::default()
            });

            commands.entity(ent).insert(EnabledInGame);

            let spawn_action = if let Some(action) = controller.take_spawn_action() {
                action
            } else {
                ServerAction::DirectTravel(info.id, Some(movement.clone()))
            };

            controller.send_packet(spawn_action.into_pkt());
        }
    }
}