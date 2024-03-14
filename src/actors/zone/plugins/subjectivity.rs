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

use atlas::ParamBox;
use bevy::{app::{App, Plugin}, utils::HashMap};
use bevy_ecs::{entity::Entity, system::{IntoSystem, Resource, System}, world::Mut};

use crate::actors::EntityType;

pub type SubjectivityLensArguments = (Entity, Entity);
type SubjectivityLensSystem = dyn System<In = SubjectivityLensArguments, Out = ParamBox>;
type EntityTypeSubjectivityLensSystemMap = HashMap<EntityType, Box<SubjectivityLensSystem>>;

#[derive(Resource)]
struct SubjectivityMap(EntityTypeSubjectivityLensSystemMap);

pub struct SubjectivityPlugin;

impl Plugin for SubjectivityPlugin {
    fn build(&self, app: &mut App) {
        app.world.insert_resource(SubjectivityMap(EntityTypeSubjectivityLensSystemMap::new()));
    }
}

pub trait SubjectivityExt {
    fn add_subjective_lens<T: IntoSystem<SubjectivityLensArguments, ParamBox, Marker>, Marker>(&mut self, entity_type: EntityType, system: T) -> &mut Self;
    fn get_subjective_params(&mut self, player: Entity, avatar: Entity) -> Option<ParamBox>;
}

impl SubjectivityExt for App {
    fn add_subjective_lens<T: IntoSystem<SubjectivityLensArguments, ParamBox, Marker>, Marker>(&mut self, entity_type: EntityType, system: T) -> &mut Self {
        let mut system = IntoSystem::into_system(system);
        system.initialize(&mut self.world);

        if let Some(mut entity_type_subjectivity_map) = self.world.get_resource_mut::<SubjectivityMap>() {
            entity_type_subjectivity_map.0.insert(entity_type, Box::new(system));
        }

        self
    }

    fn get_subjective_params(&mut self, player: Entity, avatar: Entity) -> Option<ParamBox> {
        self.world.resource_scope(|world, mut subjectivity_map: Mut<SubjectivityMap>| {
            if let Some(entity_type) = world.get::<EntityType>(avatar) {
                if let Some(system) = subjectivity_map.0.get_mut(entity_type) {
                    Some(system.run((player, avatar), world))
                } else {
                    world.get::<ParamBox>(avatar).cloned()
                }
            } else {
                world.get::<ParamBox>(avatar).cloned()
            }
        })
    }
}