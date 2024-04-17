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

use atlas::{oaPktFOWUpdate, ParamBox, PlayerComponent, PlayerParams};
use bevy_ecs::{query::Added, system::{Query, Res}};
use log::warn;

use crate::actors::{AvatarComponent, FloorMapInfos};

use super::PlayerController;

pub fn initialize_fog_of_war(
    query: Query<(&AvatarComponent, &ParamBox, &PlayerController), Added<PlayerComponent>>,
    floor_maps: Res<FloorMapInfos>,
) {
    for (avatar, params, controller) in query.iter() {
        let pos = params.get_impl::<dyn PlayerParams>().unwrap().pos().1;

        if let Some(floor_map) = floor_maps.0.iter().find(|v| v.bounding_box.contains(&pos)) {
            // todo: implement actual map discovery instead of setting all bits to 1
            controller.send_message(oaPktFOWUpdate {
                zone_id: floor_map.zone_id as u32,
                bytes_per_row: floor_map.num_tiles_y,
                field_4: [u8::MAX].repeat(
                    (floor_map.num_tiles_x * floor_map.num_tiles_y) as usize
                ).into(),
                ..Default::default()
            }.into_message());
        } else {
            warn!("No floor map found for player: {}", avatar.name);
        }
    }
}