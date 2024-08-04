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

use std::{str::FromStr, time::{SystemTime, UNIX_EPOCH}};

use atlas::{oaPktCombatUpdate, oaPktCooldownUpdate, oaPktLoopActionIterruptible, oaPktQuestUpdate, CooldownEntry, CooldownUpdate, NonClientBaseComponent, NonClientBaseParams, ParamBox};
use bevy::app::App;
use bevy_ecs::{query::{With, Without}, system::{In, Query}};
use log::debug;

use super::{plugins::{CommandsExt, CommandsInput, GameMessage, PlayerController}, AvatarComponent, CurrentTarget};

pub fn register_commands(app: &mut App) {
    app.add_command("avatar_info", cmd_target_info);
    app.add_command("play_animation", cmd_play_animation);
    app.add_command("update_quest", cmd_update_quest);
    app.add_command("cooldown", cmd_cooldown);
    app.add_command("toggle_combat", cmd_toggle_combat);
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

fn cmd_play_animation(
    In((entity, _, args)): In<CommandsInput>,
    players: Query<(&AvatarComponent, Option<&CurrentTarget>, &PlayerController), Without<NonClientBaseComponent>>,
    mut avatars: Query<&mut ParamBox, With<NonClientBaseComponent>>,
) {
    if let Ok((player_avatar, current_target, controller)) = players.get(entity) {
        if  let Some(CurrentTarget(target)) = current_target &&
            let Ok(mut params) = avatars.get_mut(*target) &&
            let Some(params) = params.get_impl_mut::<dyn NonClientBaseParams>()
        {
            params.set_action0((
                args.get(0).cloned().unwrap_or_default(), 
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f32()
            ));
            params.set_action0option(1);
            params.set_action0duration(4.0);
        } else {
            controller.send_message(oaPktLoopActionIterruptible {
                target: player_avatar.id,
                command: atlas::OaPktLoopActionIterruptibleCommand::PlayInterruptLoopAction,
                anim_name: args.get(0).cloned().unwrap_or_default(),
                ..Default::default()
            }.into_message());
        }
    }
}

fn cmd_update_quest(
    In((entity, _, args)): In<CommandsInput>,
    players: Query<(&AvatarComponent, &PlayerController)>,
) {
    if 
        let Ok((avatar, controller)) = players.get(entity)
    {
        controller.send_message(
            oaPktQuestUpdate {
                player: avatar.id,
                quest_id: args.get(0)
                    .and_then(|s| FromStr::from_str(s).ok())
                    .unwrap_or(0),
                quest_failed: args.get(1)
                    .and_then(|s| FromStr::from_str(s).ok())
                    .unwrap_or(false),
                entry_count: 0,
                ..Default::default()
            }.into_message()
        );
    }
}

fn cmd_toggle_combat(
    In((entity, _, args)): In<CommandsInput>,
    players: Query<(&AvatarComponent, &PlayerController)>,
) {
    if 
        let Ok((avatar, controller)) = players.get(entity) &&
        let Some(combat) = args.first().and_then(|a| FromStr::from_str(a).ok())
    {
        controller.send_message(
            oaPktCombatUpdate {
                field_1: avatar.id,
                field_2: if combat {
                    atlas::OaPktCombatUpdateField2::ToggleOnCombat
                } else {
                    atlas::OaPktCombatUpdateField2::ToggleOffCombat
                },
                ..Default::default()
            }.into_message()
        );
    }
}


/*

[3109.45] DevAbility: Ability: Recuperate, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Stance: Prime Champion, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Raw Power, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Blood Storm, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Rampage, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Bane, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Banish, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Sanguine Curse, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Stance: Prime Guardian, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Dash, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Taunt, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Vitality Slash, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Impulse, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Essence Touch, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000
[3109.45] DevAbility: Ability: Presage, OnCooldown:0, TotalCoolDown: 0.000000, Remaining cooldown: 0.000000

*/

fn cmd_cooldown(
    In((entity, _, _)): In<CommandsInput>,
    players: Query<(&AvatarComponent, &PlayerController)>,
) {
    if 
        let Ok((avatar, controller)) = players.get(entity)
    {
        controller.send_message(
            oaPktCooldownUpdate {
                avatar_id: avatar.id,
                field_2: CooldownUpdate {
                    entry_count: 15,
                    entries: vec![
                        CooldownEntry {
                            field_0: 26120,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 3887,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40679,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40677,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40678,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40675,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40683,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40680,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 3838,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40619,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40621,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40620,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40624,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40623,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        },
                        CooldownEntry {
                            field_0: 40622,
                            field_1: false,
                            field_2: 1.0,
                            field_3: 0.0,
                        }
                    ]
                },
                ..Default::default()
            }.into_message()
        );
    }
}