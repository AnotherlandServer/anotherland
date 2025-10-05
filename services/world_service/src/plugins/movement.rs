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

use bevy::{app::{Plugin, PostUpdate, PreUpdate, Update}, ecs::{component::Component, system::Res, world::World}, math::{Quat, Vec3}, prelude::{Added, App, Changed, Commands, Entity, In, Query, With}, time::{Real, Time, Virtual}};
use log::{debug, error};
use mlua::{Lua, Table};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, Class, GameObjectData, NonClientBase, NpcOtherland, Player};
use protocol::{oaPktMoveManagerPosUpdate, oaPktMoveManagerStateChanged, Physics, PhysicsState};
use scripting::{EntityScriptCommandsExt, LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{OtherlandQuatExt, QuatWrapper, Vec3Wrapper};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{Navmesh}};

use super::{AvatarInfo, Interests, NetworkExtPriv, PlayerController};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_non_client_movement);
        app.add_systems(Update, extrapolate_player_positions);
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

    api.set("SetMoverKey", lua.create_bevy_function(world, |
            In((object, mover_key)): In<(Table, u16)>,
            mut query: Query<&mut Movement>,
        | -> WorldResult<()> {
            let mut movement = query.get_mut(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;
            
            movement.mover_key = mover_key;
            Ok(())
        })?)?;

    api.set("SetMoverType", lua.create_bevy_function(world, |
            In((object, mover_type)): In<(Table, u8)>,
            mut query: Query<&mut Movement>,
        | -> WorldResult<()> {
            let mut movement = query.get_mut(object.entity()?)
                .map_err(|_| anyhow!("object not found"))?;
            
            movement.mover_type = mover_type;
            Ok(())
        })?)?;  

    Ok(())
}

#[derive(Component)]
pub struct ForceSyncPositionUpdate;

#[derive(Component, Clone, Debug)]
pub struct Movement {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub radius: f32,
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
    mut commands: Commands,
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

        debug!("Avatar ID: {}", pkt.avatar_id);
        debug!("New Pos: {}", movement.position);
        debug!("New Rot: {:?} / {} / {}", pkt.rot, movement.rotation, movement.rotation.as_unit_vector());
        debug!("New Vel: {}", movement.velocity);
        debug!("New key: {}", movement.mover_key);
        debug!("New physics: {:?}", pkt.physics.state);

        commands
            .entity(ent)
            .insert(ForceSyncPositionUpdate)
            .fire_lua_event("OnPositionUpdated", (Vec3Wrapper(movement.position), QuatWrapper(movement.rotation), Vec3Wrapper(movement.velocity)));
    }
}

pub fn handle_move_manager_state_changed(
    In((ent, pkt)): In<(Entity, oaPktMoveManagerStateChanged)>,
    mut query: Query<&mut Movement>,
    mut commands: Commands,
) {
    if let Ok(mut movement) = query.get_mut(ent) {
        movement.mover_type = pkt.mover_type;
        movement.mover_replication_policy = pkt.mover_replication_policy;
        movement.version = pkt.new_version;
        movement.mover_key = pkt.mover_key;

        debug!("Player mover init: {movement:#?}");

        commands
            .entity(ent)
            .insert(ForceSyncPositionUpdate);
    }
}

pub fn setup_non_client_movement(
    mut query: Query<(Entity, &mut GameObjectData), Added<NonClientBaseTag>>,
    res: Res<Time<Virtual>>,
    navmesh: Res<Navmesh>,
    mut commands: Commands,
) {
    for (ent, mut obj) in query.iter_mut() {
        let mut pos = *obj.get::<_, Vec3>(NonClientBase::Pos).unwrap();
        let collision_extent = *obj.get::<_, Vec3>(NonClientBase::CollisionExtent).unwrap();

        if 
            obj.class() == Class::NpcOtherland &&
            obj.get::<_, f32>(NpcOtherland::MoveSpeed).copied().unwrap_or_default() > 0.0
        {
            pos.y = navmesh.get_floor_height(pos)
                .unwrap_or_else(|| {
                    error!("Failed to get floor height for NPC at position {pos}");
                    pos.y
                }) + collision_extent.y;
            
            obj.set(NonClientBase::Pos, pos);
        }

        let movement = Movement {
            position: *obj.get::<_, Vec3>(NonClientBase::Pos).unwrap(),
            rotation: Quat::from_unit_vector(*obj.get::<_, Vec3>(NonClientBase::Rot).unwrap()),
            velocity: Vec3::ZERO,
            radius: collision_extent.x.max(collision_extent.z),
            mode: PhysicsState::Walking,
            mover_type: 1,
            mover_replication_policy: 9,
            version: 1,
            seconds: res.elapsed_secs_f64(),
            mover_key: 0,
        };

        commands.entity(ent).insert(movement);
    }
}

#[allow(clippy::type_complexity)]
pub fn send_position_updates(
    positions: Query<(Entity, &AvatarInfo, &Movement), (Changed<Movement>, With<ForceSyncPositionUpdate>)>,
    players: Query<(&Interests, &PlayerController)>,
    mut commands: Commands,
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

        commands
            .entity(entity)
            .remove::<ForceSyncPositionUpdate>();
    }
}

pub fn extrapolate_player_positions(
    mut positions: Query<&mut Movement, With<PlayerTag>>,
    time: Res<Time<Real>>,
) {
    for mut mov in positions.iter_mut() {
        if mov.velocity != Vec3::ZERO {
            let vel = mov.velocity;

            mov.position += vel * time.delta_secs();
        }
    }
}
