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

use atlas::oaPktMoveManagerPosUpdate;
use bevy_ecs::{query::Changed, system::Query};

use crate::actors::{zone::plugins::Position, AvatarComponent, InterestList};

use super::PlayerController;

pub fn send_position_updates(
    positions: Query<(&AvatarComponent, &Position), Changed<Position>>,
    players: Query<(&InterestList, &PlayerController)>,
) {
    for (avatar, pos) in positions.iter() {
        // check player interest list to dispatch updates
        for (interests, controller) in players.iter() {
            if interests.contains(avatar.id) {
                controller.send_message(oaPktMoveManagerPosUpdate {
                    avatar_id: avatar.id.as_u64(),
                    pos: pos.position.into(),
                    rot: pos.rotation.into(),
                    vel: pos.velocity.into(),
                    physics: pos.physics_state.into(),
                    mover_key: pos.mover_key,
                    seconds: pos.seconds,
                    ..Default::default()
                }.into_message());
            }
        }
    }
}