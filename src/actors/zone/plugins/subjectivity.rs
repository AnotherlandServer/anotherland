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

use atlas::{AvatarId, NonClientBaseComponent, ParamBox, ParamSetBox, PlayerComponent};
use bevy::{app::{App, First, Last, Plugin}, utils::{HashMap, HashSet}};
use bevy_ecs::{component::Component, entity::Entity, event::{Event, EventWriter}, query::{Added, Changed, With, Without}, removal_detection::RemovedComponents, system::{Commands, IntoSystem, Query, Resource, System}, world::{Mut, World}};
use log::debug;

use crate::actors::{AvatarComponent, EntityType};

pub type SubjectivityLensArguments = (Entity, Entity);
type SubjectivityLensSystem = dyn System<In = SubjectivityLensArguments, Out = ParamSetBox>;
type EntityTypeSubjectivityLensSystemMap = HashMap<EntityType, Box<SubjectivityLensSystem>>;

#[derive(Resource)]
struct SubjectivityMap(EntityTypeSubjectivityLensSystemMap);

#[derive(Component)]
pub struct SubjectiveParamSet(HashMap<Entity, ParamSetBox>);

#[derive(Component)]
pub struct BackupSubjectiveParamSet(HashMap<Entity, ParamSetBox>);

#[derive(Event)]
pub struct SubjectiveParamsChangedEvent {
    pub entity: Entity, 
    pub avatar: AvatarId, 
    pub player: Entity,
    pub params: ParamSetBox,
}

impl SubjectiveParamSet {
    pub fn params_for_player(&self, player: Entity, params: &ParamBox) -> ParamBox {
        let mut params = params.clone();

        if 
            let Some(subjective_set) = self.0.get(&player) &&
            !subjective_set.is_empty()
        {
            params.apply(subjective_set.clone());
        }

        params
    }

    pub fn get_params(&self, player: Entity) -> Option<&ParamSetBox> {
        self.0.get(&player)
    }

    pub fn get_params_mut(&mut self, player: Entity) -> Option<&mut ParamSetBox> {
        self.0.get_mut(&player)
    }
}

pub struct SubjectivityPlugin;

impl Plugin for SubjectivityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SubjectiveParamsChangedEvent>();
        
        app.add_systems(First, prepare_subjective_param_updates);
        app.add_systems(First, cleanup_subjective_params);
        app.add_systems(Last, send_subjective_param_update_events);

        app.world.insert_resource(SubjectivityMap(EntityTypeSubjectivityLensSystemMap::new()));
    }
}

pub trait SubjectivityExt {
    fn add_subjective_params_initializer<T: IntoSystem<SubjectivityLensArguments, ParamSetBox, Marker>, Marker>(&mut self, entity_type: EntityType, system: T) -> &mut Self;
    fn get_subjective_params(&mut self, player: Entity, avatar: Entity) -> Option<ParamBox>;
}

impl SubjectivityExt for App {
    fn add_subjective_params_initializer<T: IntoSystem<SubjectivityLensArguments, ParamSetBox, Marker>, Marker>(&mut self, entity_type: EntityType, system: T) -> &mut Self {
        let mut system = IntoSystem::into_system(system);
        system.initialize(&mut self.world);

        if let Some(mut entity_type_subjectivity_map) = self.world.get_resource_mut::<SubjectivityMap>() {
            entity_type_subjectivity_map.0.insert(entity_type, Box::new(system));
        }

        self
    }

    fn get_subjective_params(&mut self, player: Entity, avatar: Entity) -> Option<ParamBox> {
        if let Some(params) = self.world.get::<ParamBox>(avatar) {
            if let Some(subjective_params) = self.world.get::<SubjectiveParamSet>(avatar) {
                Some(subjective_params.params_for_player(player, params))
            } else {
                Some(params.clone())
            }
        } else {
            None
        }
    }
}

pub fn prepare_subjective_param_updates(
    spawned_players: Query<Entity, Added<PlayerComponent>>,
    avatars: Query<Entity, With<NonClientBaseComponent>>,
    mut cmds: Commands,
) {
    for player_ent in spawned_players.iter() {
        for ent in avatars.iter() {
            cmds.add(move |world: &mut World| {
                world.resource_scope(|world, mut subjectivity_map: Mut<SubjectivityMap>| {
                    if let Some(entity_type) = world.get::<EntityType>(ent) {
                        if let Some(system) = subjectivity_map.0.get_mut(entity_type) {
                            let set = system.run((player_ent, ent), world);

                            if let Some(mut subjective_params) = world.get_mut::<SubjectiveParamSet>(ent) {
                                subjective_params.0.insert(player_ent, set);
                            } else if let Some(mut ent) = world.get_entity_mut(ent) {
                                let mut subjective_set = SubjectiveParamSet(HashMap::new());

                                subjective_set.0.insert(player_ent, set);

                                ent
                                    .insert(subjective_set)
                                    .insert(BackupSubjectiveParamSet(HashMap::new()));
                            }
                        }
                    }
                })
            });
        }
    }
}

fn cleanup_subjective_params(
    mut disconnected: RemovedComponents<PlayerComponent>,
    mut subjective_entities: Query<&mut SubjectiveParamSet>,
) {
    let disconnected: Vec<Entity> = disconnected.read().collect();

    for mut params in subjective_entities.iter_mut() {
        for entity in disconnected.iter() {
            params.0.remove(entity);
        }
    }
}

pub fn send_subjective_param_update_events(
    mut params: Query<(Entity, &AvatarComponent, &SubjectiveParamSet, &mut BackupSubjectiveParamSet), Changed<SubjectiveParamSet>>,
    mut ev: EventWriter<SubjectiveParamsChangedEvent>,
) {
    ev.send_batch(
        params.iter_mut()
        .flat_map(|(entity, avatar, params, mut prev_params)| {
            let players = prev_params.0
                .keys()
                .chain(params.0.keys())
                .cloned()
                .collect::<HashSet<Entity>>();

            let mut events = Vec::new();
            
            for player in players {
                if let Some(params) = params.0.get(&player) {
                    if let Some(prev_params) = prev_params.0.get_mut(&player) {
                        let diff = params.diff(prev_params);
                        if !diff.is_empty() {
                            // store params for future comparison
                            params.clone_into(prev_params);

                            debug!("Subjective params updated");
            
                            events.push(SubjectiveParamsChangedEvent {
                                entity,
                                avatar: avatar.id,
                                player,
                                params: diff,
                            });
                        }
                    } else {
                        prev_params.0.insert(player, params.clone());
                    }
                } else {
                    prev_params.0.remove(&player);
                }
            }

            events
        })
    );
}