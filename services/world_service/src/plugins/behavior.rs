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

use std::{collections::VecDeque, str::FromStr};

use bevy::{app::Plugin, ecs::{resource::Resource, system::SystemId}, platform::collections::HashMap, prelude::{App, Commands, Entity, In, IntoSystem, Query, Res}};
use log::{debug, warn};
use mlua::{Function, IntoLua, MultiValue, Table};
use obj_params::{Class, GameObjectData};
use protocol::{oaPktAvatarTellBehavior, oaPktAvatarTellBehaviorBinary, CPktRequestAvatarBehaviors};
use scripting::{EntityScriptCommandsExt, ScriptObject};
use toolkit::NativeParam;

use crate::error::WorldError;

use super::{AvatarIdManager, Avatar, NetworkExtPriv, PlayerController};

pub struct StringBehavior {
    pub name: String,
    pub args: Vec<String>,
}

pub struct BinaryBehavior {
    pub name: String,
    pub args: NativeParam,
}

impl FromStr for StringBehavior {
    type Err = WorldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = s.split(char::is_whitespace).map(|v| v.to_owned());
        let behavior = args.next()
            .ok_or(anyhow::Error::msg("invalid behavior format"))?;

        Ok(StringBehavior {
            name: behavior.to_lowercase(),
            args: args.collect()
        })
    }
}

pub type BehaviorArguments<T> = In<(Entity, Entity, T)>;
type BehaviorSystemMap<T> = HashMap<String, SystemId<BehaviorArguments<T>>>;
type ClassBehaviorSystemMap<T> = HashMap<Class, BehaviorSystemMap<T>>;

#[derive(Resource)]
struct BehaviorMap<T: 'static>(ClassBehaviorSystemMap<T>);

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BehaviorMap(ClassBehaviorSystemMap::<StringBehavior>::new()));
        app.insert_resource(BehaviorMap(ClassBehaviorSystemMap::<BinaryBehavior>::new()));

        app.register_message_handler(handle_avatar_tell_behavior);
        app.register_message_handler(handle_avatar_tell_behavior_binary);
        app.register_message_handler(handle_avatar_request_behavior);
    }
}

#[allow(dead_code)]
pub trait BehaviorExt {
    fn register_string_behavior<T: IntoSystem<BehaviorArguments<StringBehavior>, (), Marker> + 'static, Marker>(&mut self, class: Class, name: &str, system: T) -> &mut Self;
    fn register_binary_behavior<T: IntoSystem<BehaviorArguments<BinaryBehavior>, (), Marker> + 'static, Marker>(&mut self, class: Class, name: &str, system: T) -> &mut Self;
}

impl BehaviorExt for App {
    fn register_string_behavior<T: IntoSystem<BehaviorArguments<StringBehavior>, (), Marker> + 'static, Marker>(&mut self, class: Class, name: &str, system: T) -> &mut Self {
        let system = self.world_mut().register_system(system);

        let mut map = self.world_mut().get_resource_mut::<BehaviorMap<StringBehavior>>()
            .unwrap();

        if !map.0.contains_key(&class) {
            map.0.insert(class, HashMap::new());
        }
        
        if let Some(behaviors) = map.0.get_mut(&class) {
            behaviors.insert(name.to_owned().to_lowercase(), system);
        }
        
        self
    }

    fn register_binary_behavior<T: IntoSystem<BehaviorArguments<BinaryBehavior>, (), Marker> + 'static, Marker>(&mut self, class: Class, name: &str, system: T) -> &mut Self {
        let system = self.world_mut().register_system(system);

        let mut map = self.world_mut().get_resource_mut::<BehaviorMap<BinaryBehavior>>()
            .unwrap();

        if !map.0.contains_key(&class) {
            map.0.insert(class, HashMap::new());
        }
        
        if let Some(behaviors) = map.0.get_mut(&class) {
            behaviors.insert(name.to_owned().to_lowercase(), system);
        }
        
        self
    }
}

fn handle_avatar_tell_behavior(
    In((ent, pkt)): In<(Entity, oaPktAvatarTellBehavior)>,
    instigator: Query<(Entity, &PlayerController, Option<&ScriptObject>)>,
    target: Query<(&Avatar, &GameObjectData, Option<&ScriptObject>)>,
    behaviors: Res<BehaviorMap<StringBehavior>>,
    avatars: Res<AvatarIdManager>,
    mut commands: Commands
) {
    if 
        let Ok((instigator_ent, controller, instigator_script)) = instigator.get(ent) &&
        let Some(target_ent) = avatars.resolve_avatar_id(pkt.target) &&
        let Ok((target_info, target, target_script)) = target.get(target_ent) &&
        let Ok(behavior) = pkt.behavior.parse::<StringBehavior>()
    {
        if controller.avatar_id() != pkt.instigator {
            warn!("Avatar {} tried to instigate behavior on behalf of {}", controller.avatar_id(), pkt.instigator);
            return;
        }

        if let Some(system) = behaviors.0
            .get(&target.class())
            .and_then(|m| m.get(&behavior.name))
        {
            commands.run_system_with(*system, (instigator_ent, target_ent, behavior));
        } else if 
            let Some(instigator) = instigator_script &&
            let Some(target_script) = target_script &&
            let Ok(behaviors) = target_script.object().get::<Table>("_BEHAVIOR") &&
            let Ok(behavior_func) = behaviors.get::<Function>(behavior.name.as_str())
        {
            let mut args = MultiValue::new();
            args.push_back(instigator.object().into_lua(instigator.vm()).unwrap());
            args.append(&mut behavior.args.into_iter()
                .map(|v| v.into_lua(target_script.vm()).unwrap())
                .collect()
            );

            commands.entity(target_ent)
                .call_lua_method(behavior_func, args);
        } else {
            warn!("No behavior '{}' defined for entity {:?}:{}. Args: {:?}", behavior.name, target.class().name(), target_info.name, behavior.args)
        }
    }
}

fn handle_avatar_tell_behavior_binary(
    In((ent, pkt)): In<(Entity, oaPktAvatarTellBehaviorBinary)>,
    instigator: Query<(Entity, &PlayerController, Option<&ScriptObject>)>,
    target: Query<(&Avatar, &GameObjectData, Option<&ScriptObject>)>,
    behaviors: Res<BehaviorMap<BinaryBehavior>>,
    avatars: Res<AvatarIdManager>,
    mut commands: Commands
) {
    if 
        let Ok((instigator_ent, controller, instigator_script)) = instigator.get(ent) &&
        let Some(target_ent) = avatars.resolve_avatar_id(pkt.target) &&
        let Ok((target_info, target, target_script)) = target.get(target_ent)
    {
        let behavior = BinaryBehavior {
            name: pkt.behavior.to_lowercase(),
            args: pkt.data,
        };

        if controller.avatar_id() != pkt.instigator {
            warn!("Avatar {} tried to instigate binary behavior on behalf of {}", controller.avatar_id(), pkt.instigator);
            return;
        }

        if let Some(system) = behaviors.0
            .get(&target.class())
            .and_then(|m| m.get(&behavior.name))
        {
            commands.run_system_with(*system, (instigator_ent, target_ent, behavior));
        } else if 
            let Some(instigator_script) = instigator_script &&
            let Some(target_script) = target_script &&
            let Ok(behaviors) = target_script.object().get::<Table>("_BEHAVIOR") &&
            let Ok(behavior_func) = behaviors.get::<Function>(behavior.name.as_str())
        {
            let mut args = MultiValue::new();
            args.push_back(instigator_script.object().into_lua(instigator_script.vm()).unwrap());

            if let NativeParam::Struct(vec) = behavior.args {
                args.append(
                    &mut vec.into_iter()
                        .map(|v| v.into_lua(target_script.vm()))
                        .collect::<mlua::Result<VecDeque<_>>>().unwrap()
                );
            } else {
                args.push_back(behavior.args.into_lua(target_script.vm()).unwrap());
            }

            commands.entity(target_ent)
                .call_lua_method(behavior_func, args);
        } else {
            warn!("No binary behavior '{}' defined for entity {:?}:{}. Args: {:#?}", behavior.name, target.class().name(), target_info.name, behavior.args)
        }
    }
}

fn handle_avatar_request_behavior(
    In((ent, pkt)): In<(Entity, CPktRequestAvatarBehaviors)>,
    instigator: Query<(Entity, &PlayerController, Option<&ScriptObject>)>,
    target: Query<(&Avatar, &GameObjectData, Option<&ScriptObject>)>,
    behaviors: Res<BehaviorMap<StringBehavior>>,
    avatars: Res<AvatarIdManager>,
    mut commands: Commands
) {
    debug!("Request behavior: {} Data: {}", pkt.behaviour, pkt.data);

    if 
        let Ok((instigator_ent, controller, instigator_script)) = instigator.get(ent) &&
        let Some(target_ent) = avatars.resolve_avatar_id(pkt.avatar_id) &&
        let Ok((target_info, target, target_script)) = target.get(target_ent) &&
        let Ok(behavior) = format!("{} {}", pkt.behaviour, pkt.data).parse::<StringBehavior>()
    {
        if controller.avatar_id() != pkt.avatar_id {
            warn!("Avatar {} tried to instigate behavior on behalf of {}", controller.avatar_id(), pkt.avatar_id);
            return;
        }

        if let Some(system) = behaviors.0
            .get(&target.class())
            .and_then(|m| m.get(&behavior.name))
        {
            commands.run_system_with(*system, (instigator_ent, target_ent, behavior));
        } else if 
            let Some(instigator) = instigator_script &&
            let Some(target_script) = target_script &&
            let Ok(behaviors) = target_script.object().get::<Table>("_BEHAVIOR") &&
            let Ok(behavior_func) = behaviors.get::<Function>(behavior.name.as_str())
        {
            let mut args = MultiValue::new();
            args.push_back(instigator.object().into_lua(instigator.vm()).unwrap());
            args.append(&mut behavior.args.into_iter()
                .map(|v| v.into_lua(target_script.vm()).unwrap())
                .collect()
            );

            commands.entity(target_ent)
                .call_lua_method(behavior_func, args);
        } else {
            warn!("No behavior '{}' defined for entity {:?}:{}. Args: {:?}", behavior.name, target.class().name(), target_info.name, behavior.args)
        }
    }
}
