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

mod commands;
mod lua;
mod questlog;

pub use questlog::*;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{hierarchy::ChildOf, lifecycle::RemovedComponents, message::{Message, MessageReader}, query::{Changed, With}, resource::Resource, schedule::IntoScheduleConfigs, system::{In, ParamSet, Res, ResMut}}, math::Vec3, platform::collections::HashSet, prelude::{Added, App, Commands, Component, Entity, Query}, state::state::OnEnter};
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use mlua::{Function, Table, Value};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase};
use protocol::{oaPktQuestEvent, oaPktQuestGiverStatus, oaPktQuestRequest, oaPktQuestUpdate, oaPktRequestQuestAction, oaQuestBeacon, oaQuestCondition, oaQuestTemplate, AvatarFilter, CPktStream_165_2, CPktStream_165_7, OaPktQuestEventEvent, OaPktQuestRequestRequest, OaPktRequestQuestActionKind, OaQuestConditionKind, QuestUpdateData};
use realm_api::{QuestCondition, QuestProgressionState, RealmApi, WorldDef};
use scripting::{EntityScriptCommandsExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject};

use crate::{instance::{InstanceState, ZoneInstance}, plugins::{AsyncOperationEntityCommandsExt, Avatar, AvatarIdManager, CommandExtPriv, DespawnAvatar, DialogueState, InterestState, InterestTransmitted, Interests, NetworkExtPriv, PlayerController, player_error_handler_system, quests::{commands::{command_accept_quest, command_complete_quest, command_fail_quest, command_finish_quest}, lua::{AvatarFilterLua, hot_reload_quests, insert_questlog_api}}}};
pub struct QuestsPlugin {
    quests_path: PathBuf,
}

impl QuestsPlugin {
    pub fn new(quests_path: PathBuf) -> Self {
        Self { quests_path }
    }
}

// (unused) QuestSettings removed

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            cleanup_quest_markers,
            hot_reload_quests,
            init_quest_entities, 
            load_questlogs_for_joined_players,
            quest_accepter,
            quest_abandoner,
            quest_returner,
        ));

        app.add_systems(Update, (
            transmit_questlog,
            handle_quest_state_changes,
            handle_quest_condition_update,
            attach_active_quests,
            attach_or_detach_quest_on_state_change,
            detach_from_despawned_player,
            sync_quest_state.after(handle_quest_state_changes), 
            (
                update_available_quests, 
                update_quest_markers, 
                sync_quest_markers,
                quest_segue_handler,
            ).chain().after(handle_quest_state_changes),
        ));

        app.add_message::<QuestStateUpdated>();
        app.add_message::<QuestConditionUpdate>();
        app.add_message::<AcceptQuest>();
        app.add_message::<AbandonQuest>();
        app.add_message::<ReturnQuest>();
        app.add_message::<RequestNextQuest>();

        app.register_command("accept_quest", command_accept_quest);
        app.register_command("complete_quest", command_complete_quest);
        app.register_command("finish_quest", command_finish_quest);
        app.register_command("fail_quest", command_fail_quest);

        app.register_message_handler(handle_quest_request);
        app.register_message_handler(handle_quest_action_request);

        app.insert_resource(QuestRegistry::default());

        insert_questlog_api(app.world_mut()).unwrap();

        let quests_path = self.quests_path.clone();

        app.add_systems(OnEnter(InstanceState::Initializing), 
            move |
                _instance: Res<ZoneInstance>,
                mut runtime: ResMut<LuaRuntime>,
                mut commands: Commands,
            | {
                // We probably shouldn't load all quests on a per-instance basis.
                // Refactor to cache the majority of them once for the service and only load 
                // those quests that are needed for the specific map.
                info!("Loading quests from {:?}", quests_path);
                match quests_path.read_dir() {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            match runtime.load_script(&format!("quests.{}", entry.path().file_stem().unwrap().to_str().unwrap())) {
                                Ok(quest) => {
                                    let Ok(init_fn) = quest.get::<Function>("Init") else {
                                        continue;
                                    };

                                    commands.call_lua_method(init_fn, quest);
                                }
                                Err(err) => {
                                    error!("Failed to load quest {:?}: {:?}", entry.path(), err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to read quest directory: {:?}", err);
                    }
                }
            });
    }
}

#[derive(Message)]
pub struct AcceptQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct AbandonQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct FailQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct ReturnQuest {
    pub player: Entity,
    pub quest_id: i32,
}

pub struct Quest {
    table: Table,
    id: i32,
    owned: bool,
    world_def: Arc<WorldDef>,
}

#[derive(Resource, Default)]
pub struct QuestRegistry(pub HashMap<i32, Arc<Quest>>);

#[derive(Clone, Copy)]
pub enum QuestState {
    Available,
    Abandoned,
    Accepted,
    Failed,
    Completed,
    Finished,
}

#[derive(Message)]
pub struct QuestStateUpdated {
    pub player: Entity,
    pub quest_id: i32,
    pub state: QuestState,
}

#[derive(Message, Clone, Copy)]
pub struct QuestConditionUpdate {
    pub player: Entity,
    pub quest_id: i32,
    pub condition_id: i32,
    pub update: ConditionUpdate,
}

#[derive(Clone, Copy)]
pub enum ConditionUpdate {
    Added(i32),
    Removed(i32),
    Set(i32),
}

#[derive(Message)]
pub struct RequestNextQuest {
    pub player: Entity,
}

#[derive(Component)]
pub struct QuestEntity {
    visible_on_available: Vec<i32>,
    visible_on_complete: Vec<i32>,
    visible_on_finished: Vec<i32>,
    visible_on_in_progress: Vec<i32>,
}

impl QuestEntity {
    pub fn is_visible(&self, log: &QuestLog) -> bool {
        for id in &self.visible_on_available {
            if log.available.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_complete {
            if log.completed.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_finished {
            if log.finished.contains(id) {
                return true;
            }
        }

        for id in &self.visible_on_in_progress {
            if log.in_progress.contains(id) {
                return true;
            }
        }

        false
    }
}

pub struct QuestProgress {
    template: Arc<Quest>,
    state: Option<realm_api::QuestState>,
}

fn init_quest_entities(
    objects: Query<(Entity, &GameObjectData), Added<NonClientBaseTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in objects.iter() {
        let quest_entity = QuestEntity {
            visible_on_available: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestAvailable)
                .unwrap_or(&vec![]).clone(),
            visible_on_complete: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestComplete)
                .unwrap_or(&vec![]).clone(),
            visible_on_finished: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestFinished)
                .unwrap_or(&vec![]).clone(),
            visible_on_in_progress: obj.get::<_, Vec<i32>>(NonClientBase::VisibleOnQuestInProgress)
                .unwrap_or(&vec![]).clone(),
        };

        if !quest_entity.visible_on_available.is_empty() ||
            !quest_entity.visible_on_complete.is_empty() ||
            !quest_entity.visible_on_finished.is_empty() ||
            !quest_entity.visible_on_in_progress.is_empty()
        {
            commands.entity(ent)
                .insert(quest_entity);
        }
    }
}


fn sync_quest_state(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<&PlayerController>,
) {
    for &QuestStateUpdated { player, state, quest_id } in events.read() {
        let Ok(player_controller) = players.get(player) else {
            continue;
        };

        match state {
            QuestState::Available => {
                debug!("Player {} updated quest {} to available", player_controller.character_id(), quest_id);
            },
            QuestState::Abandoned => {
                debug!("Player {} abandoned quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestAbandoned,
                    ..Default::default()
                });
            },
            QuestState::Accepted => {
                debug!("Player {} accepted quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestAccepted,
                    ..Default::default()
                });
            },
            QuestState::Failed => {
                debug!("Player {} failed quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestFailed,
                    ..Default::default()
                });
            },
            QuestState::Completed => {
                debug!("Player {} completed quest {}", player_controller.character_id(), quest_id);
            },
            QuestState::Finished => {
                debug!("Player {} finished quest {}", player_controller.character_id(), quest_id);

                player_controller.send_packet(oaPktQuestEvent {
                    field_1: player_controller.avatar_id(),
                    quest_id,
                    event: OaPktQuestEventEvent::QuestFinished,
                    ..Default::default()
                });
            },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_quest_markers(
    mut interest_events: MessageReader<InterestTransmitted>,
    updated_questlogs: Query<Entity, Changed<QuestLog>>,
    players: Query<(&QuestLog, &ScriptObject, &Interests), With<PlayerTag>>,
    entities: Query<&ScriptObject, With<NonClientBaseTag>>,
    mut commands: Commands,
) {
    // Check all interests if questlog changed
    for player_ent in updated_questlogs.iter() {
        let Ok((quest_log, player_script, interests)) = players.get(player_ent) else {
            warn!("Player not yet fully initialized!");
            continue;
        };

        for (_, QuestProgress { template: quest, .. }) in quest_log.quests.iter() {
            if !quest.owned {
                continue;
            }

            let Ok(func) = quest.table.get::<Function>("UpdateQuestMarker") else {
                error!("Failed to get UpdateQuestMarker function for quest: {}", quest.id);
                continue;
            };

            for (&interest, (_, state)) in interests.iter() {
                if !matches!(state, InterestState::Transmitted) {
                    continue;
                }

                let Ok(interest_script) = entities.get(interest) else {
                    continue;
                };

                commands
                    .call_lua_method(func.clone(), (quest.table.clone(), player_script.object().clone(), interest_script.object().clone()));
            }
        }
    }

    // Check newly added interests
    for &InterestTransmitted(player, target) in interest_events.read() {
        let Ok((quest_log, player_script, _)) = players.get(player) else {
            continue;
        };

        let Ok(interest_script) = entities.get(target) else {
            continue;
        };

        for (_, QuestProgress { template: quest, .. }) in quest_log.quests.iter() {
            if !quest.owned {
                continue;
            }

            let Ok(func) = quest.table.get::<Function>("UpdateQuestMarker") else {
                continue;
            };

            commands
                .call_lua_method(func.clone(), (quest.table.clone(), player_script.object().clone(), interest_script.object().clone()));
        }

    }
}

#[allow(clippy::type_complexity)]
fn update_available_quests(
    mut events: MessageReader<QuestStateUpdated>,
    mut players: ParamSet<(
        Query<(&mut QuestLog, &ScriptObject)>,
        Query<Entity, Added<QuestLog>>,
    )>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, state, .. } in events.read() {
        if  (
                matches!(state, QuestState::Finished) ||
                matches!(state, QuestState::Completed) ||
                matches!(state, QuestState::Accepted) ||
                matches!(state, QuestState::Abandoned)
            ) &&
            let Ok((mut questlog, script_object)) = players.p0().get_mut(player)
        {
            debug!("Testing available quests for player: {}", player);

            // Clear all available quests
            questlog.available.clear();
            questlog.quests.retain(|_, q| q.state.is_some());

            // Check quest availability
            for (quest_id, quest) in quests.0.iter() {
                if questlog.quests.contains_key(quest_id) || !quest.owned {
                    continue;
                }

                let Ok(func) = quest.table.get::<Function>("MarkAvailableConditional") else {
                    error!("Failed to get MarkAvailableConditional function for quest: {}", quest.id);
                    continue;
                };

                commands.call_lua_method(
                    func, 
                    (quest.table.clone(), script_object.object().clone())
                );
            }
        }
    }

    let newly_loaded: Vec<Entity> = players.p1().iter().collect();
    for player in newly_loaded {
        if let Ok((mut questlog, script_object)) = players.p0().get_mut(player) {
            debug!("Testing available quests for newly loaded questlog: {}", player);

            // Clear all available quests
            questlog.available.clear();
            questlog.quests.retain(|_, q| q.state.is_some());

            // Check quest availability
            for (quest_id, quest) in quests.0.iter() {
                if questlog.quests.contains_key(quest_id) || !quest.owned {
                    continue;
                }

                let Ok(func) = quest.table.get::<Function>("MarkAvailableConditional") else {
                    error!("Failed to get MarkAvailableConditional function for quest: {}", quest.id);
                    continue;
                };

                commands.call_lua_method(
                    func, 
                    (quest.table.clone(), script_object.object().clone())
                );
            }
        }
    }
}

fn load_questlogs_for_joined_players(
    players: Query<(Entity, &PlayerController), (Added<ScriptObject>, With<PlayerTag>)>,
    mut commands: Commands,
) {
    for (entity, controller) in players.iter() {
        let controller = controller.clone();

        commands
            .entity(entity)
            .perform_async_operation(async move {
                let mut res = RealmApi::get()
                    .query_quest_states()
                    .character_id(controller.character_id())
                    .query()
                    .await?;

                let mut quests = vec![];

                while let Some(quest_state) = res.try_next().await? {
                    quests.push(quest_state);
                }

                Ok(quests)
            })
            .on_finish_run_system(insert_questlog_for_player)
            .on_error_run_system(player_error_handler_system);
    }
}

fn insert_questlog_for_player(
    In((entity, quests)): In<(Entity, Vec<realm_api::QuestState>)>,
    quest_registry: Res<QuestRegistry>,
    mut commands: Commands,
) {
    let mut quest_log = QuestLog {
        quests: quests.into_iter()
            .filter_map(|q| {
                let quest = quest_registry.0.get(&q.quest_id)?.clone();
                    Some((q.quest_id, QuestProgress {
                        template: quest,
                        state: Some(q),
                    }))
                })
                .collect(),
            ..Default::default()
    };

    quest_log.update_fast_access_maps();

    commands.entity(entity)
        .insert(quest_log);
}

fn handle_db_quest_update(
    In((player, (quest_id, db_state))): In<(Entity, (i32, Option<realm_api::QuestState>))>,
    mut players: Query<(&ScriptObject, &PlayerController, &mut QuestLog)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    let Ok((script_object, controller, mut quest_log)) = players.get_mut(player) else {
        return;
    };

    let Some(quest_template) = quests.0.get(&quest_id) else {
        error!("Received quest update for unknown quest id: {}", quest_id);
        return;
    };

    if 
        let Some(state) = &db_state &&
        let Some(progress) = quest_log.quests.get_mut(&quest_id) 
    {
        if progress.state.as_ref().map(|s| s.state != state.state).unwrap_or(true) {
            // Quest state changed
            let quest_state = match state.state {
                QuestProgressionState::Active => QuestState::Accepted,
                QuestProgressionState::Completed => QuestState::Completed,
                QuestProgressionState::Finished => QuestState::Finished,
                QuestProgressionState::Failed => QuestState::Failed,
            };

            commands.write_message(QuestStateUpdated {
                player,
                quest_id,
                state: quest_state,
            });
        }

        controller.send_packet(oaPktQuestUpdate {
            player: controller.avatar_id(),
            quest_id: state.quest_id as u32,
            entry_count: state.conditions.len() as u32,
            quest_failed: false,
            accepted_time: state.accepted_time.timestamp_millis(),
            conditions: state.conditions.iter()
                .map(|&QuestCondition { id, current_count, .. }| QuestUpdateData {
                    condition_id: id,
                    count: current_count,
                    ..Default::default()
                }).collect(),
            ..Default::default()
        });

        progress.state = db_state;
        quest_log.update_fast_access_maps();
    } else if 
        db_state.is_none() &&
        let Some(progress) = quest_log.quests.remove(&quest_id) 
    {
        if progress.state.is_some() {
            // Quest was removed from DB, but player had it in progress
            // Mark it as abandoned
            commands.write_message(QuestStateUpdated {
                player,
                quest_id,
                state: QuestState::Abandoned,
            });
        }

        quest_log.update_fast_access_maps();
    }

    let Ok(func) = quest_template.table.get::<Function>("StateUpdated") else {
        error!("Failed to get StateUpdated function for quest: {}", quest_template.id);
        return;
    };

    commands.call_lua_method(
        func,
        (quest_template.table.clone(), script_object.object().clone())
    );
}

fn transmit_questlog(
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

fn sync_quest_markers(
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

fn send_quest(_lua: &mlua::Lua, controller: &PlayerController, quest: &Quest, zone: &ZoneInstance, beacon_query: &Query<&GameObjectData>) {
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

fn handle_quest_request(
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

#[derive(Component)]
pub struct AttachedQuest { quest_id: i32 }

#[derive(Component)]
pub struct QuestAvailable;

#[derive(Component)]
pub struct QuestPlayer(pub Entity);

fn quest_accepter(
    mut events: MessageReader<AcceptQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    instance: Res<ZoneInstance>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &AcceptQuest { player, quest_id } in events.read() {
        let Ok((questlog, player_controller)) = players.get(player) else {
            continue;
        };

        let Some(quest) = quests.0.get(&quest_id).cloned() else {
            warn!("Player {} tried to accept unknown quest {}", player, quest_id);
            continue;
        };

        debug!("Player {} is trying to accept quest {}", player, quest_id);

        if questlog.available.contains(&quest_id) {
            let controller = player_controller.clone();

            let mut state = RealmApi::get()
                .create_empty_queststate(controller.character_id(), quest_id, QuestProgressionState::Active);

            // Init quest conditions on accept
            if let Ok(conditions) = quest.table.get::<Table>("conditions") {
                for pairs in conditions.pairs::<Value, Table>() {
                    let Ok((_, condition)) = pairs else {
                        continue;
                    };

                    state.conditions.push(QuestCondition {
                        id: condition.get::<i32>("id").unwrap_or(0),
                        required_count: condition.get::<i32>("required_count").unwrap_or(1),
                        current_count: 0,
                    });
                }
            }

            commands
                .entity(player)
                .perform_async_operation(async move {
                    Ok((
                        quest_id,
                        Some(RealmApi::get().create_queststate(&state).await?)
                    ))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to accept unavailable quest {}", player, quest_id);
        }
    }
}

fn quest_returner(
    mut events: MessageReader<ReturnQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    mut commands: Commands,
) {
    for &ReturnQuest { player, quest_id } in events.read() {
        let Ok((questlog, player_controller)) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest) = questlog.quests.get(&quest_id) &&
            let Some(state) = &quest.state &&
            matches!(state.state, QuestProgressionState::Completed)
        {
            let controller = player_controller.clone();
            let mut state = state.clone();

            commands
                .entity(player)
                .perform_async_operation(async move {
                    state.update_state(QuestProgressionState::Finished).await?;

                    Ok((
                        quest_id,
                        Some(state)
                    ))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to finish uncompleted quest {}", player, quest_id);
        }
    }
}

fn quest_abandoner(
    mut events: MessageReader<AbandonQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    mut commands: Commands,
) {
    for &AbandonQuest { player, quest_id } in events.read() {
        let Ok((questlog, player_controller)) = players.get(player) else {
            continue;
        };

        if 
            let Some(quest) = questlog.quests.get(&quest_id) &&
            let Some(state) = &quest.state &&
            (
                matches!(state.state, QuestProgressionState::Active) ||
                matches!(state.state, QuestProgressionState::Completed)
            )
        {
            let controller = player_controller.clone();
            let state = state.clone();

            commands
                .entity(player)
                .perform_async_operation(async move {
                    state.delete().await?;
                    Ok((quest_id, None))
                })
                .on_finish_run_system(handle_db_quest_update)
                .on_error_run_system(player_error_handler_system);
        } else {
            warn!("Player {} tried to abandon unstarted quest {}", player, quest_id);
        }
    }
}

fn handle_quest_state_changes(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<&ScriptObject>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, quest_id, state } in events.read() {
        let Ok(script_object) = players.get(player) else {
            continue;
        };

        let Some(quest) = quests.0.get(&quest_id) else {
            continue;
        };

        let func_name = match state {
            QuestState::Available => "OnQuestAvailable",
            QuestState::Accepted => "OnQuestAccepted",
            QuestState::Completed => "OnQuestCompleted",
            QuestState::Finished => "OnQuestFinished",
            QuestState::Failed => "OnQuestFailed",
            QuestState::Abandoned => "OnQuestAbandoned",
        };

        if let QuestState::Finished = state {
            commands.write_message(RequestNextQuest { player });
        }

        let Ok(func) = quest.table.get::<Function>(func_name) else {
            continue;
        };

        commands.call_lua_method(
            func, 
            (quest.table.clone(), script_object.object().clone())
        );
    }
}

fn handle_quest_action_request(
    In((ent, pkt)): In<(Entity, oaPktRequestQuestAction)>,
    avatar_id_manager: Res<AvatarIdManager>,
    objects: Query<&ScriptObject>,
    mut commands: Commands,
) {
    if 
        let Some(target_ent) = avatar_id_manager.resolve_avatar_id(pkt.target) &&
        let Ok(target) = objects.get(target_ent)
    {
        commands
            .entity(ent)
            .call_named_lua_method("RequestInteraction", (
                if let OaPktRequestQuestActionKind::Interact = pkt.kind {
                    "interact"
                } else {
                    warn!("Unknown quest action kind: {:?}", pkt.kind);
                    "unknown"
                },
                target.object().clone()
            ));
    }    
}

fn handle_quest_condition_update(
    mut events: MessageReader<QuestConditionUpdate>,
    players: Query<(&QuestLog, &PlayerController)>,
    mut commands: Commands,
) {
    for &QuestConditionUpdate { player, quest_id, condition_id, update } in events.read() {
        let Ok((quest_log, controller)) = players.get(player) else {
            continue;
        };

        let Some(mut quest_state) = quest_log.quests.get(&quest_id).and_then(|q| q.state.clone()) else {
            continue;
        };

        let controller = controller.clone();

        commands
            .entity(player)
            .perform_async_operation(async move {
                quest_state.update_condition(condition_id, 
                    match update {
                        ConditionUpdate::Added(_) => realm_api::ConditionUpdate::Increment,
                        ConditionUpdate::Removed(_) => realm_api::ConditionUpdate::Increment,
                        ConditionUpdate::Set(_) => realm_api::ConditionUpdate::Set,
                    },
                    match update {
                        ConditionUpdate::Added(v) => v,
                        ConditionUpdate::Removed(v) => -v,
                        ConditionUpdate::Set(v) => v,
                    }).await?;

                Ok((quest_id, Some(quest_state)))
            })
            .on_finish_run_system(handle_db_quest_update)
            .on_error_run_system(player_error_handler_system);
    }
}

fn quest_segue_handler(
    mut events: MessageReader<RequestNextQuest>,
    query: Query<(&ScriptObject, &DialogueState), With<PlayerTag>>,
    mut commands: Commands,
) {
    for &RequestNextQuest { player } in events.read() {
        if let Ok((script_object, dialogue_state)) = query.get(player) {
            commands.entity(dialogue_state.speaker)
                .call_named_lua_method("RequestDialogue", script_object.object().clone());
        }
    }
}

fn cleanup_quest_markers(
    mut removed_players: RemovedComponents<PlayerTag>,
    markers: Query<(Entity, &QuestPlayer)>,
    mut commands: Commands,
) {
    for player in removed_players.read() {
        debug!("Cleaning up quest markers for removed player: {}", player);

        for (marker, quest_player) in markers.iter() {
            if quest_player.0 == player {
                commands.entity(marker).despawn();
            }
        }
    }
}

fn attach_active_quests(
    players: Query<(&QuestLog, &ScriptObject), Added<QuestLog>>,
    mut commands: Commands,
) {
    for (quest_log, script_object) in players.iter() {
        for (_, progress) in quest_log.quests.iter() {
            if progress.state.as_ref().map(|s| s.state) == Some(QuestProgressionState::Active) {
                let Ok(attach_quest_to_player) = progress.template.table.get::<Function>("AttachToPlayer") else {
                    error!("Failed to get AttachToPlayer function for quest: {}", progress.template.id);
                    continue;
                };

                // Newly accepted quest, attach to player
                commands.call_lua_method(
                    attach_quest_to_player.clone(),
                    (progress.template.table.clone(), script_object.object().clone())
                );
            }
        }
    }
}

fn attach_or_detach_quest_on_state_change(
    mut events: MessageReader<QuestStateUpdated>,
    players: Query<(&ScriptObject, &PlayerController)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, quest_id, state } in events.read() {
        let Ok((script_object, _controller)) = players.get(player) else {
            continue;
        };

        let Some(quest_template) = quests.0.get(&quest_id) else {
            error!("Received quest update for unknown quest id: {}", quest_id);
            return;
        };

        match state {
            QuestState::Accepted => {
                let Ok(attach_quest_to_player) = quest_template.table.get::<Function>("AttachToPlayer") else {
                    error!("Failed to get AttachToPlayer function for quest: {}", quest_template.id);
                    return;
                };

                // Newly accepted quest, attach to player
                commands.call_lua_method(
                    attach_quest_to_player.clone(),
                    (quest_template.table.clone(), script_object.object().clone())
                );
            },
            QuestState::Completed | QuestState::Finished | QuestState::Failed | QuestState::Abandoned => {
                let Ok(detach_quest_from_player) = quest_template.table.get::<Function>("DetachFromPlayer") else {
                    error!("Failed to get DetachFromPlayer function for quest: {}", quest_template.id);
                    return;
                };

                // Quest finished/failed/abandoned, detach from player
                commands.call_lua_method(
                    detach_quest_from_player.clone(),
                    (quest_template.table.clone(), script_object.object().clone())
                );
            },
            _ => { /* No action needed for other states */ }
        }
    }
}

fn detach_from_despawned_player(
    mut events: MessageReader<DespawnAvatar>,
    players: Query<(&ScriptObject, &QuestLog)>,
    mut commands: Commands,
) {
    for &DespawnAvatar(ent) in events.read() {
        let Ok((script_object, quest_log)) = players.get(ent) else {
            continue;
        };

        quest_log.quests.iter()
            .filter(|(_, progress)| progress.state.as_ref().map(|s| s.state) == Some(QuestProgressionState::Active))
            .for_each(|(_, progress)| {
                let Ok(detach_quest_from_player) = progress.template.table.get::<Function>("DetachFromPlayer") else {
                    error!("Failed to get DetachFromPlayer function for quest: {}", progress.template.id);
                    return;
                };

                // Player despawned, detach active quests
                commands.call_lua_method(
                    detach_quest_from_player.clone(),
                    (progress.template.table.clone(), script_object.object().clone())
                );
            });
    }
}
