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

use bevy::{app::{Plugin, PostUpdate, PreUpdate, Update}, ecs::{change_detection::DetectChangesMut, component::Component, system::Res, world::World}, math::{Quat, Vec3}, prelude::{Added, App, Changed, Commands, Entity, In, Query, With}, time::{Real, Time}};
use log::debug;
use mlua::{Lua, Table};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase, Player};
use protocol::{oaPktMoveManagerPosUpdate, oaPktMoveManagerStateChanged, Physics, PhysicsState};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{OtherlandQuatExt, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::error::WorldResult;

use super::{AvatarInfo, Interests, NetworkExtPriv, PlayerController};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_non_client_movement);
        app.add_systems(Update, update_position);
        app.add_systems(PostUpdate, send_position_updates);

        app.register_message_handler(handle_move_manager_state_changed);
        app.register_message_handler(handle_move_manager_pos_update);

        insert_movement_api(app.world_mut()).unwrap();
    }
}

fn insert_movement_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let api = lua.create_table().unwrap();
    runtime.register_native("movement", api.clone()).unwrap();

    api.set("GetPosition", lua.create_bevy_function(world, 
        |
            In(object): In<Table>,
            query: Query<&Movement>,
        | -> WorldResult<Vec3Wrapper> {
            let movement = query.get(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;
            
            Ok(Vec3Wrapper(movement.position))
        })?)?;

    api.set("GetRotation", lua.create_bevy_function(world, 
        |
            In(object): In<Table>,
            query: Query<&Movement>,
        | -> WorldResult<QuatWrapper> {
            let movement = query.get(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;
            
            Ok(QuatWrapper(movement.rotation))
        })?)?;

    api.set("GetVelocity", lua.create_bevy_function(world, |
            In(object): In<Table>,
            query: Query<&Movement>,
        | -> WorldResult<Vec3Wrapper> {
            let movement = query.get(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;
            
            Ok(Vec3Wrapper(movement.velocity))
        })?)?;

    Ok(())
}

#[derive(Component, Clone, Debug)]
pub struct Movement {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub mode: PhysicsState,
    pub mover_type: u8,
    pub mover_replication_policy: u8,
    pub version: u16,
    pub mover_key: u16,
    pub seconds: f64,
}

pub fn handle_move_manager_pos_update(
    In((ent, pkt)): In<(Entity, oaPktMoveManagerPosUpdate)>,
    mut query: Query<(&mut GameObjectData, &mut Movement), With<PlayerTag>>,
) {
    if let Ok((mut obj, mut movement)) = query.get_mut(ent) {
        movement.mode = pkt.physics.state;
        movement.position = pkt.pos.into();
        movement.rotation = pkt.rot.clone().into();
        movement.velocity = pkt.vel.into();
        movement.seconds = pkt.seconds;
        movement.mover_key = pkt.mover_key;

        obj.set(Player::Pos, (0u32, movement.position));
        obj.set(Player::Rot, movement.rotation.as_unit_vector());

        debug!("New Pos: {}", movement.position);
        debug!("New Rot: {:?} / {} / {}", pkt.rot, movement.rotation, movement.rotation.as_unit_vector());
        debug!("New Vel: {}", movement.velocity);
        debug!("New key: {}", movement.mover_key);
    }
}

pub fn handle_move_manager_state_changed(
    In((ent, pkt)): In<(Entity, oaPktMoveManagerStateChanged)>,
    mut query: Query<&mut Movement>,
) {
    if let Ok(mut movement) = query.get_mut(ent) {
        movement.mover_type = pkt.mover_type;
        movement.mover_replication_policy = pkt.mover_replication_policy;
        movement.version = pkt.new_version;
        movement.mover_key = pkt.mover_key;

        debug!("Player mover init: {movement:#?}");
    }
}

pub fn setup_non_client_movement(
    query: Query<(Entity, &GameObjectData), Added<NonClientBaseTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in query.iter() {
        let movement = Movement {
            position: *obj.get::<_, Vec3>(NonClientBase::Pos).unwrap(),
            rotation: Quat::from_unit_vector(*obj.get::<_, Vec3>(NonClientBase::Rot).unwrap()),
            velocity: Vec3::ZERO,
            mode: PhysicsState::Walking,
            mover_type: 1,
            mover_replication_policy: 7,
            version: 1,
            seconds: 0.0,
            mover_key: 0,
        };

        commands.entity(ent).insert(movement);
    }
}

pub fn send_position_updates(
    positions: Query<(Entity, &AvatarInfo, &Movement), Changed<Movement>>,
    players: Query<(&Interests, &PlayerController)>,
) {
    for (entity, avatar, pos) in positions.iter() {
        // check player interest list to dispatch updates
        for (interests, controller) in players.iter() {
            if interests.contains_key(&entity) {
                controller.send_packet(oaPktMoveManagerPosUpdate {
                    avatar_id: avatar.id,
                    pos: pos.position.into(),
                    rot: pos.rotation.into(),
                    vel: pos.velocity.into(),
                    physics: Physics {
                        state: pos.mode,
                    },
                    mover_key: pos.mover_key,
                    seconds: pos.seconds,
                    ..Default::default()
                });
            }
        }
    }
}

pub fn update_position(
    mut positions: Query<&mut Movement>,
    time: Res<Time<Real>>,
) {
    for mut pos in positions.iter_mut() {
        let vel = pos.velocity;

        pos.bypass_change_detection().position += vel * time.delta_secs();
    }
}
