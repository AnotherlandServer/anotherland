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

use bevy::{ecs::{entity::Entity, hierarchy::ChildOf, lifecycle::RemovedComponents, query::{Added, Changed}, system::{Commands, In, Query, Res}}, math::Vec3, platform::collections::{HashMap, HashSet}};
use log::{debug, error, warn};
use mlua::Table;
use obj_params::GameObjectData;
use protocol::{AvatarFilter, CPktStream_165_2, CPktStream_165_7, OaPktQuestRequestRequest, OaQuestConditionKind, QuestUpdateData, oaPktQuestGiverStatus, oaPktQuestRequest, oaPktQuestUpdate, oaQuestBeacon, oaQuestCondition, oaQuestTemplate};
use realm_api::{QuestCondition, QuestProgressionState};
use scripting::{LuaRuntime, LuaTableExt};

use crate::{instance::ZoneInstance, plugins::{AbandonQuest, AcceptQuest, Avatar, PlayerController, Quest, QuestAvailable, QuestLog, QuestPlayer, QuestRegistry, RequestNextQuest, ReturnQuest, quests::lua::AvatarFilterLua}};

pub(super) fn transmit_questlog(
    query: Query<(&QuestLog, &PlayerController), Added<QuestLog>>,
) {
    for (quest_log, controller) in query.iter() {
        for (_, quest) in quest_log.quests.iter() {
            if 
                let Some(state) = &quest.state &&
                !matches!(state.state, QuestProgressionState::Finished)
            {
                controller.send_packet(oaPktQuestUpdate {
                    player: controller.avatar_id(),
                    quest_id: quest.template.id as u32,
                    entry_count: state.conditions.len() as u32,
                    conditions: state.conditions.iter()
                        .map(|&QuestCondition { id, current_count, .. }| QuestUpdateData {
                            condition_id: id,
                            count: current_count,
                            ..Default::default()
                        }).collect(),
                    quest_failed: matches!(state.state, QuestProgressionState::Failed),
                    accepted_time: state.accepted_time.timestamp_millis(),
                    ..Default::default()
                });
            }
        }
    }
}

pub(super) fn sync_quest_markers(
    changed_markers: Query<Entity, Changed<QuestAvailable>>,
    avatars: Query<&Avatar>,
    players: Query<&PlayerController>,
    markers: Query<(&ChildOf, &QuestPlayer)>,
    mut removed_markers: RemovedComponents<QuestAvailable>,
    mut commands: Commands,
) {
    let mut added_avatars = HashMap::new();
    let mut removed_avatars = HashMap::new();

    for marker_ent in changed_markers.iter() {
        let Ok((ChildOf(parent), QuestPlayer(player))) = markers.get(marker_ent) else {
            continue;
        };

        let Ok(avatar) = avatars.get(*parent) else {
            continue;
        };

        added_avatars
            .entry(*player)
            .or_insert_with(Vec::new)
            .push(avatar.id);
    }

    for marker_ent in removed_markers.read() {
        let Ok((ChildOf(parent), QuestPlayer(player))) = markers.get(marker_ent) else {
            continue;
        };

        let Ok(avatar) = avatars.get(*parent) else {
            continue;
        };

        removed_avatars
            .entry(*player)
            .or_insert_with(Vec::new)
            .push(avatar.id);

        commands.entity(marker_ent).despawn();
    }

    let players_to_notify = added_avatars.keys()
        .chain(removed_avatars.keys())
        .copied()
        .collect::<HashSet<_>>();

    for player_ent in players_to_notify {
        let Ok(player_controller) = players.get(player_ent) else {
            continue;
        };

        let added_avatars = added_avatars.get(&player_ent).cloned().unwrap_or_default();
        let mut removed_avatars = removed_avatars.get(&player_ent).cloned().unwrap_or_default();

        removed_avatars.retain(|id| !added_avatars.contains(id));

        debug!("Quest markers updated for player {}: added: {:?}, removed: {:?}", player_ent, added_avatars, removed_avatars);

        player_controller.send_packet(oaPktQuestGiverStatus {
            avatar_count1: added_avatars.len() as u32,
            avatar_count2: removed_avatars.len() as u32,
            enable_questmarker_for_avatars: added_avatars,
            disable_questmarker_for_avatars: removed_avatars,
            ..Default::default()
        });
    }
}


pub(super) fn send_quest(_lua: &mlua::Lua, controller: &PlayerController, quest: &Quest, zone: &ZoneInstance, beacon_query: &Query<&GameObjectData>) {
    let conditions = quest.table.get::<Table>("conditions").ok()
        .and_then(|table| {
            let mut conditions = vec![];

            for pair in table.pairs::<i32, Table>() {
                let (_, condition_table) = pair.ok()?;

                let kind = match condition_table.get::<String>("type").ok()?.as_str() {
                        "loot" => OaQuestConditionKind::Loot,
                        "interact" => OaQuestConditionKind::Interact,
                        "dialog" => OaQuestConditionKind::Dialog,
                        "wait" => OaQuestConditionKind::Wait,
                        _ => {
                            warn!("Unknown quest condition type: {}", condition_table.get::<String>("type").ok()?);
                            continue;
                        }
                    };

                let beacon = if 
                        let Ok(beacon) = condition_table.get::<Table>("beacon") && 
                        let Ok(ent) = beacon.entity() &&
                        let Ok(data) = beacon_query.get(ent)
                    {
                        oaQuestBeacon {
                            world_guid: *zone.world_def.guid(),
                            zone_guid: *zone.zone.guid(),
                            position: data.get_named::<Vec3>("pos").copied().unwrap_or_default().into(),
                            height: data.get_named::<i32>("BeaconHeight").copied().unwrap_or_default() as u32,
                            radius: data.get_named::<i32>("BeaconRadius").copied().unwrap_or_default() as u32,
                        }
                    } else {
                        oaQuestBeacon::default()
                    };

                let (filter1, filter2) = match kind {
                    OaQuestConditionKind::Unk0 | 
                    OaQuestConditionKind::Unk2 | 
                    OaQuestConditionKind::Unk6 | 
                    OaQuestConditionKind::Unk7 | 
                    OaQuestConditionKind::Unk8 | 
                    OaQuestConditionKind::Interact => {
                        (
                            condition_table.get::<AvatarFilterLua>("avatar_filter").ok()
                            .map(|f| f.into())
                            .unwrap_or_default(),
                            AvatarFilter::default()
                        )
                    },
                    OaQuestConditionKind::Dialog => {
                        (
                            AvatarFilter::default(),
                            AvatarFilter {
                                kind: condition_table.get::<i32>("dialog").unwrap_or(0),
                                ..Default::default()
                            }
                        )
                    },
                    OaQuestConditionKind::Loot => {
                        (
                            AvatarFilter::default(),
                            AvatarFilter {
                                kind: 4,
                                filter: condition_table.get::<String>("item").unwrap_or_default(),
                            }
                        )
                    },
                    OaQuestConditionKind::Unk5 => {
                        (
                            AvatarFilter::default(),
                            condition_table.get::<AvatarFilterLua>("avatar_filter").ok()
                            .map(|f| f.into())
                            .unwrap_or_default()
                        )
                    },
                    OaQuestConditionKind::Unk17 => {
                        (
                            AvatarFilter::default(),
                            condition_table.get::<AvatarFilterLua>("avatar_filter").ok()
                            .map(|f| f.into())
                            .unwrap_or_default()
                        )
                    },
                    OaQuestConditionKind::Wait => (AvatarFilter::default(), AvatarFilter::default()),
                };

                let condition = oaQuestCondition {
                    quest_id: quest.id,
                    condition_id: condition_table.get::<i32>("id").ok()?,
                    kind,
                    filter1,
                    filter2,
                    required_count: condition_table.get::<i32>("required_count").ok()?,
                    waypoint: beacon,
                    ..Default::default()
                };

                conditions.push(condition);
            }

            Some(conditions)
        })
        .unwrap_or_default();

    let pkt = CPktStream_165_2 {
        field_1: oaQuestTemplate {
            quest_id: quest.id,
            world_guid: *quest.world_def.guid(),
            level: quest.table.get::<i32>("level").unwrap_or(0),
            bit_reward: quest.table.get::<i32>("bit_reward").unwrap_or(0),
            exp_reward: quest.table.get::<i32>("exp_reward").unwrap_or(0),
            progress_dialogue: quest.table.get::<i32>("progress_dialogue").unwrap_or_default(),
            completion_dialogue: quest.table.get::<i32>("completion_dialogue").unwrap_or_default(),
            system_flags: 16,
            ..Default::default()
        },
        conditions: conditions.len() as u32,
        field_3: conditions,
        ..Default::default()
    };

    debug!("Sending quest {} to player {}: {:#?}", quest.id, controller.character_id(), pkt);

    controller.send_packet(pkt);
}

pub(super) fn handle_quest_request(
    In((ent, pkt)): In<(Entity, oaPktQuestRequest)>,
    players: Query<(&QuestLog, &PlayerController)>,
    quests: Res<QuestRegistry>,
    runtime: Res<LuaRuntime>,
    zone: Res<ZoneInstance>,
    beacon_query: Query<&GameObjectData>,
    mut commands: Commands,
) {
    debug!("Received quest request from player {}: {:?}", ent, pkt);

    match pkt.request {
        OaPktQuestRequestRequest::Request => {
            let Some(quest) = quests.0.get(&pkt.quest_id) else {
                error!("Player {} requested unknown quest {}", ent, pkt.quest_id);
                return;
            };

            let Ok((_, player_controller)) = players.get(ent) else {
                return;
            };

            send_quest(runtime.vm(), player_controller, quest, &zone, &beacon_query);
        },
        OaPktQuestRequestRequest::QueryActive => {
            let Ok((questlog, player_controller)) = players.get(ent) else {
                return;
            };

            let mut pkt = CPktStream_165_7 {
                player: player_controller.avatar_id(),
                quest_list: vec![0; 0x4e2],
                ..Default::default()
            };

            for (&quest_id, progress) in &questlog.quests {
                if progress.state.is_none() || !(0..=9999).contains(&quest_id) {
                    continue;
                }

                pkt.quest_list[(quest_id / 8) as usize] |= 1 << (quest_id % 8);
            }

            debug!("Sending active quest list to player {}: {:?}", ent, pkt);

            player_controller.send_packet(pkt);
        },
        OaPktQuestRequestRequest::Accept => {
            commands.write_message(AcceptQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        },
        OaPktQuestRequestRequest::Abandon => {
            commands.write_message(AbandonQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        },
        OaPktQuestRequestRequest::Return => {
            commands.write_message(ReturnQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        },
        OaPktQuestRequestRequest::RequestNext => {
            commands.write_message(RequestNextQuest {
                player: ent,
            });
        }
    }
}