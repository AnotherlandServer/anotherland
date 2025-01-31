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

use bevy::{app::{App, Plugin}, ecs::system::SystemId, prelude::{Commands, Entity, In, IntoSystem, Query, Res, Resource}, utils::HashMap};
use clap::Parser;
use derive_builder::Builder;
use log::warn;
use protocol::oaPktCheatingClusterNode;
use toolkit::NativeParam;

use crate::error::WorldResult;

use super::{MessageType, NetworkExtPriv, PlayerController};

#[derive(Resource, Default)]
struct CommandHandlers(HashMap<String, SystemId<CommandInput>>);

type CommandInput = In<(Entity, Vec<NativeParam>)>;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandHandlers>();

        app.register_message_handler(handle_command_request);
    }
}

pub trait CommandExtPriv {
    fn register_command<T: IntoSystem<CommandInput, (), Marker> + 'static, Marker>(&mut self, name: &str, system: T);
}

impl CommandExtPriv for App {
    fn register_command<T: IntoSystem<CommandInput, (), Marker> + 'static, Marker>(&mut self, name: &str, system: T) {
        let system = self.world_mut().register_system(system);

        self.world_mut()
            .resource_mut::<CommandHandlers>()
            .0
            .insert(name.to_owned(), system);
    }
}

fn handle_command_request(
    In((ent, pkt)): In<(Entity, oaPktCheatingClusterNode)>,
    cmd_handlers: Res<CommandHandlers>,
    mut commands: Commands,
) {
    if let NativeParam::Struct(args) = pkt.command {
        let mut iter = args.into_iter();

        if 
            let Some(_node) = iter.next() &&
            let Some(NativeParam::String(cmd)) = iter.next()
        {
            if let Some(system) = cmd_handlers.0.get(&cmd) {
                commands.run_system_with_input(*system, (ent, iter.collect()));
            } else {
                warn!("Unknown command: {}", cmd);
            }
        }
    } else {
        warn!("Invalid request format: {:?}", pkt.command);
    }
}