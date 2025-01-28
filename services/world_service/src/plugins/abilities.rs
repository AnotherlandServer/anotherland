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

use bevy::{app::{App, Plugin}, prelude::{Entity, In, Query}};
use log::debug;
use protocol::{oaPktAbilityRequest, oaPktAbilityUse, OaPktAbilityUseAbilityType, OaPktAbilityUseEventType};
use toolkit::types::Uuid;

use super::{Interests, Movement, NetworkExtPriv, PlayerController};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_ability_request);
    }
}

/*

oaPktAbilityRequest {
        field8_0x8: 0,
        instigator: AvatarId(
            (
                106357916860630272,
                Player,
            ),
        ),
        item_id: a301ae36-0fe3-4336-9b73-0d497c179866,
        field_3: 133961,
        flag: 29,
        skill_id: Some(
            a301ae36-0fe3-4336-9b73-0d497c179866,
        ),
        target_info: None,
        toggle_mode: Some(
            1,
        ),
        field_8: Some(
            0,
        ),
        target_rotation: Some(
            NetworkVec4 {
                x: 0.5385568,
                y: 0.0,
                z: -0.0,
                w: 0.8425892,
            },
        ),
        field_10: None,
    }

*/

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    player: Query<(&PlayerController, &Interests)>,
    pos: Query<&Movement>,
) {
    
    debug!("Ability request: Avatar {} ToggleMode {:?} Skill {:?} Prediction {} Combo {:?}", pkt.caster, pkt.toggle_mode, pkt.skill_id, pkt.prediction_id, pkt.combo_stage_id);
    debug!("{:#?}", pkt);

    let invoke_location = pos.get(ent)
        .map(|pos| pos.position)
        .unwrap_or_default();

    if pkt.toggle_mode == Some(1) || pkt.toggle_mode.is_none() {
        for (controller, interests) in player.iter() {
            if controller.avatar_id() == pkt.caster {
                controller.send_packet(oaPktAbilityUse {
                    player: controller.avatar_id(),
                    caster: pkt.caster,
                    ability_id: "d5bfa0b8-a6df-45ca-a810-b9c29bcf32f3".parse().unwrap(),
                    buff_id: "3846d61b-2428-4d2c-88a5-9b17ccbfee8a".parse().unwrap(),
                    prediction_id: pkt.prediction_id,
                    event_type: OaPktAbilityUseEventType::Use,
                    combo_stage_id: pkt.combo_stage_id.unwrap_or_default() as i32,
                    ability_invoke_location: invoke_location.into(),
                    ability_type: OaPktAbilityUseAbilityType::Item,
                    server_event_duration: 1.0,
                    target: Some(pkt.caster.as_u64()),
                    ..Default::default()
                });
            } else if interests.contains(&ent) {
                controller.send_packet(oaPktAbilityUse {
                    player: controller.avatar_id(),
                    caster: pkt.caster,
                    ability_id: "d5bfa0b8-a6df-45ca-a810-b9c29bcf32f3".parse().unwrap(),
                    buff_id: "3846d61b-2428-4d2c-88a5-9b17ccbfee8a".parse().unwrap(),
                    event_type: OaPktAbilityUseEventType::Use,
                    combo_stage_id: pkt.combo_stage_id.unwrap_or_default() as i32,
                    ability_invoke_location: invoke_location.into(),
                    ability_type: OaPktAbilityUseAbilityType::Item,
                    ..Default::default()
                });
            }
        }
    }
}