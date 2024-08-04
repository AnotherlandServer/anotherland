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

use atlas::{dialogStructure, oaDialogChoice, oaDialogNode, oaDialogQuestPrototype, oaPktDialogEnd, raknet::Message, AvatarId, CPkt, CPktStream_166_2, NpcBaseComponent, NpcBaseParams, OaDialogChoiceEmoteIndex, ParamBox};
use bevy::{app::{Plugin, PostUpdate, Update}, utils::HashSet};
use bevy_ecs::{component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, query::With, system::{Commands, In, Query, Res}};
use log::debug;

use crate::{actors::{AvatarComponent, AvatarIdToEntityLookup, CurrentTarget}, scripting::dialogue::{lookup_dialogue_info, ChoiceIcon, DialogueInfo, DialogueNode}};

use super::{NetworkExt, PlayAnimationEvent, PlayerController, SubjectiveParamSet};

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_message_handler((0xa6, 0x0), handle_dialog_request);
        app.register_message_handler((0xa6, 0x1), handle_dialog_choice);
        app.add_systems(Update, update_completed_dialogues);
        app.add_event::<DialogueCompletedEvent>();
    }
}

#[derive(Component)]
pub struct CompletedDialogues(HashSet<i32>);

impl CompletedDialogues {
    pub fn new(dialogues: Vec<i32>) -> CompletedDialogues {
        CompletedDialogues(dialogues.into_iter().collect())
    }

    pub fn is_completed(&self, id: i32) -> bool {
        self.0.contains(&id)
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.0.iter().cloned().collect()
    }
}

#[derive(Event)]
pub struct DialogueCompletedEvent {
    pub player: Entity,
    pub dialogue_id: i32,
}

fn render_dialogue_messages(dialogue: &DialogueInfo, avatar_id: &AvatarId, node: usize) -> Option<(CPktStream_166_2, Option<String>)> {
    if let Some(node_info) = dialogue.nodes.get(node) {
        debug!("Node: {:?}", node_info);

        match node_info {
            DialogueNode::Line { line_id, animation, choices, quest } => {
                Some((
                    CPktStream_166_2 {
                        field_1: dialogStructure {
                            npc_id: *avatar_id,
                            dialog_id: dialogue.id,
                            dialog_node: oaDialogNode {
                                dialogue_id: dialogue.id,
                                dialog_content_id: *line_id as u32,
                                ..Default::default()
                            },
                            choice_count: choices.len() as u32,
                            choices: choices.iter()
                                .map(|choice| {
                                    oaDialogChoice {
                                        emote_index: match choice.icon {
                                            ChoiceIcon::Close => OaDialogChoiceEmoteIndex::Close,
                                            ChoiceIcon::Approve => OaDialogChoiceEmoteIndex::Approve,
                                            ChoiceIcon::Reject => OaDialogChoiceEmoteIndex::Reject,
                                            ChoiceIcon::Next => OaDialogChoiceEmoteIndex::Next,
                                            ChoiceIcon::TellMore => OaDialogChoiceEmoteIndex::TellMore,
                                        },
                                        dialogue_serial_number: choice.index.to_string(),
                                        ..Default::default()
                                    }
                                })
                                .collect(),
                            has_additional_component: quest.is_some(),
                            component_factory_id: 0,
                            quest_prototype: match quest {
                                Some(id) => oaDialogQuestPrototype {
                                    quest_id: *id as u32,
                                    ..Default::default()
                                },
                                None => oaDialogQuestPrototype::default(),
                            }
                        },
                        ..Default::default()
                    },
                    None
                ))
            },
            DialogueNode::Transition { dialogue_id } => {
                if let Some(dialogue) = lookup_dialogue_info(*dialogue_id) {
                    render_dialogue_messages(dialogue, avatar_id, 0)
                } else {
                    None
                }
            },
            DialogueNode::End => {
                None
            }
        }
    } else {
        None
    }
}

pub fn handle_dialog_request(
    In(pkt): In<CPkt>,
    player: Query<(&AvatarComponent, &PlayerController, &CurrentTarget, &CompletedDialogues)>,
    npc: Query<(&AvatarComponent, &ParamBox, &SubjectiveParamSet), With<NpcBaseComponent>>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
    mut animation_event: EventWriter<PlayAnimationEvent>,
) {
    if 
        let CPkt::oaPktDialogList(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.instigator) &&
        let Ok((player_avatar, controller, target_ent, completed_dialogues)) = player.get(*player_ent) &&
        let Ok((npc_avatar, npc_params, subjective_npc_params)) = npc.get(target_ent.0)
    {
        let npc_params = subjective_npc_params.params_for_player(*player_ent, npc_params);
        let npc_params = npc_params.get_impl::<dyn NpcBaseParams>().unwrap();

        debug!("Player {} talk to {}.", player_avatar.name, npc_avatar.name);

        if 
            let Some(dialogue_id) = npc_params.dialogs().iter().find(|id| !completed_dialogues.0.contains(*id)) &&
            let Some(dialogue) = lookup_dialogue_info(*dialogue_id)
        {
            if let Some((pkt, animation)) = render_dialogue_messages(dialogue, &npc_avatar.id, 0) {
                debug!("{:#?}", pkt.clone().into_message());

                controller.send_message(pkt.into_message());

                if let Some(animation) = animation {
                    animation_event.send(PlayAnimationEvent::PlayerInterruptAnimation { 
                        player: *player_ent, 
                        entity: target_ent.0, 
                        animation, 
                    });
                }
            } else {
                completed_event.send(DialogueCompletedEvent { 
                    player: *player_ent, 
                    dialogue_id: *dialogue_id
                });

                // end dialogue
                controller.send_message(
                    oaPktDialogEnd {
                        player_id: pkt.instigator,
                        dialogue_id: *dialogue_id,
                        ..Default::default()
                    }.into_message()
                );
            }
        }
    }
}

pub fn handle_dialog_choice(
    In(pkt): In<CPkt>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
    mut completed_event: EventWriter<DialogueCompletedEvent>,
    mut player: Query<&PlayerController>,
    mut animation_event: EventWriter<PlayAnimationEvent>,
) {
    if 
        let CPkt::oaPktDialogChoice(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.instigator) &&
        let Some(target_ent) = avatar_id_lookup.get(&pkt.target) &&
        let Ok(controller) = player.get_mut(*player_ent)
    {
        if 
            let Some(dialogue) = lookup_dialogue_info(pkt.dialog_id) &&
            let Ok(serial) = pkt.dialog_choice_serial.parse() &&
            let Some((pkt, animation)) = render_dialogue_messages(dialogue, &pkt.target, serial)
        {
            debug!("{:#?}", pkt.clone().into_message());

            controller.send_message(pkt.into_message());

            if let Some(animation) = animation {
                animation_event.send(PlayAnimationEvent::PlayerInterruptAnimation { 
                    player: *player_ent, 
                    entity: *target_ent, 
                    animation, 
                });
            }
        } else {
            completed_event.send(DialogueCompletedEvent { 
                player: *player_ent, 
                dialogue_id: pkt.dialog_id
            });

            // end dialogue
            controller.send_message(
                oaPktDialogEnd {
                    player_id: pkt.instigator,
                    dialogue_id: pkt.dialog_id,
                    ..Default::default()
                }.into_message()
            );
        }
    }
}

fn update_completed_dialogues(
    mut completed: EventReader<DialogueCompletedEvent>,
    mut players: Query<&mut CompletedDialogues>,
) {
    for DialogueCompletedEvent { player, dialogue_id } in completed.read() {
        if let Ok(mut completed) = players.get_mut(*player) {
            completed.0.insert(*dialogue_id);
        }
    }
}
