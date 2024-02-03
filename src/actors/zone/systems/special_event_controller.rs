// Copyright (C) 2023 AnotherlandServer
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

use std::{sync::Arc, time::SystemTime};

use atlas::{NonClientBaseComponent, Uuid};
use chrono::{DateTime, Local};
use log::info;
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{actors::{AvatarComponent, Spawned}, components::SpecialMapEvent};

pub struct EventInfo {
    pub event: SpecialMapEvent,
    pub active: Option<bool>,
}

pub struct SpecialEventController;

impl<'a> System<'a> for SpecialEventController {
    type SystemData = (
        WriteExpect<'a, Vec<EventInfo>>,
        Entities<'a>,
        ReadStorage<'a, AvatarComponent>,
        WriteStorage<'a, NonClientBaseComponent>,
        WriteStorage<'a, Spawned>,
    );

    fn run(&mut self, (
        mut event_infos,
        entities,
        avatar, 
        mut base,
        mut spawned
    ): Self::SystemData) {
        // check event status
        for event_info in event_infos.iter_mut() {
            if event_info.event.is_active() {
                // check if we have to activate the event
                if !event_info.active.unwrap_or(false) {
                    info!("Enabling event: {}", event_info.event.name());

                    for (ent, avatar, base) in (&entities, &avatar, &mut base).join() {
                        if let Some(instance_id) = avatar.instance_id {
                            if event_info.event.event_instances.contains(&instance_id) {
                                base.set_enable_in_game(true);
                            } else if event_info.event.hidden_instances.contains(&instance_id) {
                                base.set_enable_in_game(false);
                                spawned.remove(ent);
                            }
                        }

                        if let Some(content_id) = avatar.content_id {
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

                    for (ent, avatar, base) in (&entities, &avatar, &mut base).join() {
                        if let Some(instance_id) = avatar.instance_id {
                            if event_info.event.event_instances.contains(&instance_id) {
                                base.set_enable_in_game(false);
                                spawned.remove(ent);
                            } else if event_info.event.hidden_instances.contains(&instance_id) {
                                base.set_enable_in_game(true);
                            }
                        }

                        if let Some(content_id) = avatar.content_id {
                            if event_info.event.content.contains(&content_id) {
                                base.set_enable_in_game(false);
                                spawned.remove(ent);
                            }
                        }
                    }

                    event_info.active = Some(false);
                }
            }
        }
    }
}
