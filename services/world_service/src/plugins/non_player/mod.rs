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

mod zone_loader;
mod non_player_loader;
mod selector;
mod interaction;
mod npc_ai;

use std::time::Duration;

use bevy::{app::{App, Plugin, Update}, ecs::{lifecycle::HookContext, schedule::IntoScheduleConfigs}, time::common_conditions::on_timer};
use obj_params::Class;
pub use zone_loader::*;
pub use non_player_loader::*;
pub use selector::*;
pub use interaction::*;
pub use npc_ai::*;

use crate::plugins::{BehaviorExt, process_health_events};

pub struct NonPlayerPlugin;

impl Plugin for NonPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiStates>();

        app.add_systems(Update, (
                ai_tick
                    .run_if(on_timer(Duration::from_millis(100)))
                    .before(process_health_events),
                handle_interactions
            ));
        app
            .world_mut()
            .register_component_hooks::<AiAgent>()
            .on_remove(|mut world, HookContext { entity, .. }| {
                if let Some(mut states) = world.get_resource_mut::<AiStates>() {
                    states.0.remove(&entity);
                }
            });

        app.register_string_behavior(Class::LootScatterContainer, "interact", behavior_loot_scatter_container_interact);

        insert_npc_ai_api(app.world_mut()).unwrap();
    }
}