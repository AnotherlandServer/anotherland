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

use atlas::{AvatarId, ParamBox, ParamSetBox};
use bevy::app::{First, Last, Plugin};
use bevy_ecs::{component::Component, entity::Entity, event::{Event, EventWriter}, query::{Added, Changed, Without}, system::{Commands, Query}};

use crate::actors::AvatarComponent;

pub struct ParamsPlugin;

#[derive(Component)]
pub struct PreviousParamBox(pub ParamBox);

#[derive(Event)]
pub struct ParamsChangedEvent(pub Entity, pub AvatarId, pub ParamSetBox);

pub fn prepare_param_updates(
    spawned: Query<(Entity, &AvatarComponent, &ParamBox), (Added<ParamBox>, Without<PreviousParamBox>)>,
    mut cmds: Commands,
) {
    for (ent, avatar, params) in spawned.iter() {
        cmds.entity(ent)
            .insert(PreviousParamBox(params.clone()));
    }
}

pub fn send_param_update_events(
    mut params: Query<(Entity, &AvatarComponent, &ParamBox, &mut PreviousParamBox), Changed<ParamBox>>,
    mut ev: EventWriter<ParamsChangedEvent>,
) {
    ev.send_batch(
        params.iter_mut()
        .filter_map(|(entity, avatar, params, mut prev_params)| {
            let diff = params.diff(&prev_params.0);
            if diff.is_empty() {
                None
            } else {
                // store params for future comparison
                params.clone_into(&mut prev_params.0);

                Some(ParamsChangedEvent(entity, avatar.id, diff))
            }
        })
    );
}

impl Plugin for ParamsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ParamsChangedEvent>();
        app.add_systems(First, prepare_param_updates);
        app.add_systems(Last, send_param_update_events);
    }
}