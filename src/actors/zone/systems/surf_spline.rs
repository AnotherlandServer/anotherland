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

use atlas::oaPkt_SplineSurfing_Exit;
use bevy_ecs::{entity::Entity, event::EventWriter, system::{Commands, Query}};
use log::debug;

use crate::actors::{zone::plugins::{PlayerController, Position}, AvatarComponent, SplineSurfing};

pub fn surf_spline(
    mut players: Query<(Entity, &AvatarComponent, &Position, &mut SplineSurfing, &PlayerController)>,
    //mut ev_avatar_event: EventWriter<AvatarEventFired>,
    mut commands: Commands,
) {
    for (entity, avatar, position, mut surfing, controller) in players.iter_mut() {
        debug!("Finish surfing spline: {:?}-{}", avatar.id, surfing.spline.id);

        controller.send_message(oaPkt_SplineSurfing_Exit {
            avatar_id: avatar.id,
            spline_id: surfing.spline.id,
            ..Default::default()
        }.into_message());

        commands.entity(entity).remove::<SplineSurfing>();
    }
} 