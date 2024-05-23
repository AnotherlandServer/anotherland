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

use atlas::{dialogStructure, oaDialogChoice, oaDialogNode, oaDialogQuestPrototype, oaQuestTemplate, AvatarId, CPkt, CPktStream_166_2, NpcBaseComponent, NpcBaseParams, OaDialogChoiceEmoteIndex, ParamBox, Uuid};
use bevy_ecs::{component::Component, entity::Entity, event::Event, query::With, system::{Commands, In, Query, Res}};

use crate::actors::{AvatarComponent, AvatarIdToEntityLookup, CurrentTarget, DialogChoice, DIALOGS};

use super::PlayerController;

#[derive(Component)]
pub struct DialogState {
    dialog_id: i32,
    dialog_choice_serial: String,
}

pub fn handle_dialog_request(
    In(pkt): In<CPkt>,
    player: Query<(&PlayerController, &CurrentTarget)>,
    npc: Query<(&AvatarComponent, &ParamBox), With<NpcBaseComponent>>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
    mut cmds: Commands,
) {
    if 
        let CPkt::oaPktDialogList(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.instigator) &&
        let Ok((controller, target_ent)) = player.get(*player_ent) &&
        let Ok((npc_avatar, npc_params)) = npc.get(target_ent.0)
    {
        let npc_params = npc_params.get_impl::<dyn NpcBaseParams>().unwrap();

        // lookup the first valid dialog
        let dialog = DIALOGS
            .iter()
            .find(|((id, serial), _)| npc_params.dialogs().contains(id) && serial == "0");

        if let Some((_, dialog)) = dialog {
            // store dialog state
            cmds
                .entity(*player_ent)
                .insert(DialogState {
                    dialog_id: dialog.id,
                    dialog_choice_serial: dialog.serial.clone()
                });

            controller.send_message(CPktStream_166_2 {
                field_1: dialogStructure {
                    npc_id: npc_avatar.id,
                    dialog_id: dialog.id,
                    dialog_node: oaDialogNode { 
                        dialogue_id: dialog.id, 
                        dialog_content_id: dialog.dialog_line_id, 
                        ..Default::default()
                    },
                    choice_count: dialog.choice_serials.len() as u32,
                    choices: dialog.choice_serials
                        .iter()
                        .map(|v| match v {
                            DialogChoice::Approve(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Approve,
                                ..Default::default()
                            },
                            DialogChoice::Reject(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Reject,
                                ..Default::default()
                            },
                            DialogChoice::TellMore(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::TellMore,
                                ..Default::default() 
                            },
                            DialogChoice::Next(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Next,
                                ..Default::default() 
                            },
                        })
                        .collect(),
                    ..Default::default()
                },
                ..Default::default()
            }.into_message());
        }

        /**/
    }
}

pub fn handle_dialog_choice(
    In(pkt): In<CPkt>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
    mut player: Query<(&PlayerController, &mut DialogState)>,
) {
    if 
        let CPkt::oaPktDialogChoice(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.instigator) &&
        let Ok((controller, mut state)) = player.get_mut(*player_ent)
    {
        // lookup the next dialog line
        let dialog = DIALOGS
            .iter()
            .find(|((id, serial), _)| pkt.dialog_id == *id && &pkt.dialog_choice_serial == serial);

        if let Some((_, dialog)) = dialog {
            state.dialog_choice_serial = dialog.serial.clone();

            controller.send_message(CPktStream_166_2 {
                field_1: dialogStructure {
                    npc_id: pkt.target,
                    dialog_id: dialog.id,
                    dialog_node: oaDialogNode { 
                        dialogue_id: dialog.id, 
                        dialog_content_id: dialog.dialog_line_id, 
                        ..Default::default()
                    },
                    choice_count: dialog.choice_serials.len() as u32,
                    choices: dialog.choice_serials
                        .iter()
                        .map(|v| match v {
                            DialogChoice::Approve(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Approve,
                                ..Default::default()
                            },
                            DialogChoice::Reject(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Reject,
                                ..Default::default()
                            },
                            DialogChoice::TellMore(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::TellMore,
                                ..Default::default() 
                            },
                            DialogChoice::Next(serial) => oaDialogChoice { 
                                dialogue_serial_number: serial.clone(), 
                                emote_index: OaDialogChoiceEmoteIndex::Next,
                                ..Default::default() 
                            },
                        })
                        .collect(),
                    field_5: dialog.quest.is_some(),
                    component_factory_id: 0,
                    quest_prototype: oaDialogQuestPrototype {
                        field_0: 0,
                        quest_id: 1350,
                        field_2: 0,
                    },
                },
                ..Default::default()
            }.into_message());
        }
    }
}

/*

oaQuestTemplate {
                        field_0: 1350,
                        field_1: "Interaction".to_string(),
                        field_2: 4,
                        field_5: Uuid::new(),
                        field_14: "Sellars wishes to check if your input is recognized by the Otherland network. He will summon a small box that you should open.".to_string(),
                        field_15: Uuid::new(),
                        ..Default::default()
                    }.to_bytes().into()

*/