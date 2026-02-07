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

mod cache;
mod component;
mod speaker;

pub use component::*;
use realm_api::{Choice, QuestProgressionState};
pub use speaker::*;

use anyhow::anyhow;
use bevy::{app::{Plugin, PreStartup, Update}, ecs::{message::{Message, MessageReader}, relationship::RelationshipTarget}, prelude::{App, Commands, Entity, In, Query, Res, With, World}};
use log::{debug, warn};
use mlua::{Lua, Table};
use obj_params::{GameObjectData, Player, tags::PlayerTag};
use protocol::{CPktStream_166_2, DialogStructure, OaPktDialogListKind, oaDialogNode, oaDialogQuestPrototype, oaPktDialogChoice, oaPktDialogEnd, oaPktDialogList};
use scripting::{LuaExt, LuaRuntime, LuaTableExt};
use toolkit::types::AvatarId;

use crate::{error::WorldResult, plugins::{Avatar, InstanceManager, QuestLog, QuestProgress, ReturnQuest}};

use super::{AvatarIdManager, NetworkExtPriv, PlayerController};

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, insert_dialogue_api);
        app.add_systems(Update, (run_dialogues, send_dialogue_end, run_deferred_quest_dialogues));

        app.register_message_handler(handle_dialogue_request);
        app.register_message_handler(handle_dialogue_choice);

        app.add_message::<DialogueEnd>();
        app.add_message::<RunDialogue>();
        app.add_message::<RunDeferredQuestDialogues>();
    }
}

#[derive(Message)]
pub struct RunDialogue {
    pub player: Entity,
    pub speaker: Entity,
    pub serial: Option<i32>,
}

#[derive(Message)]
pub struct DialogueEnd {
    pub player: Entity,
}

#[derive(Message)]
pub struct RunDeferredQuestDialogues {
    pub player: Entity,
}

fn insert_dialogue_api(
    world: &mut World,
) {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let dialogue_api = lua.create_table().unwrap();
    runtime.register_native("dialogue", dialogue_api.clone()).unwrap();

    dialogue_api.set("ShowTutorialMessage", lua.create_bevy_function(world, lua_show_tutorial_message).unwrap()).unwrap();
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

fn handle_dialogue_request(
    In((ent, pkt)): In<(Entity, oaPktDialogList)>,
    avatar_id_manager: Res<AvatarIdManager>,
    instance_manager: Res<InstanceManager>,
    mut commands: Commands,
) {
    debug!("Received dialogue request from player entity {}: {:#?}", ent, pkt);

    match pkt.kind {
        OaPktDialogListKind::Target => {
            if let Some(speaker) = avatar_id_manager.resolve_avatar_id(pkt.target) {
                commands.write_message(RunDialogue {
                    player: ent,
                    speaker,
                    serial: None
                });
            }
        },
        OaPktDialogListKind::Guid => {
            if let Some(speaker) = instance_manager.find_instance(pkt.id) {
                commands.write_message(RunDialogue {
                    player: ent,
                    speaker,
                    serial: None
                });
            }
        },
    }
}

fn handle_dialogue_choice(
    In((ent, pkt)): In<(Entity, oaPktDialogChoice)>,
    player: Query<&QuestLog>,
    quests: Query<&QuestProgress>,
    avatar_id_manager: Res<AvatarIdManager>,
    mut commands: Commands,
) {
    let Some(speaker) = avatar_id_manager.resolve_avatar_id(pkt.target) else {
        return;
    };

    let Ok(quest_log) = player.get(ent) else {
        return;
    };

    if 
        let Ok(index) = pkt.dialog_choice_serial.parse()
    {
        if index == 0 {
            for quest in quest_log.quests.values() {
                if 
                    let Ok(quest_progress) = quests.get(*quest) &&
                    quest_progress.state().state == QuestProgressionState::Completed
                {
                    commands
                        .write_message(ReturnQuest {
                            player: ent,
                            quest_id: quest_progress.state().quest_id,
                        })
                        .entity(ent)
                        .insert(DeferredQuestDialogueResponse {
                            speaker
                        });

                    return;
                }
            }
        }

        commands
            .write_message(RunDialogue {
                player: ent,
                speaker,
                serial: if index == 0 {
                    None
                } else {
                    Some(index)
                }
            });
    } else {
        commands.write_message(DialogueEnd {
            player: ent
        });
    }
}

fn send_dialogue_end(
    mut messages: MessageReader<DialogueEnd>,
    players: Query<&PlayerController>,
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
    }
}

fn run_dialogues(
    mut messages: MessageReader<RunDialogue>,
    player: Query<(&PlayerController, &GameObjectData, &QuestLog), With<PlayerTag>>,
    target: Query<(&Avatar, &Dialogues)>,
    dialogues: Query<&Dialogue>,
) {

    for &RunDialogue { player: player_ent, speaker, serial } in messages.read() {

        if 
            let Ok((target, dialogue_ents)) = target.get(speaker) &&
            let Ok((controller, object, questlog)) = player.get(player_ent)
        {
            'dialogue: for dialogue in dialogue_ents.collection() {
                let Ok(dialogue) = dialogues.get(*dialogue) else {
                    continue;
                };

                let branches = dialogue.0.branches
                    .iter()
                    .filter(|branch| {
                        if let Some(selector) = branch.selector.as_ref() {
                            *object.get::<_, i32>(Player::Lvl).unwrap() >= selector.level &&
                            (selector.quests_available.is_empty() || selector.quests_available.iter().any(|quest| questlog.available.contains(quest))) &&
                            (selector.quests_in_progress.is_empty() || selector.quests_in_progress.iter().any(|quest| questlog.in_progress.contains(quest))) &&
                            (selector.quests_complete.is_empty() || selector.quests_complete.iter().any(|quest| questlog.completed.contains(quest))) &&
                            (selector.quests_finished.is_empty() || selector.quests_finished.iter().any(|quest| questlog.finished.contains(quest))) &&
                            (selector.quests_upcoming.is_empty() || selector.quests_upcoming.iter().any(|quest| {
                                questlog.available.contains(quest) ||
                                (
                                    !questlog.in_progress.contains(quest) &&
                                    !questlog.completed.contains(quest) &&
                                    !questlog.finished.contains(quest)
                                )
                            }))
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>();

                for branch in branches.iter() {
                    let Some(current_line) = (if let Some(serial) = serial {
                        branch.lines.iter().find(|line| line.serial == serial)
                    } else {
                        branch.lines.first()
                    }) else {
                        warn!("No line found in branch for serial {:?}", serial);
                        continue;
                    };

                    let next_line = branch.lines
                        .iter()
                        .find(|line| line.serial > current_line.serial);

                    let choices = if let Some(choice) = current_line.choice.as_ref() {
                        vec![protocol::oaDialogChoice {
                            dialogue_serial_number: next_line
                                .map(|line| line.serial)
                                .unwrap_or(0)
                                .to_string(),
                            emote_index: match choice {
                                Choice::Close => protocol::OaDialogChoiceEmoteIndex::Close,
                                Choice::Approve => protocol::OaDialogChoiceEmoteIndex::Approve,
                                Choice::Reject => protocol::OaDialogChoiceEmoteIndex::Reject,
                                Choice::Next => protocol::OaDialogChoiceEmoteIndex::Next,
                                Choice::TellMore => protocol::OaDialogChoiceEmoteIndex::TellMore,
                                _ => protocol::OaDialogChoiceEmoteIndex::Close,
                            },

                            ..Default::default()
                        }]
                    } else {
                        vec![protocol::oaDialogChoice {
                            dialogue_serial_number: next_line
                                .map(|line| line.serial)
                                .unwrap_or(0)
                                .to_string(),
                            emote_index: protocol::OaDialogChoiceEmoteIndex::TellMore,
                            ..Default::default()
                        }]
                    };

                    let pkt = CPktStream_166_2 {
                        field_1: DialogStructure {
                            npc_id: target.id,
                            dialog_id: dialogue.0.id,
                            dialog_node: oaDialogNode {
                                dialogue_id: dialogue.0.id,
                                dialog_content_id: current_line.line_id as u32,
                                ..Default::default()
                            },
                            choice_count: choices.len() as u32,
                            choices,
                            has_additional_component: current_line.quest_id.is_some(),
                            component_factory_id: 0,
                            quest_prototype: match current_line.quest_id {
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
                    break 'dialogue;
                }
            }
        }
    }
}

fn run_deferred_quest_dialogues(
    mut messages: MessageReader<RunDeferredQuestDialogues>,
    deferreds: Query<&DeferredQuestDialogueResponse>,
    mut commands: Commands,
) {
    for &RunDeferredQuestDialogues { player } in messages.read() {
        if let Ok(deferred) = deferreds.get(player) {
            commands
                .write_message(RunDialogue {
                    player,
                    speaker: deferred.speaker,
                    serial: None,
                })
                .entity(player)
                .remove::<DeferredQuestDialogueResponse>();
        }
    }
}
