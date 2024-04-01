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

use atlas::{MightIncludeParamsMut, NonClientBaseComponent, NonClientBaseParams, ParamBox};
use bevy_ecs::{entity::Entity, query::With, system::{Commands, Query, ResMut}};
use log::info;

use crate::actors::{zone::resources::EventInfos, AvatarComponent, Spawned};

pub fn sepcial_event_controller(
    mut event_infos: ResMut<EventInfos>, 
    mut commands: Commands,
    mut query: Query<(Entity, &AvatarComponent, &mut ParamBox), With<NonClientBaseComponent>>,
) {
    // check event status
    for event_info in event_infos.0.iter_mut() {
        if event_info.event.is_active() {
            // check if we have to activate the event
            if !event_info.active.unwrap_or(false) {
                info!("Enabling event: {}", event_info.event.name());

                for (ent, avatar, mut base) in query.iter_mut() 
                    .map(|(e, a, p)| (e, a, p.map_unchanged(|p| p.get_impl_mut::<dyn NonClientBaseParams>().unwrap())))
                {
                    if let Some(instance_id) = avatar.instance_id {
                        if event_info.event.event_instances.contains(&instance_id) {
                            base.set_enable_in_game(true);
                        } else if event_info.event.hidden_instances.contains(&instance_id) {
                            base.set_enable_in_game(false);
                            commands.entity(ent).remove::<Spawned>();
                        }
                    }

                    if let Some(content_id) = avatar.record_id {
                        if event_info.event.content.contains(&content_id) {
                            base.set_enable_in_game(true);
                        }
                    }
                }

                event_info.active = Some(true);
            }
        } else {
            // check if we have to deactivate the event
            if event_info.active.unwrap_or(true) {
                info!("Disabling event: {}", event_info.event.name());

                for (ent, avatar, mut base) in query.iter_mut() 
                    .map(|(e, a, p)| (e, a, p.map_unchanged(|p| p.get_impl_mut::<dyn NonClientBaseParams>().unwrap())))
                {
                    if let Some(instance_id) = avatar.instance_id {
                        if event_info.event.event_instances.contains(&instance_id) {
                            base.set_enable_in_game(false);
                            commands.entity(ent).remove::<Spawned>();
                        } else if event_info.event.hidden_instances.contains(&instance_id) {
                            base.set_enable_in_game(true);
                        }
                    }

                    if let Some(content_id) = avatar.record_id {
                        if event_info.event.content.contains(&content_id) {
                            base.set_enable_in_game(false);
                            commands.entity(ent).remove::<Spawned>();
                        }
                    }
                }

                event_info.active = Some(false);
            }
        }
    }
}