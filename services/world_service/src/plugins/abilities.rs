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

use bevy::{app::{App, Plugin}, ecs::{event::EventWriter, query::With, system::Res}, prelude::{Entity, In, Query}};
use log::debug;
use obj_params::{tags::ItemBaseTag, GameObjectData};
use protocol::oaPktAbilityRequest;

use super::{AvatarIdManager, HealthUpdateEvent, Interests, Inventory, Movement, NetworkExtPriv, PlayerController};

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
    In((_ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    _avatar_man: Res<AvatarIdManager>,
    _player: Query<(&PlayerController, &Interests, &Movement, &GameObjectData, &Inventory)>,
    _items: Query<&GameObjectData, With<ItemBaseTag>>,
    _targets: Query<(&GameObjectData, &Movement)>,
    mut _health_update: EventWriter<HealthUpdateEvent>,
) {
    
    debug!("Ability request: Avatar {} ToggleMode {:?} Skill {:?} Prediction {} Combo {:?}", pkt.caster, pkt.toggle_mode, pkt.skill_id, pkt.prediction_id, pkt.combo_stage_id);
    debug!("{:#?}", pkt);



    /*let invoke_location = pos.get(ent)
        .map(|pos| pos.position)
        .unwrap_or_default();

    if pkt.toggle_mode == Some(1) || pkt.toggle_mode.is_none() {
        // Incredibly quick and dirty damage logic
        let target_avatar = pkt.params.and_then(|s| s.parse::<AvatarId>().ok());/*player.get(ent).ok()
            .and_then(|(_, _, _, data)| data.get::<_, AvatarId>(Player::Target).ok())
            .cloned();*/

        let effects = if 
            let Some(target_avatar) = target_avatar &&
            let Some(target_ent) = avatar_man.entity_from_avatar_id(target_avatar)
        {
            let id = HealthUpdateEvent::damage(target_ent, 100)
                .send(&mut health_update);

            vec![AbilityEffect {
                effect_type: 1,
                target_actor: target_avatar,
                flags: 72,
                total_damage_or_heal_amount: Some(100.0),
                //effect_delay: Some(0.5),
                //effect_duration_from_server: Some(1.0),
                delta_hp_id: Some(id),
                ..Default::default()
            }]
        } else {
            vec![]
        };

        debug!("Effects: {:#?}", effects);

        for (controller, interests, movement, _) in player.iter() {
            if controller.avatar_id() == pkt.caster {
                debug!("Send ability use");

                controller.send_packet(oaPktAbilityUse {
                    player: controller.avatar_id(),
                    caster: pkt.caster,
                    ability_id: "a7e96bda-e2c1-44e1-ad88-ecdc08c271a4".parse().unwrap(),
                    buff_id: "b6ece40e-dd0e-4748-9dbf-69797e123be1".parse().unwrap(),
                    prediction_id: pkt.prediction_id,
                    event_type: OaPktAbilityUseEventType::Charge,
                    combo_stage_id: pkt.combo_stage_id.unwrap_or_default() as i32,
                    //ability_invoke_location: invoke_location.into(),
                    ability_type: OaPktAbilityUseAbilityType::Item,
                    server_event_duration: 1.7,
                    flag: if pkt.target_rotation.is_some() { 2 } else { 0 },
                    rotation: pkt.target_rotation.clone(),
                    effect_count: effects.len() as u32,
                    effects: effects.clone(),
                    //effect_count: 1,
                    /*effects: vec![AbilityEffect {
                        effect_type: 0x0,
                        //target_actor: pkt.caster,
                        flags: 48,
                        effect_delay: Some(0.5),
                        effect_duration_from_server: Some(1.0),
                        ..Default::default()
                    }],*/
                    //flag: 6,
                    //target_hit_location: Some(invoke_location.into()),
                    //rotation: Some(movement.rotation.into()),

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
    }*/
}

/*fn handle_ability_request(
    In((ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    avatar_man: Res<AvatarIdManager>,
    player: Query<(&PlayerController, &Interests, &Movement, &GameObjectData)>,
    pos: Query<&Movement>,
    mut health_update: EventWriter<HealthUpdateEvent>,
) {
    
    debug!("Ability request: Avatar {} ToggleMode {:?} Skill {:?} Prediction {} Combo {:?}", pkt.caster, pkt.toggle_mode, pkt.skill_id, pkt.prediction_id, pkt.combo_stage_id);
    //debug!("{:#?}", pkt);

    let invoke_location = pos.get(ent)
        .map(|pos| pos.position)
        .unwrap_or_default();

    if pkt.toggle_mode == Some(1) || pkt.toggle_mode.is_none() {
        // Incredibly quick and dirty damage logic
        let target_avatar = pkt.params.and_then(|s| s.parse::<AvatarId>().ok());/*player.get(ent).ok()
            .and_then(|(_, _, _, data)| data.get::<_, AvatarId>(Player::Target).ok())
            .cloned();*/

        let effects = if 
            let Some(target_avatar) = target_avatar &&
            let Some(target_ent) = avatar_man.entity_from_avatar_id(target_avatar)
        {
            let id = HealthUpdateEvent::damage(target_ent, 100)
                .send(&mut health_update);

            vec![AbilityEffect {
                effect_type: 1,
                target_actor: target_avatar,
                flags: 72,
                total_damage_or_heal_amount: Some(100.0),
                //effect_delay: Some(0.5),
                //effect_duration_from_server: Some(1.0),
                delta_hp_id: Some(id),
                ..Default::default()
            }]
        } else {
            vec![]
        };

        debug!("Effects: {:#?}", effects);

        for (controller, interests, movement, _) in player.iter() {
            if controller.avatar_id() == pkt.caster {
                debug!("Send ability use");

                controller.send_packet(oaPktAbilityUse {
                    player: controller.avatar_id(),
                    caster: pkt.caster,
                    ability_id: "a7e96bda-e2c1-44e1-ad88-ecdc08c271a4".parse().unwrap(),
                    buff_id: "b6ece40e-dd0e-4748-9dbf-69797e123be1".parse().unwrap(),
                    prediction_id: pkt.prediction_id,
                    event_type: OaPktAbilityUseEventType::Charge,
                    combo_stage_id: pkt.combo_stage_id.unwrap_or_default() as i32,
                    //ability_invoke_location: invoke_location.into(),
                    ability_type: OaPktAbilityUseAbilityType::Item,
                    server_event_duration: 1.7,
                    flag: if pkt.target_rotation.is_some() { 2 } else { 0 },
                    rotation: pkt.target_rotation.clone(),
                    effect_count: effects.len() as u32,
                    effects: effects.clone(),
                    //effect_count: 1,
                    /*effects: vec![AbilityEffect {
                        effect_type: 0x0,
                        //target_actor: pkt.caster,
                        flags: 48,
                        effect_delay: Some(0.5),
                        effect_duration_from_server: Some(1.0),
                        ..Default::default()
                    }],*/
                    //flag: 6,
                    //target_hit_location: Some(invoke_location.into()),
                    //rotation: Some(movement.rotation.into()),

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
                    server_event_duration: 1.0,
                    target: Some(pkt.caster),
                    ..Default::default()
                });
            }
        }
    }
}*/

/*

fn handle_ability_request(
    In((ent, pkt)): In<(Entity, oaPktAbilityRequest)>,
    avatar_man: Res<AvatarIdManager>,
    player: Query<(&PlayerController, &Interests, &Movement, &GameObjectData)>,
    mut target: Query<&mut Health>,
    pos: Query<&Movement>,
) {
    
    debug!("Ability request: Avatar {} ToggleMode {:?} Skill {:?} Prediction {} Combo {:?}", pkt.caster, pkt.toggle_mode, pkt.skill_id, pkt.prediction_id, pkt.combo_stage_id);
    //debug!("{:#?}", pkt);

    let invoke_location = pos.get(ent)
        .map(|pos| pos.position)
        .unwrap_or_default();

    if pkt.toggle_mode == Some(1) || pkt.toggle_mode.is_none() {
        // Incredibly quick and dirty damage logic
        let target_avatar = pkt.params.and_then(|s| s.parse::<AvatarId>().ok());/*player.get(ent).ok()
            .and_then(|(_, _, _, data)| data.get::<_, AvatarId>(Player::Target).ok())
            .cloned();*/

        let dmg = if let Some(target_ent) = target_avatar.and_then(|target| avatar_man.entity_from_avatar_id(target)) {
            if let Ok(mut target) = target.get_mut(target_ent) {
                target.current -= 100;
                if target.current < target.min {
                    target.current = target.min;
                }

                Some((target.current, 100))
            } else {
                None
            }
        } else {
            None
        };

        let (hp_updated, effects) = if 
            let Some((hp, dmg)) = dmg &&
            let Some(target_avatar) = target_avatar
        {
            let id = random();

            (
                Some(oaPkt_Combat_HpUpdate {
                    avatar_id: target_avatar,
                    hp,
                    id,
                    ..Default::default()
                }),

                vec![AbilityEffect {
                    effect_type: 1,
                    target_actor: target_avatar,
                    flags: 72,
                    total_damage_or_heal_amount: Some(dmg as f32),
                    //effect_delay: Some(0.5),
                    //effect_duration_from_server: Some(1.0),
                    delta_hp_id: Some(id as i32),
                    ..Default::default()
                }]
            )
        } else {
            (None, vec![])
        };

        debug!("Effects: {:#?}", effects);

        for (controller, interests, movement, _) in player.iter() {
            if let Some(hp_updated) = &hp_updated {
                controller.send_packet(hp_updated.clone());
            }

            if controller.avatar_id() == pkt.caster {
                debug!("Send ability use");

                controller.send_packet(oaPktAbilityUse {
                    player: controller.avatar_id(),
                    caster: pkt.caster,
                    ability_id: "d5bfa0b8-a6df-45ca-a810-b9c29bcf32f3".parse().unwrap(),
                    buff_id: "ac81b7d5-8034-430a-9873-7b5c569abd37".parse().unwrap(),
                    prediction_id: pkt.prediction_id,
                    event_type: OaPktAbilityUseEventType::Charge,
                    combo_stage_id: pkt.combo_stage_id.unwrap_or_default() as i32,
                    //ability_invoke_location: invoke_location.into(),
                    ability_type: OaPktAbilityUseAbilityType::Item,
                    server_event_duration: 1.7,
                    flag: if pkt.target_rotation.is_some() { 2 } else { 0 },
                    rotation: pkt.target_rotation.clone(),
                    effect_count: effects.len() as u32,
                    effects: effects.clone(),
                    //effect_count: 1,
                    /*effects: vec![AbilityEffect {
                        effect_type: 0x0,
                        //target_actor: pkt.caster,
                        flags: 48,
                        effect_delay: Some(0.5),
                        effect_duration_from_server: Some(1.0),
                        ..Default::default()
                    }],*/
                    //flag: 6,
                    //target_hit_location: Some(invoke_location.into()),
                    //rotation: Some(movement.rotation.into()),

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

*/