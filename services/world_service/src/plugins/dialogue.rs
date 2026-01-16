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

use anyhow::anyhow;
use bevy::{app::{Plugin, PreStartup, Update}, ecs::{component::Component, message::{Message, MessageReader}}, prelude::{App, Commands, Entity, In, Query, Res, With, World}};
use log::debug;
use mlua::{FromLua, Lua, Table};
use obj_params::tags::{NpcOtherlandTag, PlayerTag};
use protocol::{oaDialogNode, oaDialogQuestPrototype, oaPktDialogChoice, oaPktDialogEnd, oaPktDialogList, CPktStream_166_2, DialogStructure};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, EntityScriptCommandsExt, ScriptObject};
use toolkit::types::AvatarId;

use crate::{error::WorldResult, plugins::{Avatar, ReturnQuest}};

use super::{AvatarIdManager, NetworkExtPriv, PlayerController};

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, insert_dialogue_api);
        app.add_systems(Update, (send_dialogue_nodes, send_dialogue_end));

        app.register_message_handler(handle_dialogue_request);
        app.register_message_handler(handle_dialogue_choice);

        app.add_message::<DialogueNodeSelected>();
        app.add_message::<DialogueEnd>();
    }
}

#[derive(Message)]
pub struct DialogueNodeSelected {
    pub player: Entity,
    pub speaker: Entity,
    pub id: i32,
    pub index: usize,
}

#[derive(Message)]
pub struct DialogueEnd {
    pub player: Entity,
}

pub enum ChoiceEmote {
    Close,
    Approve,
    Reject,
    Next,
    TellMore,
}

impl FromLua for ChoiceEmote {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        let s: String = FromLua::from_lua(value, lua)?;
        match s.as_str() {
            "Close" => Ok(ChoiceEmote::Close),
            "Approve" => Ok(ChoiceEmote::Approve),
            "Reject" => Ok(ChoiceEmote::Reject),
            "Next" => Ok(ChoiceEmote::Next),
            "TellMore" => Ok(ChoiceEmote::TellMore),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "ChoiceEmote".to_string(),
                message: Some(format!("invalid dialogue choice: {}", s)),
            }),
        }
    }
}

pub struct DialogueChoice {
    pub choice_emote: ChoiceEmote,
    pub next_index: Option<usize>,
}

impl FromLua for DialogueChoice {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        let table: Table = FromLua::from_lua(value, lua)?;
        let choice_emote: ChoiceEmote = table.get("choice_emote")?;
        let next_index: Option<usize> = table.get("next_index").ok();
        Ok(DialogueChoice {
            choice_emote,
            next_index,
        })
    }
}

pub struct DialogueNode {
    pub content_id: u32,
    pub quest_id: Option<i32>,
    pub choices: Vec<DialogueChoice>,
}

impl FromLua for DialogueNode {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        let table: Table = FromLua::from_lua(value, lua)?;

        let content_id: u32 = table.get("content_id")?;
        let quest_id: Option<i32> = table.get("quest_id").ok();
        let choices_table: Table = table.get("choices")?;
        let mut choices = Vec::new();
        for pair in choices_table.pairs::<mlua::Value, mlua::Value>() {
            let (_, choice_value) = pair?;
            let choice: DialogueChoice = FromLua::from_lua(choice_value, lua)?;
            choices.push(choice);
        }

        Ok(DialogueNode {
            content_id,
            quest_id,
            choices,
        })
    }
}

#[derive(Component)]
pub struct DialogueState {
    pub quest_id: i32,
    pub speaker: Entity,
    pub current_index: usize,
    pub quest_finisher: bool,
    pub nodes: Vec<DialogueNode>,
}

fn insert_dialogue_api(
    world: &mut World,
) {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let dialogue_api = lua.create_table().unwrap();
    runtime.register_native("dialogue", dialogue_api.clone()).unwrap();

    dialogue_api.set("ShowTutorialMessage", lua.create_bevy_function(world, lua_show_tutorial_message).unwrap()).unwrap();
    dialogue_api.set("ExecuteDialogue", lua.create_bevy_function(world, lua_exec_dialogue).unwrap()).unwrap();
    dialogue_api.set("AbortDialogue", lua.create_bevy_function(world, lua_abort_dialogue).unwrap()).unwrap();

}

fn lua_show_tutorial_message(
    In((player, tutorial_id)): In<(Table, i32)>,
    query: Query<&PlayerController>,
) -> WorldResult<()> {
    let controller = query.get(player.entity()?)
        .map_err(|_| anyhow!("player not found"))?;

    controller.send_packet(CPktStream_166_2 {
        field_1: DialogStructure {
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

fn lua_exec_dialogue(
    In((player, speaker, quest_id, finish_quest, dialogue)): In<(Table, Table, i32, bool, Table)>,
    runtime: Res<LuaRuntime>,
    mut commands: Commands,
) -> mlua::Result<()> {
    commands.entity(player.entity()?)
        .insert(DialogueState {
            current_index: 0,
            quest_id,
            speaker: speaker.entity()?,
            quest_finisher: finish_quest,
            nodes: {
                let mut nodes = Vec::new();
                for pair in dialogue.pairs::<mlua::Value, mlua::Value>() {
                    let (_, node_value) = pair?;
                    let node: DialogueNode = FromLua::from_lua(node_value, runtime.vm())?;
                    nodes.push(node);
                }
                nodes
            },
        });

    commands.write_message(DialogueNodeSelected {
        player: player.entity()?,
        speaker: speaker.entity()?,
        id: quest_id,
        index: 1,
    });

    Ok(())
}

fn lua_abort_dialogue(
    In(player): In<Table>,
    //players: Query<(&DialogueState, &PlayerController)>,
    mut commands: Commands,
) -> mlua::Result<()> {
    commands.write_message(DialogueEnd {
        player: player.entity()?
    });

    Ok(())
}

fn handle_dialogue_request(
    In((ent, pkt)): In<(Entity, oaPktDialogList)>,
    avatar_id_manager: Res<AvatarIdManager>,
    query: Query<&ScriptObject, With<PlayerTag>>,
    mut commands: Commands,
) {
    if 
        let Some(target_ent) = avatar_id_manager.resolve_avatar_id(pkt.target) &&
        let Ok(player) = query.get(ent)
    {
        commands.entity(target_ent)
            .call_named_lua_method("RequestDialogue", player.object().clone());
    }
}

fn handle_dialogue_choice(
    In((ent, pkt)): In<(Entity, oaPktDialogChoice)>,
    avatar_id_manager: Res<AvatarIdManager>,
    mut commands: Commands,
) {
    let Some(speaker) = avatar_id_manager.resolve_avatar_id(pkt.target) else {
        return;
    };

    if let Ok(index) = pkt.dialog_choice_serial.parse() {
        commands.write_message(DialogueNodeSelected {
            player: ent,
            speaker,
            id: pkt.dialog_id,
            index,
        });
    } else {
        commands.write_message(DialogueEnd {
            player: ent
        });
    }
}

fn send_dialogue_nodes(
    mut messages: MessageReader<DialogueNodeSelected>,
    mut players: Query<(&mut DialogueState, &PlayerController)>,
    speakers: Query<&Avatar, With<NpcOtherlandTag>>,
    mut commands: Commands,
) {
    for event in messages.read() {
        let Ok((mut state, controller)) = players.get_mut(event.player) else {
            continue;
        };

        let Ok(speaker_info) = speakers.get(event.speaker) else {
            commands.write_message(DialogueEnd {
                player: event.player
            });
            continue;
        };

        state.current_index = event.index - 1;
        
        if let Some(node) = state.nodes.get(state.current_index) {
            let choices = node.choices.iter().map(|choice| {
                protocol::oaDialogChoice {
                    dialogue_serial_number: choice.next_index.unwrap_or(event.index  + 1).to_string(),
                    emote_index: match choice.choice_emote {
                        ChoiceEmote::Close => protocol::OaDialogChoiceEmoteIndex::Close,
                        ChoiceEmote::Approve => protocol::OaDialogChoiceEmoteIndex::Approve,
                        ChoiceEmote::Reject => protocol::OaDialogChoiceEmoteIndex::Reject,
                        ChoiceEmote::Next => protocol::OaDialogChoiceEmoteIndex::Next,
                        ChoiceEmote::TellMore => protocol::OaDialogChoiceEmoteIndex::TellMore,
                    },

                    ..Default::default()
                }
            }).collect::<Vec<_>>();

            let pkt = CPktStream_166_2 {
                field_1: DialogStructure {
                    npc_id: speaker_info.id,
                    dialog_id: state.quest_id,
                    dialog_node: oaDialogNode {
                        dialogue_id: state.quest_id,
                        dialog_content_id: node.content_id,
                        ..Default::default()
                    },
                    choice_count: choices.len() as u32,
                    choices,
                    has_additional_component: node.quest_id.is_some(),
                    component_factory_id: 0,
                    quest_prototype: match node.quest_id {
                        Some(id) => oaDialogQuestPrototype {
                            quest_id: id as u32,
                            ..Default::default()
                        },
                        None => oaDialogQuestPrototype::default(),
                    }
                },
                ..Default::default()
            };

            debug!("Sending dialogue to player {}: {:#?}", controller.avatar_id(), pkt);

            controller.send_packet(pkt);
        } else if state.quest_finisher {
            commands.write_message(ReturnQuest {
                player: event.player,
                quest_id: state.quest_id,
            });
        } else {
            // Node not found, end dialogue
            commands.write_message(DialogueEnd {
                player: event.player
            });
        }
    }
}

fn send_dialogue_end(
    mut messages: MessageReader<DialogueEnd>,
    players: Query<&PlayerController>,
    mut commands: Commands,
) {
    for event in messages.read() {
        let Ok(controller) = players.get(event.player) else {
            continue;
        };

        debug!("Ending dialogue for player {}", controller.avatar_id());

        controller.send_packet(oaPktDialogEnd {
            player_id: controller.avatar_id(),
            dialogue_id: -1,
            ..Default::default()
        });

        commands.entity(event.player)
            .remove::<DialogueState>();
    }
}