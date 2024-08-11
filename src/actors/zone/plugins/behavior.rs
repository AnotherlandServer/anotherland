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

use atlas::NativeParam;
use bevy::{app::{App, Plugin, PreUpdate}, utils::HashMap};
use bevy_ecs::{entity::Entity, event::{Event, Events}, system::{IntoSystem, Resource, System}, world::{Mut, World}};
use log::warn;

use crate::actors::{AvatarComponent, EntityType};

#[derive(Debug)]
pub enum Behavior {
    String(String, Vec<String>),
    Binary(String, NativeParam),
}

impl Behavior {
    pub fn name(&self) -> &str {
        match self {
            Behavior::String(name, _) => name,
            Behavior::Binary(name, _) => name,
        }
    }
}

#[derive(Event)]
pub struct RequestBehavior {
    pub entity: Entity,
    pub behavior: Behavior,
}

#[derive(Event)]
pub struct TellBehavior {
    pub instigator: Entity,
    pub target: Entity,
    pub behavior: Behavior,
}

pub type BehaviorArguments = (Entity, Entity, Behavior);
type BehaviorSystem = dyn System<In = BehaviorArguments, Out = ()>;
type BehaviorSystemMap = HashMap<String, Box<BehaviorSystem>>;
type EntityTypeBehaviorSystemMap = HashMap<EntityType, BehaviorSystemMap>;

#[derive(Resource)]
struct BehaviorMap(EntityTypeBehaviorSystemMap);

pub struct AvatarBehaviorPlugin;

impl Plugin for AvatarBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TellBehavior>();
        app.add_event::<RequestBehavior>();

        app.world.insert_resource(BehaviorMap(EntityTypeBehaviorSystemMap::new()));

        app.add_systems(PreUpdate, (perform_told_behavior, perform_requested_behavior));
    }
}

pub trait BehaviorExt {
    fn add_behavior<T: IntoSystem<BehaviorArguments, (), Marker>, Marker>(&mut self, entity_type: EntityType, name: &str, system: T) -> &mut Self;
    fn tell_behavior(&mut self, instigator: Entity, target: Entity, behavior: String);
    fn tell_behavior_binary(&mut self, instigator: Entity, target: Entity, behavior: String, data: NativeParam);
    fn request_behavior(&mut self, target: Entity, behavior: String, data: String);
}

impl BehaviorExt for App {
    fn add_behavior<T: IntoSystem<BehaviorArguments, (), Marker>, Marker>(&mut self, entity_type: EntityType, name: &str, system: T) -> &mut Self {
        let mut system = IntoSystem::into_system(system);
        system.initialize(&mut self.world);

        if let Some(mut entity_type_behavior_map) = self.world.get_resource_mut::<BehaviorMap>() {
            if !entity_type_behavior_map.0.contains_key(&entity_type) {
                entity_type_behavior_map.0.insert(entity_type, HashMap::new());
            }
            
            if let Some(behaviors) = entity_type_behavior_map.0.get_mut(&entity_type) {
                behaviors.insert(name.to_owned().to_lowercase(), Box::new(system));
            }
        }
        
        self
    }

    fn tell_behavior(&mut self, instigator: Entity, target: Entity, behavior: String) {
        let mut args = behavior.split(char::is_whitespace).map(|v| v.to_owned());
        let behavior = args.next().unwrap();

        self.world.send_event(TellBehavior {
            instigator,
            target,
            behavior: Behavior::String(behavior.to_lowercase(), args.collect()),
        });
    }

    fn tell_behavior_binary(&mut self, instigator: Entity, target: Entity, behavior: String, data: NativeParam) {
        self.world.send_event(TellBehavior {
            instigator,
            target,
            behavior: Behavior::Binary(behavior.to_lowercase(), data),
        });
    }

    fn request_behavior(&mut self, entity: Entity, behavior: String, data: String) {
        self.world.send_event(RequestBehavior {
            entity,
            behavior: Behavior::String(behavior.to_lowercase(), data.split(' ').map(|v| v.to_string()).collect()),
        });
    }
}

fn perform_told_behavior(world: &mut World) {
    world.resource_scope(|world, mut events: Mut<Events<TellBehavior>>| {
        world.resource_scope(|world, mut behaviors: Mut<BehaviorMap>| {
            for ev in events.drain() {
                let entity_type = *world.get::<EntityType>(ev.target).unwrap();

                if let Some(behavior) = behaviors.0
                    .get_mut(&entity_type)
                    .and_then(|m| m.get_mut(ev.behavior.name())) {

                    behavior.run((ev.instigator, ev.target, ev.behavior), world);
                    behavior.apply_deferred(world);
                } else {
                    let avatar = world.get::<AvatarComponent>(ev.target).unwrap();
                    warn!("No behavior '{}' defined for entity {:?}:{}. But client calls for it!", ev.behavior.name(), entity_type, avatar.name)
                }
            }
        });
    });
}


fn perform_requested_behavior(world: &mut World) {
    world.resource_scope(|world, mut events: Mut<Events<RequestBehavior>>| {
        world.resource_scope(|world, mut behaviors: Mut<BehaviorMap>| {
            for ev in events.drain() {
                let entity_type = *world.get::<EntityType>(ev.entity).unwrap();

                if let Some(behavior) = behaviors.0
                    .get_mut(&entity_type)
                    .and_then(|m| m.get_mut(ev.behavior.name())) {

                    behavior.run((ev.entity, ev.entity, ev.behavior), world);
                    behavior.apply_deferred(world);
                } else {
                    let avatar = world.get::<AvatarComponent>(ev.entity).unwrap();
                    warn!("No behavior '{}' defined for entity {:?}:{}. But client requests it!", ev.behavior.name(), entity_type, avatar.name)
                }
            }
        });
    });
}
