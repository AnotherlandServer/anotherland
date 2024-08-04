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

use atlas::{NonClientBaseComponent, NpcOtherlandComponent, ParamBox};
use bevy::app::{Plugin, PostUpdate};
use bevy_ecs::{entity::Entity, event::{Event, EventReader}, query::With, system::Query};
use log::debug;

use crate::actors::current_gametime;

use super::SubjectiveParamSet;

#[derive(Event)]
pub enum PlayAnimationEvent {
    QueueAnimation{ entity: Entity, animation: String },
    InterruptAnimation{ entity: Entity, animation: String },
    PlayerInterruptAnimation{ player: Entity, entity: Entity, animation: String },
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayAnimationEvent>();
        app.add_systems(PostUpdate, play_animations);
    }
}

fn play_animations(
    mut events: EventReader<PlayAnimationEvent>,
    mut query: Query<(&mut ParamBox, Option<&mut SubjectiveParamSet>), With<NonClientBaseComponent>>,
) {
    for event in events.read() {
        match event {
            PlayAnimationEvent::QueueAnimation { entity, animation } => {
                if 
                    let Ok((mut params, _)) = query.get_mut(*entity)
                {
                    params.set_param("action0", (animation.to_owned(), current_gametime().as_secs_f32()));
                    params.set_param("action0Duration", 4.0); // TODO: Set actual animation duration
                    params.set_param("action0Option", 1); // TODO: Not sure what that does
                }
            },
            PlayAnimationEvent::InterruptAnimation { entity, animation } => {
                if 
                    let Ok((mut params, _)) = query.get_mut(*entity)
                {
                    params.set_param("action0", (animation.to_owned(), current_gametime().as_secs_f32()));
                    params.set_param("action0Duration", 4.0); // TODO: Set actual animation duration
                    params.set_param("action0Option", 1); // TODO: Not sure what that does
                }
            },
            PlayAnimationEvent::PlayerInterruptAnimation { player, entity, animation } => {
                if 
                    let Ok((_, Some(mut subjective_params))) = query.get_mut(*entity) &&
                    let Some(params) = subjective_params.get_params_mut(*player)
                {
                    debug!("Play animation '{}' for player {:?}", animation, *player);

                    params.set_param("action0", (animation.to_owned(), current_gametime().as_secs_f32()));
                    params.set_param("action0Duration", 4.0); // TODO: Set actual animation duration
                    params.set_param("action0Option", 1); // TODO: Not sure what that does
                }
            }
        }
    }
}
