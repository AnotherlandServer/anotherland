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

use atlas::{dialogStructure, oaDialogNode, AvatarId, CPktStream_166_2, ParamBox, PlayerComponent, PlayerParams, TriggerComponent, TriggerParams};
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs::{entity::Entity, query::{Added, With, Without}, system::{Commands, In, Query, Resource, SystemId}};

use crate::actors::{zone::{plugins::{BehaviorArguments, BehaviorExt, CompletedDialogues, DialogueCompletedEvent, PlayerController}, systems::{CombatStyleAssassin, CombatStyleCyber, CombatStyleEnergizer, CombatStyleHacker, CombatStyleNone, CombatStyleRage, CombatStyleTech}}, EntityType};

#[derive(Resource)]
pub struct TriggerScripts(HashMap<&'static str, SystemId<(Entity, Entity), ()>>);

type TriggerArguments = (Entity, Entity);

#[derive(Component)]
struct TriggerScript(SystemId<(Entity, Entity), ()>);

pub struct TriggerBehaviors;

impl Plugin for TriggerBehaviors {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut scripts = HashMap::new();
        scripts.insert("ClassClear", app.world.register_system(trigger_class_clear));
        scripts.insert("ClassWarrior", app.world.register_system(trigger_class_warrior));
        scripts.insert("ClassMarksman", app.world.register_system(trigger_class_marksman));
        scripts.insert("ClassAssassin", app.world.register_system(trigger_class_assassin));
        scripts.insert("ClassEnergizer", app.world.register_system(trigger_class_energizer));
        scripts.insert("HN01", app.world.register_system(trigger_hn01));
        scripts.insert("HN05", app.world.register_system(trigger_hn05));
        scripts.insert("HN08", app.world.register_system(trigger_hn08));
        scripts.insert("HN10", app.world.register_system(trigger_hn10));
        scripts.insert("HN14", app.world.register_system(trigger_hn14));
        scripts.insert("HN19", app.world.register_system(trigger_hn19));
        scripts.insert("HN20", app.world.register_system(trigger_hn20));
        scripts.insert("HN21", app.world.register_system(trigger_hn21));
        scripts.insert("HN22", app.world.register_system(trigger_hn22));

        app.add_behavior(EntityType::Trigger, "triggeraction", trigger_actions);
        app.insert_resource(TriggerScripts(scripts));

        app.add_systems(First, initialize_trigger);
    }
}

fn initialize_trigger(
    query: Query<(Entity, &ParamBox), Added<TriggerComponent>>,
    scripts: Res<TriggerScripts>,
    mut cmds: Commands,
) {
    for (entity, params) in query.iter() {
        let params = params.get_impl::<dyn TriggerParams>().unwrap();

        if let Some(script) = scripts.0.get(params.lua_script()) {
            cmds.entity(entity)
                .insert(TriggerScript(*script));
        } else {
            warn!("Script '{}' not implemented!", params.lua_script())
        }
    }
}

fn trigger_actions(
    In((instigator, target, _)): In<BehaviorArguments>,
    triggers: Query<(&ParamBox, Option<&TriggerScript>), With<TriggerComponent>>,
    mut cmds: Commands,
) {
    if let Ok((params, script)) = triggers.get(target) {
        if let Some(script) = script {
            debug!("Run trigger: {}", params.get_impl::<dyn TriggerParams>().unwrap().lua_script());
            cmds.run_system_with_input(script.0, (instigator, target));
        } else {
            warn!("Client triggered unimplemented script: '{}'!", params.get_impl::<dyn TriggerParams>().unwrap().lua_script());
        }
    }
}

pub fn trigger_class_clear(
    In((instigator, _)): In<TriggerArguments>,
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<TriggerComponent>)>,
    mut cmds: Commands,
) {
    if let Ok(mut player) = players.get_mut(instigator) {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();
        player.set_combat_style(6);
        cmds.entity(instigator)
            .remove::<CombatStyleRage>()
            .remove::<CombatStyleTech>()
            .remove::<CombatStyleAssassin>()
            .remove::<CombatStyleEnergizer>()
            .remove::<CombatStyleHacker>()
            .remove::<CombatStyleCyber>()
            .insert(CombatStyleNone);
    }
}

pub fn trigger_class_warrior(
    In((instigator, _)): In<TriggerArguments>,
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<TriggerComponent>)>,
    mut cmds: Commands,
) {
    if let Ok(mut player) = players.get_mut(instigator) {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_combat_style(0);
        cmds.entity(instigator).insert(CombatStyleRage);
    }
}

pub fn trigger_class_marksman(
    In((instigator, _)): In<TriggerArguments>,
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<TriggerComponent>)>,
    mut cmds: Commands,
) {
    if let Ok(mut player) = players.get_mut(instigator) {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();
        
        player.set_combat_style(1);
        cmds.entity(instigator).insert(CombatStyleTech);
    }
}

pub fn trigger_class_assassin(
    In((instigator, _)): In<TriggerArguments>,
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<TriggerComponent>)>,
    mut cmds: Commands,
) {
    if let Ok(mut player) = players.get_mut(instigator) {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();
        
        player.set_combat_style(2);
        cmds.entity(instigator).insert(CombatStyleAssassin);
    }
}

pub fn trigger_class_energizer(
    In((instigator, _)): In<TriggerArguments>,
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<TriggerComponent>)>,
    mut cmds: Commands,
) {
    if let Ok(mut player) = players.get_mut(instigator) {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();
        
        player.set_combat_style(3);
        cmds.entity(instigator).insert(CombatStyleEnergizer);
    }
}

pub fn trigger_hn01(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2804)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2804,
                dialog_node: oaDialogNode {
                    dialogue_id: 2804,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2804
        });
    }
}

pub fn trigger_hn05(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2838) &&
        !completed.is_completed(2808)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2838,
                dialog_node: oaDialogNode {
                    dialogue_id: 2838,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2808,
                dialog_node: oaDialogNode {
                    dialogue_id: 2808,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2804
        });

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2808
        });
    }
}

pub fn trigger_hn08(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2811)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2811,
                dialog_node: oaDialogNode {
                    dialogue_id: 2811,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2811
        });
    }
}

pub fn trigger_hn10(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2813)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2813,
                dialog_node: oaDialogNode {
                    dialogue_id: 2813,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2813
        });
    }
}

pub fn trigger_hn14(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2817)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2817,
                dialog_node: oaDialogNode {
                    dialogue_id: 2817,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2817
        });
    }
}

pub fn trigger_hn19(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2822)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2822,
                dialog_node: oaDialogNode {
                    dialogue_id: 2822,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2822
        });
    }
}


pub fn trigger_hn20(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2823)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2823,
                dialog_node: oaDialogNode {
                    dialogue_id: 2823,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2823
        });
    }
}

pub fn trigger_hn21(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2824)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2824,
                dialog_node: oaDialogNode {
                    dialogue_id: 2824,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2824
        });
    }
}

pub fn trigger_hn22(
    In((instigator, _)): In<TriggerArguments>,
    players: Query<(&PlayerController, &CompletedDialogues)>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
) {
    if 
        let Ok((controller, completed)) = players.get(instigator) &&
        !completed.is_completed(2825)
    {
        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                npc_id: AvatarId::default(),
                dialog_id: 2825,
                dialog_node: oaDialogNode {
                    dialogue_id: 2825,
                    dialog_content_id: 0,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                choice_count: 0,
                choices: vec![],
                ..Default::default()
            },
            ..Default::default()
        }.into_message());

        completed_event.send(DialogueCompletedEvent { 
            player: instigator,
            dialogue_id: 2825
        });
    }
}