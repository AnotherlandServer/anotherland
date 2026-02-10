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

use bevy::{ecs::{entity::Entity, system::{Commands, In, Query, Res}}, math::Vec3};
use log::{debug, error, warn};
use protocol::{OaPktRequestQuestActionKind, oaPkt_SplineSurfing_Acknowledge, oaPkt_SplineSurfing_Exit};
use regex::Regex;
use scripting::{EntityScriptCommandsExt, ScriptObject};
use toolkit::{NativeParam, types::AvatarId};

use crate::plugins::{AvatarIdManager, BinaryBehavior, PlayerController, StringBehavior};

pub(super) fn behavior_flight_tube(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    query: Query<&PlayerController>,
) {
    debug!("FlightTube beahavior: {:?}", behavior.args);

    let re = Regex::new(r"SplineID=([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}) InverseTravel=([0-1]) Loc=\[ -?(\d+\.?\d*) -?(\d+\.?\d*) -?(\d+\.?\d*) \]").unwrap();
    if let Some(captures) = re.captures(&behavior.args.join(" ")) {
        let spline_id = captures[1].parse().unwrap();
        let inverse_travel = &captures[2] == "1";
        let loc = Vec3::new(
            captures[3].parse().unwrap(), 
            captures[4].parse().unwrap(), 
            captures[5].parse().unwrap(),
        );


        if let Ok(controller) = query.get(ent) {
            controller.send_packet(
                oaPkt_SplineSurfing_Acknowledge {
                    avatar_id: controller.avatar_id(),
                    spline_id,
                    acknowledged: true,
                    inverse_travel,
                    loc: loc.into(),
                    ..Default::default()
                }
            );

            controller.send_packet(
                oaPkt_SplineSurfing_Exit {
                    avatar_id: controller.avatar_id(),
                    spline_id,
                    ..Default::default()
                }
            );
        }
    } else {
        error!("Failed to parse FlightTube behavior: {:?}", behavior.args);
    }
}

pub(super) fn behavior_loot_avatar(
    In((ent, _, behavior)): In<(Entity, Entity, BinaryBehavior)>,
    avatar_id_manager: Res<AvatarIdManager>,
    objects: Query<&ScriptObject>,
    mut commands: Commands,
) {
    if 
        let NativeParam::Struct(params) = &behavior.args &&
        let Some(NativeParam::String(target)) = params.first() &&
        let Ok(target) = target.parse() &&
        let Some(target_ent) = avatar_id_manager.resolve_avatar_id(target) &&
        let Ok(player) = objects.get(ent)
    {
        commands
            .entity(target_ent)
            .call_named_lua_method("RequestLoot", player.object().clone());
    }  
}