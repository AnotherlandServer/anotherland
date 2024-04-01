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

use atlas::{ParamBox, ParamClass, PlayerClass, PlayerComponent, PlayerParams};
use bevy::{app::Plugin, prelude::App};
use bevy_ecs::{query::With, system::{In, Query}};
use serde_json::json;

use crate::actors::{zone::plugins::{SubjectivityExt, SubjectivityLensArguments}, EntityType};

pub struct SubjectivePlayers;

impl Plugin for SubjectivePlayers {
    fn build(&self, app: &mut App) {
        app.add_subjective_lens(EntityType::Player, player_lens);
    }
}

fn player_lens(
    In((player_id, other_player_id)): In<SubjectivityLensArguments>,
    players: Query<&ParamBox, With<PlayerComponent>>,
) -> ParamBox {
    // are we looking at ourself?
    if player_id == other_player_id {
        let mut player = players
            .get(player_id)
            .unwrap()
            .clone();

        player
    } else {
        let player = players.get(player_id).unwrap();
        player.clone()
    }
}