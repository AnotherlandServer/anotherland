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

use atlas::{NonClientBaseComponent, NonClientBaseParams, ParamBox, PlayerComponent, PlayerParams};
use bevy::app::{First, Plugin, PostUpdate};
use bevy_ecs::{change_detection::DetectChangesMut, component::Component, entity::Entity, query::{Added, Changed, Or, With}, system::{Commands, Query}};
use glam::{Quat, Vec3};

use crate::{actors::PhysicsState, util::OtherlandQuatExt};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(First, init_position);
        app.add_systems(PostUpdate, sync_position_params);
    }
}

#[derive(Clone, Component)]
pub struct Position {
    pub mover_key: u16,
    pub replica: u8,
    pub version: u16,
    pub seconds: f64,

    pub physics_state: PhysicsState,

    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
}

fn init_position(
    query: Query<(Entity, &ParamBox), (Added<ParamBox>, With<NonClientBaseComponent>)>,
    mut cmd: Commands,
) {
    for (entity, params) in query.iter() {
        if let Some(non_client_base) = params.get_impl::<dyn NonClientBaseParams>() {
            cmd.entity(entity)
                .insert(Position {
                    mover_key: 0,
                    replica: 7,
                    version: 1,
                    seconds: 0.0,
                    physics_state: PhysicsState::Walking,
                    position: *non_client_base.pos(),
                    rotation: Quat::from_unit_vector(*non_client_base.rot()),
                    velocity: Vec3::default(),
                });
        } else {
            unimplemented!()
        }
    }
}

fn sync_position_params(
    mut query: Query<(&Position, &mut ParamBox), Changed<Position>>,
) {
    for (pos, mut params) in query.iter_mut() {
        // bypass change detection, because movements are synchronized in specialized messages

        if let Some(player) = params.bypass_change_detection().get_impl_mut::<dyn PlayerParams>() {
            player.set_pos((0, pos.position));
            player.set_rot(pos.rotation.as_unit_vector());
        } else if let Some(non_client_base) = params.bypass_change_detection().get_impl_mut::<dyn NonClientBaseParams>() {
            non_client_base.set_pos(pos.position);
            non_client_base.set_rot(pos.rotation.as_unit_vector());
        } else {
            unimplemented!()
        }
    }
}