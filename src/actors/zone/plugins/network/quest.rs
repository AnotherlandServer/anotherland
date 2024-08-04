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

use std::{collections::HashSet, io::Write};

use atlas::{oaPktQuestEvent, oaPktQuestGiverStatus, oaQuestCondition, oaQuestTemplate, CPkt, CPktStream_165_2, CPktStream_165_7, NonClientBaseComponent, NpcBaseComponent, NpcBaseParams, OaPktQuestEventEvent, OaPktQuestRequestRequest, ParamBox, PlayerComponent, PlayerParams};
use bevy_ecs::{component::Component, entity::Entity, event::{EventReader, EventWriter}, query::{Added, Changed, Or, With}, system::{Commands, In, Query, Res}};
use bitvec::prelude::*;
use log::{debug, warn};

use crate::{actors::{zone::plugins::{QuestAbandoned, QuestAccepted, QuestCompleted, QuestFinished, QuestGiverStatus, QuestLog, Status, SubjectiveParamSet}, AvatarComponent, AvatarIdToEntityLookup, InterestList}, scripting::quest::lookup_quest_info};

use super::PlayerController;

pub fn handle_quest_request(
    In(pkt): In<CPkt>,
    player: Query<(&AvatarComponent, &PlayerController, &QuestLog)>,
    mut quest_accepted_event: EventWriter<QuestAccepted>,
    mut quest_abandoned_event: EventWriter<QuestAbandoned>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
) {
    if 
        let CPkt::oaPktQuestRequest(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.player) &&
        let Ok((avatar, controller, questlog)) = player.get(*player_ent)
    {
        match pkt.request {
            OaPktQuestRequestRequest::Accept => {
                quest_accepted_event.send(QuestAccepted(*player_ent, pkt.quest_id));
            },
            OaPktQuestRequestRequest::Return => {
                warn!("Qust return not implemented");
            },
            OaPktQuestRequestRequest::Abandon => {
                quest_abandoned_event.send(QuestAbandoned(*player_ent, pkt.quest_id));
            },
            OaPktQuestRequestRequest::Request => {
                unreachable!()
            },
            OaPktQuestRequestRequest::QueryActive => {
                let bv = bits![mut u8, Lsb0; 0; 10000];
                let mut active_quests = HashSet::new();

                active_quests.extend(questlog.in_progress.iter().map(|progress| progress.info.id));
                active_quests.extend(questlog.completed.iter().map(|progress| progress.info.id));
                
                for quest_id in active_quests {
                    bv.set(quest_id as usize, true);
                }

                controller.send_message(CPktStream_165_7 {
                    player: avatar.id,
                    quest_list: bv.to_bitvec().into_vec(),
                    ..Default::default()
                }.into_message());
            },
        };
    }
}

pub fn handle_quest_debug_request(
    In(pkt): In<CPkt>,
    player: Query<(&AvatarComponent, &QuestLog)>,
    mut quest_finished_event: EventWriter<QuestFinished>,
    mut quest_accepted_event: EventWriter<QuestAccepted>,
    mut quest_completed_event: EventWriter<QuestCompleted>,
    mut quest_abandoned_event: EventWriter<QuestAbandoned>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
) {
    if 
        let CPkt::oaPktQuestDebugRequest(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.player) &&
        let Ok((avatar, questlog)) = player.get(*player_ent)
    {

        debug!("{:#?}", pkt);

        match pkt.action {
            atlas::OaPktQuestDebugRequestAction::Finish => {
                quest_finished_event.send(QuestFinished(*player_ent, pkt.quest_id));
            },
            atlas::OaPktQuestDebugRequestAction::Add => {
                quest_accepted_event.send(QuestAccepted(*player_ent, pkt.quest_id));
            },
            atlas::OaPktQuestDebugRequestAction::Reset => {
                quest_abandoned_event.send(QuestAbandoned(*player_ent, pkt.quest_id));
            },
            atlas::OaPktQuestDebugRequestAction::Complete => {
                quest_completed_event.send(QuestCompleted(*player_ent, pkt.quest_id));
            },
            atlas::OaPktQuestDebugRequestAction::Unknown => warn!("Unknown quest debug action"),
            atlas::OaPktQuestDebugRequestAction::SetCycle => todo!(),
            atlas::OaPktQuestDebugRequestAction::Reload => todo!(),
        }
    }
}

pub fn notify_quest_accepted(
    mut ev: EventReader<QuestAccepted>,
    players: Query<(&AvatarComponent, &PlayerController)>,
) {
    for &QuestAccepted(player, id) in ev.read() {
        if let Ok((avatar, controller)) = players.get(player) {
            controller.send_message(oaPktQuestEvent {
                field_1: avatar.id,
                quest_id: id,
                event: OaPktQuestEventEvent::QuestAccepted,
                ..Default::default()
            }.into_message());
        }
    }
}

pub fn notify_quest_abandoned(
    mut ev: EventReader<QuestAbandoned>,
    players: Query<(&AvatarComponent, &PlayerController)>,
) {
    for &QuestAbandoned(player, id) in ev.read() {
        if let Ok((avatar, controller)) = players.get(player) {
            controller.send_message(oaPktQuestEvent {
                field_1: avatar.id,
                quest_id: id,
                event: OaPktQuestEventEvent::QuestAbandoned,
                ..Default::default()
            }.into_message());
        }
    }
}

pub fn update_quest_giver_status(
    players: Query<(&QuestGiverStatus, &PlayerController, &InterestList), Or<(Changed<QuestGiverStatus>, Changed<InterestList>)>>,
    npcs: Query<&AvatarComponent, With<NonClientBaseComponent>>,
) {
    for (status, controller, interests) in players.iter() {
        let mut marked = Vec::new();
        let mut unmarked = Vec::new();

        for &entity in interests.interests.iter() {
            let status = status.get(entity);

            if let Ok(avatar) = npcs.get(entity) {
                if matches!(status, Status::None) {
                    unmarked.push(avatar.id);
                } else {
                    marked.push(avatar.id);
                }
            }
        }

        debug!("Marked: {:#?}", marked);

        controller.send_message(oaPktQuestGiverStatus {
            avatar_count1: marked.len() as u32,
            enable_questmarker_for_avatars: marked,
            avatar_count2: unmarked.len() as u32,
            disable_questmarker_for_avatars: unmarked,
            ..Default::default()
        }.into_message());
    }
}

// oaPktQuestUpdate String
/* 
n~^~<GUIDs separated by |>~^~<LuaAssignedString>~^~<LuaParameter?>

n = 1 or something else
s = substring

*/