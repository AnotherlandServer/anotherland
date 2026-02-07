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
use bevy::{ecs::{change_detection::DetectChanges, component::Component, entity::Entity, query::Added, relationship::RelationshipTarget, system::{Commands, In, Query}, world::Ref}, math::Vec3, platform::collections::HashSet};
use futures::TryStreamExt;
use log::debug;
use protocol::{AvatarFilter, CPktStream_165_2, CPktStream_165_7, OaPktQuestRequestRequest, OaQuestConditionKind, QuestUpdateData, oaPktQuestGiverStatus, oaPktQuestRequest, oaPktQuestUpdate, oaQuestBeacon, oaQuestCondition, oaQuestTemplate};
use realm_api::{AvatarSelector, Condition, QuestCondition, QuestProgressionState, RealmApi};
use toolkit::types::AvatarId;

use crate::{plugins::{AbandonQuest, AcceptQuest, AsyncOperationEntityCommandsExt, ContentCache, ContentCacheRef, Interests, PlayerController, QuestGiver, QuestLog, QuestProgress, QuestTags, ReturnQuest, WeakCache, player_error_handler_system, quests::cache::QuestTemplateCache}};

pub struct AvatarFilterConverter(AvatarSelector);

impl Into<AvatarFilter> for AvatarFilterConverter {
    fn into(self) -> AvatarFilter {
        let mut filter = AvatarFilter::default();

        match self.0 {
            AvatarSelector::ContentId(id) => {
                filter.kind = 1;
                filter.filter = id.to_string();
            },
            AvatarSelector::InstanceId(id) => {
                filter.kind = 2;
                filter.filter = id.to_string();
            },
            AvatarSelector::QuestTag(tag) => {
                filter.kind = 3;
                filter.filter = tag.to_string();
            },
            AvatarSelector::LootItem(item) => {
                filter.kind = 4;
                filter.filter = item.to_string();
            },
            AvatarSelector::DialogId(id) => {
                filter.kind = 5;
                filter.filter = id.to_string();
            },
        }

        filter
    }
}

pub(super) fn transmit_questlog(
    players: Query<(&PlayerController, &QuestLog), Added<QuestLog>>,
    quests: Query<&QuestProgress>,
) {
    for (controller, quest_log) in players.iter() {
        for quest_ent in quest_log.quests.values() {
            if 
                let Ok(progress) = quests.get(*quest_ent) &&
                !matches!(progress.state().state, QuestProgressionState::Finished)
            {
                debug!("Sending quest update for quest {} to player {}", progress.state().quest_id, controller.character_id());

                controller.send_packet(oaPktQuestUpdate {
                    player: controller.avatar_id(),
                    quest_id: progress.state().quest_id as u32,
                    entry_count: progress.state().conditions.len() as u32,
                    conditions: progress.state().conditions.iter()
                        .map(|&QuestCondition { id, current_count, .. }| QuestUpdateData {
                            condition_id: id,
                            count: current_count,
                            ..Default::default()
                        }).collect(),
                    quest_failed: matches!(progress.state().state, QuestProgressionState::Failed),
                    accepted_time: progress.state().accepted_time.timestamp_millis(),
                    ..Default::default()
                });
            }
        }
    }
}

#[derive(Component, Default)]
pub struct QuestGiverStatus(HashSet<(Entity, AvatarId)>);

pub(super) fn sync_quest_markers(
    mut players: Query<(&PlayerController, Ref<QuestLog>, Ref<Interests>, &mut QuestGiverStatus)>,
    questgiver_tags: Query<&QuestGiver>,
    npcs: Query<&QuestTags>,
) {
    for (controller, questlog, interests, mut status) in players.iter_mut() {
        if questlog.is_changed() || interests.is_changed() {
            let mut questgivers = HashSet::new();

            'npc_loop: for (&ent, (avatar, _)) in interests.collection().iter() {
                let Ok(tags) = npcs.get(ent).map(|tags| tags.collection()) else {
                    continue;
                };

                for tag_ent in tags {
                    let Ok(questgiver) = questgiver_tags.get(*tag_ent) else {
                        continue;
                    };

                    if questlog.available.contains(&questgiver.template.id) {
                        questgivers.insert((ent, *avatar));
                        continue 'npc_loop;
                    }
                }
            }

            let added_avatars = questgivers.difference(&status.0)
                .map(|(_, avatar)| *avatar)
                .collect::<Vec<_>>();
            let removed_avatars = status.0.difference(&questgivers)
                .map(|(_, avatar)| *avatar)
                .collect::<Vec<_>>();

            if added_avatars.is_empty() && removed_avatars.is_empty() {
                continue;
            }

            status.0 = questgivers;

            controller.send_packet(oaPktQuestGiverStatus {
                avatar_count1: added_avatars.len() as u32,
                avatar_count2: removed_avatars.len() as u32,
                enable_questmarker_for_avatars: added_avatars,
                disable_questmarker_for_avatars: removed_avatars,
                ..Default::default()
            });
        }
    }
}

pub(super) fn handle_quest_request(
    In((ent, pkt)): In<(Entity, oaPktQuestRequest)>,
    players: Query<(&QuestLog, &PlayerController)>,
    //quests: Res<QuestRegistry>,
    //runtime: Res<LuaRuntime>,
    //zone: Res<ZoneInstance>,
    //beacon_query: Query<&GameObjectData>,
    mut commands: Commands,
) {
    debug!("Received quest request from player {}: {:?}", ent, pkt);

    match pkt.request {
        OaPktQuestRequestRequest::Request => {
            let Ok((_, player_controller)) = players.get(ent) else {
                return;
            };

            let player_controller = player_controller.clone();

            commands
                .entity(ent)
                .perform_async_operation(async move {
                    let Some(template) = QuestTemplateCache::get(&pkt.quest_id).await? 
                    else {
                        return Err(anyhow!("Quest template not found for quest id: {}", pkt.quest_id).into());
                    };

                    let Some(world) = RealmApi::get()
                        .get_worlddef(template.world_id as u16)
                        .await? 
                    else {
                        return Err(anyhow!("World definition not found for world id: {}", template.world_id).into());
                    };

                    let completed_beacon = if 
                        let Some(id) = template.completion_dialogue_id &&
                        let Some(mut placement) = RealmApi::get()
                            .query_placements_by_selector(Some(template.world_id), None, AvatarSelector::DialogId(id)).await?
                            .into_iter()
                            .next() &&
                        let Some(content) = ContentCache::get(&ContentCacheRef::Uuid(placement.content_guid)).await?
                    {
                        placement.data.set_parent(Some(content));

                        Some(oaQuestBeacon {
                            world_guid: *world.guid(),
                            zone_guid: placement.zone_guid,
                            position: placement.data.get_named::<Vec3>("pos").copied().unwrap_or_default().into(),
                            ..Default::default()
                        })
                    } else {
                        None
                    };

                    let mut response = CPktStream_165_2 {
                        field_1: oaQuestTemplate {
                            quest_id: template.id,
                            world_guid: *world.guid(),
                            level: template.level,
                            bit_reward: template.bit_reward.unwrap_or_default(),
                            exp_reward: template.exp_reward.unwrap_or_default(),
                            progress_dialogue: template.progress_dialogue_id.unwrap_or_default(),
                            completion_dialogue: template.completion_dialogue_id.unwrap_or_default(),
                            completed_beacon: completed_beacon.unwrap_or_default(),
                            system_flags: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    for condition in &template.conditions {
                        let beacon = {
                            let beacon_selector = match *condition {
                                Condition::Dialogue { beacon, dialogue_id, .. } => {
                                    if let Some(id) = beacon {
                                        Some(AvatarSelector::InstanceId(id))
                                    } else {
                                        Some(AvatarSelector::DialogId(dialogue_id))
                                    }
                                },
                                Condition::Interact { beacon, avatar_selector, .. } => {
                                    if let Some(id) = beacon {
                                        Some(AvatarSelector::InstanceId(id))
                                    } else {
                                        Some(avatar_selector)
                                    }
                                },
                                Condition::Kill { beacon, avatar_selector, .. } => {
                                    if let Some(id) = beacon {
                                        Some(AvatarSelector::InstanceId(id))
                                    } else {
                                        Some(avatar_selector)
                                    }
                                },
                                Condition::Loot { beacon, item_id, .. } => {
                                    if let Some(id) = beacon {
                                        Some(AvatarSelector::InstanceId(id))
                                    } else {
                                        Some(AvatarSelector::LootItem(item_id))
                                    }
                                },
                                Condition::Proximity { beacon, avatar_selector, .. } => {
                                    if let Some(id) = beacon {
                                        Some(AvatarSelector::InstanceId(id))
                                    } else {
                                        Some(avatar_selector)
                                    }
                                }
                                _ => None,
                            };

                            if 
                                let Some(selector) = beacon_selector &&
                                let Some(mut placement) = RealmApi::get()
                                    .query_placements_by_selector(Some(template.world_id), None, selector).await?
                                    .into_iter()
                                    .next() &&
                                let Some(content) = ContentCache::get(&ContentCacheRef::Uuid(placement.content_guid)).await?
                            {
                                placement.data.set_parent(Some(content));

                                Some(oaQuestBeacon {
                                    world_guid: *world.guid(),
                                    zone_guid: placement.zone_guid,
                                    position: placement.data.get_named::<Vec3>("pos").copied().unwrap_or_default().into(),
                                    height: placement.data.get_named::<i32>("BeaconHeight").copied().unwrap_or_default() as u32,
                                    radius: placement.data.get_named::<i32>("BeaconRadius").copied().unwrap_or_default() as u32,
                                })
                            } else {
                                None
                            }
                        };

                        match *condition {
                            Condition::Dialogue { id, stage, required_count, dialogue_id, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Dialog,
                                    filter1: AvatarFilter::default(),
                                    filter2: AvatarFilter {
                                        kind: dialogue_id, // Dialogue id must be written into the kind field,
                                                            // because of an implementation quirk in the client.
                                        ..Default::default()
                                    },
                                    required_count,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            },
                            Condition::Interact { id, stage, required_count, avatar_selector, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Interact,
                                    filter1: AvatarFilterConverter(avatar_selector).into(),
                                    filter2: AvatarFilter::default(),
                                    required_count,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            },
                            Condition::Kill { id, stage, required_count, avatar_selector, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Unk2,
                                    filter1: AvatarFilterConverter(avatar_selector).into(),
                                    filter2: AvatarFilter::default(),
                                    required_count,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            },
                            Condition::Loot { id, stage, required_count, item_id, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Loot,
                                    filter1: AvatarFilter::default(),
                                    filter2: AvatarFilterConverter(AvatarSelector::LootItem(item_id)).into(),
                                    required_count,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            },
                            Condition::Wait { id, stage, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Wait,
                                    filter1: AvatarFilter::default(),
                                    filter2: AvatarFilter::default(),
                                    required_count: 1,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            },
                            Condition::Proximity { id, stage, .. } => {
                                response.field_3.push(oaQuestCondition {
                                    quest_id: template.id,
                                    condition_id: id,
                                    kind: OaQuestConditionKind::Unk0,
                                    filter1: AvatarFilter::default(),
                                    filter2: AvatarFilter::default(),
                                    required_count: 1,
                                    stage,
                                    waypoint: beacon.clone().unwrap_or_default(),
                                    flags: 2,
                                    ..Default::default()
                                });
                            }
                        }
                    }

                    response.conditions = response.field_3.len() as u32;

                    debug!("Sending quest {} to player {}: {:#?}", template.id, player_controller.character_id(), response);

                    player_controller.send_packet(response);

                    Ok(())
                })
                .on_error_run_system(player_error_handler_system);
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

            for &quest_id in questlog.quests.keys() {
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
            /*commands.write_message(RequestNextQuest {
                player: ent,
            });*/
        }
    }
}