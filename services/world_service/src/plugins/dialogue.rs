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

use anyhow::anyhow;
use bevy::{app::{Plugin, PreStartup}, prelude::{App, Commands, Entity, In, Query, Res, ResMut, With, World}};
use mlua::{Lua, Table};
use obj_params::tags::PlayerTag;
use protocol::{dialogStructure, oaDialogNode, oaPktDialogList, CPktStream_166_2};
use scripting::{ApiType, LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, Scripted};
use toolkit::types::AvatarId;

use crate::error::WorldResult;

use super::{AvatarIdManager, NetworkExtPriv, PlayerController};

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, insert_dialogue_api);

        app.register_message_handler(handle_dialogue_request);
    }
}

fn insert_dialogue_api(
    world: &mut World,
) {
    let mut runtime = world.get_resource_mut::<LuaRuntime>().unwrap();
    let lua = runtime.vm().clone();
    let player = runtime.load_scripted_class(ApiType::Player, ApiType::Player.base()).unwrap().clone();

    player.set("ShowTutorialMessage", lua.create_bevy_function(world, lua_show_tutorial_message).unwrap()).unwrap();
    player.set("StartDialogue", lua.create_function(lua_start_dialogue).unwrap()).unwrap();
    player.set("FinishDialogue", lua.create_function(lua_finish_dialogue).unwrap()).unwrap();
}

fn lua_show_tutorial_message(
    In((player, tutorial_id)): In<(Table, i32)>,
    query: Query<&PlayerController>,
) -> WorldResult<()> {
    let controller = query.get(player.entity()?)
        .map_err(|_| anyhow!("player not found"))?;

    controller.send_packet(CPktStream_166_2 {
        field_1: dialogStructure {
            npc_id: AvatarId::default(), // Tutorials never have a speaker avatar
            dialog_id: tutorial_id,
            dialog_node: oaDialogNode {
                dialogue_id: tutorial_id,
                dialog_content_id: 0,
                dialogue_serial_number: "0".to_string(),
                ..Default::default()
            },
            choice_count: 0,
            choices: vec![],
            ..Default::default()
        },
        ..Default::default()
    });

    Ok(())
}

fn lua_start_dialogue(lua: &Lua, (player, speaker, dialogue): (Table, Table, i32)) -> mlua::Result<()> {
    todo!()
}

fn lua_finish_dialogue(lua: &Lua, player: Table) -> mlua::Result<()> {
    todo!()
}

fn handle_dialogue_request(
    In((ent, pkt)): In<(Entity, oaPktDialogList)>,
    avatar_id_manager: Res<AvatarIdManager>,
    query: Query<&Scripted, With<PlayerTag>>,
    mut commands: Commands,
) {
    if 
        let Some(target_ent) = avatar_id_manager.entity_from_avatar_id(pkt.target) &&
        let Ok(player) = query.get(ent)
    {
        commands.entity(target_ent)
            .call_named_lua_method("RequestDialogue", (player.script().clone()));
    }
}