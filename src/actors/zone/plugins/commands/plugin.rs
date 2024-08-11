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

use bevy::{app::{App, Plugin, PreUpdate}, utils::HashMap};
use bevy_ecs::{entity::Entity, event::{Event, Events}, system::{IntoSystem, Resource, RunSystemOnce, System, SystemId}, world::{Mut, World}};
use regex::Regex;

use crate::actors::zone::plugins::{GameMessage, PlayerController};

pub type CommandsInput = (Entity, String, Vec<String>);

#[derive(Resource)]
struct CommandsMap(HashMap<String, SystemId<CommandsInput, ()>>);

#[derive(Event)]
pub struct ExecuteCommand {
    pub player: Entity,
    pub name: String, 
    pub arguments: Vec<String>,
}

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ExecuteCommand>();

        app.world.insert_resource(CommandsMap(HashMap::new()));

        app.add_systems(PreUpdate, run_commands);
    }
}

pub trait CommandsExt {
    fn add_command<T: IntoSystem<CommandsInput, (), Marker> + 'static, Marker>(&mut self, name: &str, system: T);
    fn execute_command(&mut self, entity: Entity, command: &str);
}

impl CommandsExt for App {
    fn add_command<T: IntoSystem<CommandsInput, (), Marker> + 'static, Marker>(&mut self, name: &str, system: T) {
        let system_id = self.world.register_system(system);

        if let Some(mut commands_map) = self.world.get_resource_mut::<CommandsMap>() {
            commands_map.0.insert(name.to_string(), system_id);
        }
    }

    fn execute_command(&mut self, entity: Entity, command: &str) {
        let re = Regex::new(r"--(\w+) *(.*)").unwrap();
        if 
            let Some(captures) = re.captures(command) &&
            let Some(name) = captures.get(1) 
        {
            let arguments = if let Some(args) = captures.get(2) {
                args.as_str()
                    .split(" ")
                    .map(|v| v.to_owned())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            self.world.send_event(ExecuteCommand {
                player: entity,
                name: name.as_str().to_string(),
                arguments,
            });
        }
    }
}

fn run_commands(world: &mut World) {
    world.resource_scope(|world, mut events: Mut<Events<ExecuteCommand>>| {
        world.resource_scope(|world, commands: Mut<CommandsMap>| {
            for ExecuteCommand { player, name, arguments } in events.drain() {
                if let Some(command) = commands.0.get(&name) {
                    let _ = world.run_system_with_input(*command, (player, name, arguments));
                } else if let Some(controller) = world.get::<PlayerController>(player) {
                    controller.send_game_message(GameMessage::Normal("Command not found!".to_string()));
                }
            }
        });
    });
}