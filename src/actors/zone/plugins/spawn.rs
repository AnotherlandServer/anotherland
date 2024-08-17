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

use std::time::Instant;

use atlas::{AvatarId, AvatarType, NonClientBaseComponent, NonClientBaseParams, ParamBox, Uuid};
use bevy::{app::{Plugin, Update}, prelude::{Event, EventReader, ResMut}};
use bevy_ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query}};
use rand::{thread_rng, Rng};

use crate::actors::{zone::components, AvatarComponent, AvatarIdToEntityLookup, EntityType, SlowUpdate, Spawned, SpawnerState, UuidToEntityLookup};

#[derive(Event)]
pub struct SpawnNonPlayerAvatarEvent {
    pub id: Option<AvatarId>,
    pub name: String,
    pub entity_type: EntityType,
    pub phase_tag: String,
    pub instance_id: Option<Uuid>,
    pub content_id: Option<Uuid>,
    pub params: ParamBox
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnNonPlayerAvatarEvent>();
        app.add_systems(SlowUpdate, respawn);
        app.add_systems(Update, spawn_non_player_avatar);
    }
}

pub fn respawn(
    mut commands: Commands, 
    mut query: Query<(Entity, &ParamBox, &mut SpawnerState), (With<NonClientBaseComponent>, Without<Spawned>)>
) {
    let now = Instant::now();

    for (entity, base, mut state) in query.iter_mut()
        .map(|(e, p, s)| (e, p.get_impl::<dyn NonClientBaseParams>().unwrap(), s))
    {
        // first check if the entity is enabled at all
        if base.enable_in_game() {
            if let Some(instant) = state.respawn_instant {
                // respawn when we reached the set time
                if instant <= now {
                    // reset state
                    state.despawn_instant = None;
                    state.respawn_instant = None;

                    commands.entity(entity)
                        .insert(Spawned);
                }

                // if no explicit instant for respaning is set, respawn immediately
            } else {
                commands.entity(entity)
                        .insert(Spawned);
            }
        }

        
    }
}

pub fn spawn_non_player_avatar(
    mut events: EventReader<SpawnNonPlayerAvatarEvent>,
    mut commands: Commands,
    mut avatar_id_to_entity: ResMut<AvatarIdToEntityLookup>,
    mut uuid_to_entity: ResMut<UuidToEntityLookup>,
) {
    for ev in events.read() {
        let id = match ev.id {
            Some(id) => id,
            None => {
                let mut rng = thread_rng();
                loop {
                    let id = AvatarId::new(rng.gen_range(1..1<<56) << 0xF, AvatarType::Npc);
                    if !avatar_id_to_entity.contains_key(&id) {
                        break id;
                    }
                }
            }
        };

       let entity = ev.params.build_entity(&mut commands)
            .insert(ev.entity_type)
            .insert(AvatarComponent {
                id,
                instance_id: ev.instance_id,
                record_id: ev.content_id,
                name: ev.name.clone(),
                phase_tag: ev.phase_tag.clone(),
            })
            .insert(Spawned)
            .id();

        if let Some(params) = ev.params.get_impl::<dyn NonClientBaseParams>() {
            for tag in params.tags().split(' ') {
                match tag {
                    "RespawnPoint" => { commands.entity(entity).insert(components::RespawnPoint); }
                    "PortalHive" => { commands.entity(entity).insert(components::PortalHive); }
                    "InteractionTell" => { commands.entity(entity).insert(components::InteractionTell); }
                    _ => (),
                };
            }
        }

        avatar_id_to_entity.insert(id, entity);

        if let Some(id) = ev.instance_id {
            uuid_to_entity.insert(id, entity);
        }
    }
}