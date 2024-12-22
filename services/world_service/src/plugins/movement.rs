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

use bevy::{app::{Plugin, PreUpdate}, math::{Quat, Vec3}, prelude::{Added, App, Commands, Component, Entity, In, Query, With, Without}};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase, Player};
use protocol::{oaPktMoveManagerPosUpdate, oaPktMoveManagerStateChanged, CPkt, PhysicsState};
use toolkit::OtherlandQuatExt;

use super::NetworkExtPriv;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_non_client_movement);

        app.register_message_handler::<oaPktMoveManagerStateChanged, _, _>(handle_move_manager_state_changed);
        app.register_message_handler::<oaPktMoveManagerPosUpdate, _, _>(handle_move_manager_pos_update);
    }
}

#[derive(Component, Clone)]
pub struct Movement {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub mode: PhysicsState,
    pub mover_type: u8,
    pub mover_replication_policy: u8,
    pub version: u16,
}

pub fn handle_move_manager_pos_update(
    In((ent, pkt)): In<(Entity, CPkt)>,
    mut query: Query<&mut Movement>,
) {
    if
        let Ok(mut movement) = query.get_mut(ent) &&
        let CPkt::oaPktMoveManagerPosUpdate(pkt) = pkt
    {
        movement.mode = pkt.physics.state;
        movement.position = pkt.pos.into();
        movement.rotation = pkt.rot.into();
        movement.velocity = pkt.vel.into();
    }
}

pub fn handle_move_manager_state_changed(
    In((ent, pkt)): In<(Entity, CPkt)>,
    mut query: Query<&mut Movement>,
) {
    if
        let Ok(mut movement) = query.get_mut(ent) &&
        let CPkt::oaPktMoveManagerStateChanged(pkt) = pkt
    {
        movement.mover_type = pkt.mover_type;
        movement.mover_replication_policy = pkt.mover_replication_policy;
        movement.version = pkt.new_version;
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
            version: 0,
        };

        commands.entity(ent).insert(movement);
    }
}