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

use bevy::app::App;
use bevy_ecs::system::{In, Query};

use super::{plugins::{CommandsExt, CommandsInput, GameMessage, PlayerController}, AvatarComponent, CurrentTarget};

pub fn register_commands(app: &mut App) {
    app.add_command("avatar_info", cmd_target_info);
}

fn cmd_target_info(
    In((entity, _, _)): In<CommandsInput>,
    players: Query<(&CurrentTarget, &PlayerController)>,
    avatars: Query<&AvatarComponent>,
) {
    if 
        let Ok((CurrentTarget(target), controller)) = players.get(entity) &&
        let Ok(avatar_info) = avatars.get(*target)
    {
        controller.send_game_message(GameMessage::Normal(format!(
            "--------------------------\n\
             > AvatarID: {}\n\
             > Name: {}\n\
             > InstanceID: {}\n\
             > RecordID: {}\n\
             --------------------------",
             avatar_info.id,
             avatar_info.name,
             avatar_info.instance_id.unwrap_or_default(),
             avatar_info.record_id.unwrap_or_default(),
        )));
    }
}