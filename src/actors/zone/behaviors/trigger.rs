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

use atlas::{dialogStructure, oaDialogNode, CPktStream_166_2, ClassSkill, ClassSkills, HeavyData, HeavyDataEntry, NonClientBaseParams, Param, ParamBox, PlayerComponent, PlayerParams, TriggerComponent, TriggerParams, Uuid, UUID_NIL};
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs::{entity::Entity, query::{Added, With, Without}, system::{Commands, In, Query, Resource, SystemId}};
use bitstream_io::{ByteWrite, ByteWriter, LittleEndian};
use log::{error, info, warn};

use crate::{actors::{zone::{plugins::{AwardStartEquipmentTransaction, BehaviorArguments, BehaviorExt, InventoryTab, PlayerController, PlayerDisguise, PlayerInventory, PlayerLoadout, RemovalPending}, systems::{CombatStyleAssassin, CombatStyleCyber, CombatStyleEnergizer, CombatStyleHacker, CombatStyleNone, CombatStyleRage, CombatStyleTech}}, EntityType}, db::get_cached_item};

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
    players: Query<&PlayerController>,
) {
    if let Ok(controller) = players.get(instigator) {
        debug!("Start tutorial");

        controller.send_message(CPktStream_166_2 {
            field_1: dialogStructure {
                dialog_node: oaDialogNode {
                    dialogue_id: 2804,
                    dialogue_serial_number: "0".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }.into_message());
    }
}