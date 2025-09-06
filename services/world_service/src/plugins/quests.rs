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

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use bevy::{app::{Plugin, PostUpdate, PreUpdate}, ecs::{event::{Event, EventReader, EventWriter}, hierarchy::{ChildOf, Children}, query::{Changed, Or, With}, removal_detection::{RemovedComponentReader, RemovedComponents}, resource::Resource, schedule::IntoScheduleConfigs, system::{In, Res, SystemId}, world::World}, platform::collections::HashSet, prelude::{Added, App, Commands, Component, Entity, Query}, tasks::block_on};
use futures::TryStreamExt;
use log::{debug, error, info};
use mlua::{AsChunk, Function, IntoLua, Lua, Table};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase};
use protocol::{oaPktQuestEvent, oaPktQuestGiverStatus, oaPktQuestRequest, oaQuestTemplate, CPktStream_165_2, CPktStream_165_7, OaPktQuestEventEvent, OaPktQuestRequestRequest};
use realm_api::{QuestProgressionState, WorldDef};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use anyhow::anyhow;
use tokio::task::block_in_place;
use toolkit::NativeParam;

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance, plugins::{AvatarInfo, CommandExtPriv, ContentInfo, FutureCommands, InterestAdded, InterestState, InterestTransmitted, Interests, NetworkExtPriv, PlayerController}};
pub struct QuestsPlugin {
    quests_path: PathBuf
}

impl QuestsPlugin {
    pub fn new(quests_path: PathBuf) -> Self {
        Self { quests_path }
    }
}

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (init_quest_entities, load_questlogs_for_joined_players));
        app.add_systems(PostUpdate, (
            (sync_quest_state, update_available_quests).chain(),
            update_quest_markers,
            sync_quest_markers,
            send_quest_updates
        ));

        app.add_event::<QuestStateUpdated>();
        app.add_event::<AcceptQuest>();
        app.add_event::<AbandonQuest>();
        app.add_event::<ReturnQuest>();

        app.register_command("accept_quest", command_accept_quest);
        app.register_command("complete_quest", command_complete_quest);
        app.register_command("finish_quest", command_finish_quest);
        app.register_command("fail_quest", command_fail_quest);

        app.register_message_handler(handle_quest_request);

        let mut quests = HashMap::new();
        let instance_manager = app.world().get_resource::<ZoneInstance>().unwrap().manager.clone();
        let world_def = app.world().get_resource::<ZoneInstance>().unwrap().world_def.clone();
        let mut runtime = app.world_mut().get_resource_mut::<LuaRuntime>().unwrap();

        // We probably shouldn't load all quests on a per-instance basis.
        // Refactor to cache the majority of them once for the service and only load 
        // those quests that are needed for the specific map.
        info!("Loading quests from {:?}", self.quests_path);
        match self.quests_path.read_dir() {
            Ok(entries) => {
                for entry in entries.flatten() {
                    match runtime.load_script(&format!("quests.{}", entry.path().file_stem().unwrap().to_str().unwrap())) {
                        Ok(quest) => {
                            let info: Result<(i32, String), mlua::Error> = try {
                                (
                                    quest.get::<i32>("id")?,
                                    quest.get::<String>("world")?,
                                )
                            };

                            match info {
                                Ok((id, world)) => {
                                    if quests.contains_key(&id) {
                                        error!("Duplicate quest id {} in file {:?}", id, entry.path());
                                        continue;
                                    }

                                    let Some(world_def) = block_in_place(|| instance_manager.get_world_def_by_name(&world)) else {
                                        error!("Quest in file {:?} references unknown world: {}", entry.path(), world);
                                        continue;
                                    };

                                    quests.insert(id, Arc::new(Quest {
                                        id,
                                        table: quest,
                                        owned: world == world_def.name(),
                                        world_def,
                                    }));
                                },
                                Err(err) => {
                                    error!("Quest in file {:?} does not have a valid id: {:?}", entry.path(), err);
                                }
                            }
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

        info!("Loaded {} quests", quests.len());

        app.insert_resource(QuestRegistry(quests));

        let quest_systems = QuestSystems {
            insert_questlog_for_player: app.register_system(insert_questlog_for_player),
        };

        app.insert_resource(quest_systems);

        insert_questlog_api(app.world_mut()).unwrap();
    }
}

#[derive(Event)]
pub struct AcceptQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Event)]
pub struct AbandonQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Event)]
pub struct ReturnQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Resource)]
#[allow(clippy::type_complexity)]
struct QuestSystems {
    insert_questlog_for_player: SystemId<In<Option<(Entity, Vec<realm_api::QuestState>)>>>,
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
    Initialized,
    Available,
    Abandoned,
    Progression(QuestProgressionState),
}

#[derive(Clone, Copy)]
pub enum QuestUpdateScope {
    Player,
    Quest(i32)
}

#[derive(Event)]
pub struct QuestStateUpdated {
    pub player: Entity,
    pub scope: QuestUpdateScope,
    pub state: QuestState,
}

#[derive(Event)]
pub struct QuestConditionUpdated {
    pub player: Entity,
    pub quest_id: i32,
    pub condition_id: i32,
    pub added: i32,
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
    quest: Arc<Quest>,
    state: Option<realm_api::QuestState>,
}

#[derive(Component, Default)]
pub struct QuestLog {
    // Quests ids per status, for fast access when determining interests
    available: HashSet<i32>,
    completed: HashSet<i32>,
    finished: HashSet<i32>,
    in_progress: HashSet<i32>,

    quests: HashMap<i32, QuestProgress>,
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
    mut events: EventReader<QuestStateUpdated>,
    mut players: Query<(&mut QuestLog, &PlayerController)>,
    quests: Res<QuestRegistry>,
    instance: Res<ZoneInstance>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, scope, state} in events.read() {
        let Ok((mut quest_log, player_controller)) = players.get_mut(player) else {
            continue;
        };

        let QuestUpdateScope::Quest(quest_id) = scope else {
            continue;
        };

        let Some(quest) = quests.0.get(&quest_id) else {
            error!("Player {} tried to update unknown quest {}", player_controller.character_id(), quest_id);
            continue;
        };
        
        let quest =  quest_log.quests
            .entry(quest_id)
            .or_insert_with(|| QuestProgress {
                quest: quest.clone(),
                state: None,
            });

        match state {
            QuestState::Initialized => unreachable!(),
            QuestState::Available => {
                debug!("Player {} updated quest {} to available", player_controller.character_id(), quest_id);

                // If we've already got a state, send an abandoned event instead
                if quest.state.is_some() {
                    commands.send_event(QuestStateUpdated {
                        player,
                        scope: QuestUpdateScope::Quest(quest_id),
                        state: QuestState::Abandoned,
                    });
                    continue;
                }

                quest_log.available.insert(quest_id);
                quest_log.completed.remove(&quest_id);
                quest_log.in_progress.remove(&quest_id);
                quest_log.finished.remove(&quest_id);
            },
            QuestState::Abandoned => {
                if let Some(quest_state_tracker) = quest.state.take() {
                    let player_controller = player_controller.clone();

                    instance.spawn_task(async move {
                        if let Err(e) = quest_state_tracker.delete().await {
                            error!("Failed to delete quest state: {}", e);
                            player_controller.close();
                        }
                    });

                    quest_log.available.remove(&quest_id);
                    quest_log.completed.remove(&quest_id);
                    quest_log.in_progress.remove(&quest_id);
                    quest_log.finished.remove(&quest_id);
                }
            },
            QuestState::Progression(state) => {
                let quest_state_tracker = quest.state.get_or_insert_with(|| instance.realm_api.create_empty_queststate(
                    player_controller.character_id(), 
                    quest_id, 
                    state
                ));

                quest_state_tracker.state = state;
                let quest_state_tracker = quest_state_tracker.clone();

                match state {
                    QuestProgressionState::Active => {
                        player_controller.send_packet(oaPktQuestEvent {
                            field_1: player_controller.avatar_id(),
                            quest_id,
                            event: OaPktQuestEventEvent::QuestAccepted,
                            ..Default::default()
                        });

                        quest_log.in_progress.insert(quest_id);
                        quest_log.available.remove(&quest_id);
                        quest_log.completed.remove(&quest_id);
                        quest_log.finished.remove(&quest_id);
                    },
                    QuestProgressionState::Failed => {
                        player_controller.send_packet(oaPktQuestEvent {
                            field_1: player_controller.avatar_id(),
                            quest_id,
                            event: OaPktQuestEventEvent::QuestFailed,
                            ..Default::default()
                        });

                        quest_log.in_progress.remove(&quest_id);
                        quest_log.available.remove(&quest_id);
                        quest_log.completed.remove(&quest_id);
                        quest_log.finished.remove(&quest_id);
                    },
                    QuestProgressionState::Finished => {
                        player_controller.send_packet(oaPktQuestEvent {
                            field_1: player_controller.avatar_id(),
                            quest_id,
                            event: OaPktQuestEventEvent::QuestFinished,
                            ..Default::default()
                        });

                        quest_log.finished.insert(quest_id);
                        quest_log.in_progress.remove(&quest_id);
                        quest_log.available.remove(&quest_id);
                        quest_log.completed.remove(&quest_id);
                    },
                    _ => (),
                }

                // Store quest state
                let realm_api = instance.realm_api.clone();
                let controller = player_controller.clone();
                

                instance.spawn_task(async move {
                    let res: Result<(), realm_api::RealmApiError> = try {
                        if let Some(mut quest_state) = realm_api.query_quest_states()
                            .character_id(quest_state_tracker.character_id)
                            .quest_id(quest_state_tracker.quest_id)
                            .query().await?
                            .try_next().await?
                        {
                            quest_state.state = quest_state_tracker.state;
                            quest_state.save().await?;
                        } else {
                            realm_api.create_queststate(&quest_state_tracker).await?;
                        }
                    };

                    if let Err(e) = res {
                        error!("Failed to update or create quest state: {}", e);
                        controller.close();
                    }
                });
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_quest_markers(
    mut interest_events: EventReader<InterestTransmitted>,
    updated_questlogs: Query<Entity, Changed<QuestLog>>,
    players: Query<(&QuestLog, &ScriptObject, &Interests), With<PlayerTag>>,
    entities: Query<&ScriptObject, With<NonClientBaseTag>>,
    mut commands: Commands,
) {
    // Check all interests if questlog changed
    for player_ent in updated_questlogs.iter() {
        let Ok((quest_log, player_script, interests)) = players.get(player_ent) else {
            continue;
        };

        for (_, QuestProgress { quest, .. }) in quest_log.quests.iter() {
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

        for (_, QuestProgress { quest, .. }) in quest_log.quests.iter() {
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

enum LuaQuestState {
    Available,
    InProgress,
    Completed,
    Finished,
    Unavailable,
}

impl IntoLua for LuaQuestState {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        match self {
            LuaQuestState::Available => 0.into_lua(lua),
            LuaQuestState::InProgress => 1.into_lua(lua),
            LuaQuestState::Completed => 2.into_lua(lua),
            LuaQuestState::Finished => 3.into_lua(lua),
            LuaQuestState::Unavailable => 4.into_lua(lua),
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn insert_questlog_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("questlog", object_api.clone()).unwrap();

    object_api.set("MarkQuestAvailable", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut events: EventWriter<QuestStateUpdated>,
    | -> WorldResult<()> {
        events.write(QuestStateUpdated {
            player: owner.entity()?,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Available,
        });

        Ok(())
    })?)?;

    object_api.set("MarkQuestCompleted", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut events: EventWriter<QuestStateUpdated>,
    | -> WorldResult<()> {
        events.write(QuestStateUpdated {
            player: owner.entity()?,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Completed),
        });

        Ok(())
    })?)?;

    object_api.set("MarkQuestFinished", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut events: EventWriter<QuestStateUpdated>,
    | -> WorldResult<()> {
        events.write(QuestStateUpdated {
            player: owner.entity()?,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Finished),
        });

        Ok(())
    })?)?;

    object_api.set("MarkQuestInProgress", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut events: EventWriter<QuestStateUpdated>,
    | -> WorldResult<()> {
        events.write(QuestStateUpdated {
            player: owner.entity()?,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Active),
        });

        Ok(())
    })?)?;

    object_api.set("MarkQuestFailed", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut events: EventWriter<QuestStateUpdated>,
    | -> WorldResult<()> {
        events.write(QuestStateUpdated {
            player: owner.entity()?,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Failed),
        });

        Ok(())
    })?)?;

    object_api.set("GetQuestState", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        questlog: Query<&QuestLog>,
    | -> WorldResult<LuaQuestState> {
        let log = questlog.get(owner.entity()?)
            .map_err(|e| WorldError::Other(anyhow!("Failed to get quest log: {}", e)))?;

        if log.completed.contains(&quest_id) {
            Ok(LuaQuestState::Completed)
        } else if log.finished.contains(&quest_id) {
            Ok(LuaQuestState::Finished)
        } else if log.in_progress.contains(&quest_id) {
            Ok(LuaQuestState::InProgress)
        } else if log.available.contains(&quest_id) {
            Ok(LuaQuestState::Available)
        } else {
            Ok(LuaQuestState::Unavailable)
        }
    })?)?;

    object_api.set("UpdateQuestMarker", lua.create_bevy_function(world, |
        In((player_tbl, target_tbl, quest, state)): In<(Table, Table, Table, i32)>,
        targets: Query<&Children, With<NonClientBaseTag>>,
        markers: Query<&QuestPlayer, With<QuestMarker>>,
        mut commands: Commands,
    | -> WorldResult<()> {
        let target_entity = target_tbl.entity()?;

        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Failed to get quest id")));
        };

        let marker = 
            targets.get(target_entity)
            .ok()
            .and_then(|children| {
                children.iter()
                .filter_map(|&e| Some((e, markers.get(e).ok()?)))
                .find(|(_, quest_player)| Some(quest_player.0) == player_tbl.entity().ok())
            });

        if state == 0 && let Some((entity, _)) = marker {
            commands.entity(entity).remove::<QuestMarker>();
        } else if state != 0 && marker.is_none() {
            commands.spawn(
                (
                    QuestMarker { quest_id },
                    QuestPlayer(player_tbl.entity()?),
                    ChildOf(target_entity)
                )
            );
        }

        Ok(())
    })?)?;

    object_api.set("GetAttachedQuests", lua.create_bevy_function(world, |
        In((target, player)): In<(Table, Table)>,
        quest_target: Query<&Children, With<NonClientBaseTag>>,
        markers: Query<(&QuestPlayer, &QuestMarker)>,
        quests: Res<QuestRegistry>,
    | -> WorldResult<Vec<Table>> {
        let children = quest_target.get(target.entity()?)
            .map_err(|e| WorldError::Other(anyhow!("Failed to get quest target children: {}", e)))?;

        let mut result = vec![];
        for &child in children.iter() {
            if 
                let Ok((quest_player, quest_marker)) = markers.get(child) &&
                quest_player.0 == player.entity()? &&
                let Some(quest) = quests.0.get(&quest_marker.quest_id)
            {
                result.push(quest.table.clone());
            }
        }

        Ok(result)
    })?)?;

    Ok(())
}

fn command_accept_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut quest_update: EventWriter<QuestStateUpdated>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        quest_update.write(QuestStateUpdated {
            player: ent,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Active),
        });
    }
}

fn command_complete_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut quest_update: EventWriter<QuestStateUpdated>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        quest_update.write(QuestStateUpdated {
            player: ent,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Completed),
        });
    }
}

fn command_finish_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut quest_update: EventWriter<QuestStateUpdated>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        quest_update.write(QuestStateUpdated {
            player: ent,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Finished),
        });
    }
}

fn command_fail_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut quest_update: EventWriter<QuestStateUpdated>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        quest_update.write(QuestStateUpdated {
            player: ent,
            scope: QuestUpdateScope::Quest(quest_id),
            state: QuestState::Progression(QuestProgressionState::Failed),
        });
    }
}

fn update_available_quests(
    mut events: EventReader<QuestStateUpdated>,
    mut players: Query<(&mut QuestLog, &ScriptObject)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for &QuestStateUpdated { player, state, .. } in events.read() {
        if  (
                matches!(state, QuestState::Progression(QuestProgressionState::Finished)) ||
                matches!(state, QuestState::Progression(QuestProgressionState::Completed)) ||
                matches!(state, QuestState::Progression(QuestProgressionState::Active)) ||
                matches!(state, QuestState::Initialized)
            ) &&
            let Ok((mut questlog, script_object)) = players.get_mut(player)
        {
            debug!("Testing available quests for player: {}", player);

            // Clear all available quests
            questlog.available.clear();
            questlog.quests.retain(|_, q| q.state.is_none());

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
    instance: Res<ZoneInstance>,
    systems: Res<QuestSystems>,
    mut commands: Commands,
) {
    for (entity, controller) in players.iter() {
        let controller = controller.clone();
        let realm_api = instance.realm_api.clone();

        commands.run_system_async(async move {
            let Ok(mut res) = realm_api.query_quest_states()
                .character_id(controller.character_id())
                .query()
                .await 
            else {
                controller.close();
                return None;
            };

            let mut quests = vec![];

            loop {
                match res.try_next().await {
                    Ok(Some(quest_state)) => {
                        quests.push(quest_state);
                    },
                    Ok(None) => break,
                    Err(e) => {
                        error!("Failed to load quest states for player {}: {}", controller.character_id(), e);
                        controller.close();
                        return None;
                    }
                }
            }

            Some((entity, quests))
        }, systems.insert_questlog_for_player);
    }
}

fn insert_questlog_for_player(
    In(data): In<Option<(Entity, Vec<realm_api::QuestState>)>>,
    quest_registry: Res<QuestRegistry>,
    mut commands: Commands,
) {
    if let Some((entity, quests)) = data {
        commands.entity(entity)
            .insert(
                QuestLog {
                    available: HashSet::new(),
                    completed: quests.iter()
                        .filter(|q| matches!(q.state, QuestProgressionState::Completed))
                        .map(|q| q.quest_id)
                        .collect(),
                    finished: quests.iter()
                        .filter(|q| matches!(q.state, QuestProgressionState::Finished))
                        .map(|q| q.quest_id)
                        .collect(),
                    in_progress: quests.iter()
                        .filter(|q| matches!(q.state, QuestProgressionState::Active))
                        .map(|q| q.quest_id)
                        .collect(),
                    quests: quests.into_iter()
                        .filter_map(|q| {
                            let quest = quest_registry.0.get(&q.quest_id)?.clone();

                            Some((q.quest_id, QuestProgress {
                                quest,
                                state: Some(q),
                            }))
                        })
                        .collect(),
                }
            );

        commands.send_event(QuestStateUpdated {
            player: entity,
            scope: QuestUpdateScope::Player,
            state: QuestState::Initialized,
        });
    }
}

fn sync_quest_markers(
    changed_markers: Query<Entity, Changed<QuestMarker>>,
    avatars: Query<&AvatarInfo>,
    players: Query<&PlayerController>,
    markers: Query<(&ChildOf, &QuestPlayer)>,
    mut removed_markers: RemovedComponents<QuestMarker>,
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

fn handle_quest_request(
    In((ent, pkt)): In<(Entity, oaPktQuestRequest)>,
    players: Query<(&QuestLog, &PlayerController)>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    match pkt.request {
        OaPktQuestRequestRequest::Request => {
            let Some(quest) = quests.0.get(&pkt.quest_id) else {
                error!("Player {} requested unknown quest {}", ent, pkt.quest_id);
                return;
            };

            let Ok((_, player_controller)) = players.get(ent) else {
                return;
            };

            player_controller.send_packet(CPktStream_165_2 {
                field_1: oaQuestTemplate {
                    quest_id: quest.id,
                    world_guid: *quest.world_def.guid(),
                    level: quest.table.get::<i32>("level").unwrap_or(0),
                    bit_reward: quest.table.get::<i32>("bit_reward").unwrap_or(0),
                    exp_reward: quest.table.get::<i32>("exp_reward").unwrap_or(0),
                    ..Default::default()
                },
                ..Default::default()
            });
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

                pkt.quest_list[(quest_id / 8) as usize] = 1 << (quest_id % 8);
            }

            player_controller.send_packet(pkt);
        },
        OaPktQuestRequestRequest::Accept => {
            commands.send_event(AcceptQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        },
        OaPktQuestRequestRequest::Abandon => {
            commands.send_event(AbandonQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        },
        OaPktQuestRequestRequest::Return => {
            commands.send_event(ReturnQuest {
                player: ent,
                quest_id: pkt.quest_id,
            });
        }
    }
}

#[derive(Component)]
pub struct QuestMarker { quest_id: i32 }

#[derive(Component)]
pub struct QuestPlayer(Entity);

pub fn send_quest_updates(
    mut event: EventReader<QuestStateUpdated>,
    players: Query<&PlayerController>,
) {
    for &QuestStateUpdated { player, scope, state } in event.read() {
        let Ok(controller) = players.get(player) else {
            continue;
        };

        let (quest_id, event) = match scope {
            QuestUpdateScope::Player => continue,
            QuestUpdateScope::Quest(quest_id) => {
                let event = match state {
                    QuestState::Abandoned => OaPktQuestEventEvent::QuestAbandoned,
                    QuestState::Progression(QuestProgressionState::Active) => OaPktQuestEventEvent::QuestAccepted,
                    QuestState::Progression(QuestProgressionState::Finished) => OaPktQuestEventEvent::QuestFinished,
                    QuestState::Progression(QuestProgressionState::Failed) => OaPktQuestEventEvent::QuestFailed,
                    _ => continue,
                };

                (quest_id, event)
            }
        };

        controller.send_packet(oaPktQuestEvent {
            field_1: controller.avatar_id(),
            quest_id,
            event,
            ..Default::default()
        });
    }
}

fn quest_accepter(
    mut events: EventReader<AcceptQuest>,
    players: Query<&QuestLog>,
    mut commands: Commands,
) {
    for &AcceptQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if questlog.available.contains(&quest_id) {
            commands.send_event(QuestStateUpdated {
                player,
                scope: QuestUpdateScope::Quest(quest_id),
                state: QuestState::Progression(QuestProgressionState::Active),
            });
        }
    }
}

fn quest_returner(
    mut events: EventReader<ReturnQuest>,
    players: Query<&QuestLog>,
    mut commands: Commands,
) {
    for &ReturnQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if questlog.completed.contains(&quest_id) {
            commands.send_event(QuestStateUpdated {
                player,
                scope: QuestUpdateScope::Quest(quest_id),
                state: QuestState::Progression(QuestProgressionState::Finished),
            });
        }
    }
}

fn quest_abandoner(
    mut events: EventReader<AbandonQuest>,
    players: Query<&QuestLog>,
    mut commands: Commands,
) {
    for &AbandonQuest { player, quest_id } in events.read() {
        let Ok(questlog) = players.get(player) else {
            continue;
        };

        if questlog.in_progress.contains(&quest_id) {
            commands.send_event(QuestStateUpdated {
                player,
                scope: QuestUpdateScope::Quest(quest_id),
                state: QuestState::Abandoned,
            });
        }
    }
}
