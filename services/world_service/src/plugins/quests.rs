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

use std::{collections::HashMap, ops::Deref, path::PathBuf, sync::Arc};

use bevy::{app::{Plugin, PostUpdate, PreUpdate, Update}, ecs::{event::{Event, EventReader}, hierarchy::{ChildOf, Children}, query::{Changed, With}, removal_detection::RemovedComponents, resource::Resource, schedule::IntoScheduleConfigs, system::{In, ParamSet, Res, ResMut, SystemId}, world::World}, math::Vec3, platform::collections::HashSet, prelude::{Added, App, Commands, Component, Entity, Query}, state::state::OnEnter};
use chrono::{DateTime, Utc};
// use bonsai_bt::Status::Running;
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use mlua::{FromLua, Function, IntoLua, Lua, Table, Value};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase};
use protocol::{oaPktQuestEvent, oaPktQuestGiverStatus, oaPktQuestRequest, oaPktQuestUpdate, oaPktRequestQuestAction, oaQuestBeacon, oaQuestCondition, oaQuestTemplate, AvatarFilter, CPktStream_165_2, CPktStream_165_7, OaPktQuestEventEvent, OaPktQuestRequestRequest, OaPktRequestQuestActionKind, OaQuestConditionKind, QuestUpdateData};
use realm_api::{QuestCondition, QuestProgressionState, WorldDef};
use scripting::{EntityScriptCommandsExt, LuaExt, LuaRuntime, LuaScriptReloaded, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use anyhow::anyhow;
use tokio::task::block_in_place;
use toolkit::{NativeParam};

use crate::{error::{WorldError, WorldResult}, instance::{InstanceState, ZoneInstance}, plugins::{dialogue, AvatarIdManager, AvatarInfo, CommandExtPriv, DialogueState, FutureCommands, InterestState, InterestTransmitted, Interests, NetworkExtPriv, PlayerController}};
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
            sync_quest_state.after(handle_quest_state_changes), 
            (
                update_available_quests, 
                update_quest_markers, 
                sync_quest_markers,
                quest_segue_handler,
            ).chain().after(handle_quest_state_changes),
        ));

        app.add_event::<QuestStateUpdated>();
        app.add_event::<QuestConditionUpdate>();
        app.add_event::<AcceptQuest>();
        app.add_event::<AbandonQuest>();
        app.add_event::<ReturnQuest>();
        app.add_event::<RequestNextQuest>();

        app.register_command("accept_quest", command_accept_quest);
        app.register_command("complete_quest", command_complete_quest);
        app.register_command("finish_quest", command_finish_quest);
        app.register_command("fail_quest", command_fail_quest);

        app.register_message_handler(handle_quest_request);
        app.register_message_handler(handle_quest_action_request);

        let quest_systems = QuestSystems {
            insert_questlog_for_player: app.register_system(insert_questlog_for_player),
            quest_updated: app.register_system(handle_db_quest_update),
        };

        app.insert_resource(quest_systems);
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

#[derive(Clone)]
pub struct AvatarFilterLua(protocol::AvatarFilter);

impl Deref for AvatarFilterLua {
    type Target = protocol::AvatarFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromLua for AvatarFilterLua {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let table = match &value {
            mlua::Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarFilter".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let filter_type: String = table.get("type")?;
        let filter_value: String = table.get("filter")?;

        let kind = match filter_type.as_str() {
            "Content" => 1,
            "Instance" => 2,
            "QuestTags" => 3,
            "LootItem" => 4,
            "Dialog" => 5,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AvatarFilter".to_string(),
                message: Some(format!("unknown filter type: {}", filter_type)),
            }),
        };

        Ok(AvatarFilterLua(AvatarFilter {
            kind,
            filter: filter_value,
        }))
    }
}

impl IntoLua for AvatarFilterLua {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("type", match self.0.kind {
            1 => "Content",
            2 => "Instance",
            3 => "QuestTags",
            4 => "LootItem",
            5 => "Dialog",
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: "AvatarFilter",
                to: "table".to_string(),
                message: Some(format!("unknown filter kind: {}", self.0.kind)),
            }),
        })?;
        table.set("filter", self.0.filter)?;
        Ok(mlua::Value::Table(table))
    }
}

impl From<AvatarFilterLua> for AvatarFilter {
    fn from(value: AvatarFilterLua) -> Self {
        value.0
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
pub struct FailQuest {
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
    quest_updated: SystemId<In<(Entity, i32, Option<realm_api::QuestState>)>>,
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

#[derive(Event)]
pub struct QuestStateUpdated {
    pub player: Entity,
    pub quest_id: i32,
    pub state: QuestState,
}

#[derive(Event, Clone, Copy)]
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

#[derive(Event)]
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

#[derive(Component, Default)]
pub struct QuestLog {
    // Quests ids per status, for fast access when determining interests
    available: HashSet<i32>,
    completed: HashSet<i32>,
    finished: HashSet<i32>,
    in_progress: HashSet<i32>,

    quests: HashMap<i32, QuestProgress>,
}

impl QuestLog {
    fn mark_available(&mut self, quest: Arc<Quest>) {
        let id = quest.id;
        self.quests.entry(id).or_insert(QuestProgress {
            template: quest,
            state: None,
        });
        self.update_fast_access_maps();
    }

    fn update_fast_access_maps(&mut self) {
        self.available.clear();
        self.completed.clear();
        self.finished.clear();
        self.in_progress.clear();

        for (&id, progress) in &self.quests {
            match progress.state.as_ref().map(|s| s.state) {
                Some(QuestProgressionState::Active) => {
                    self.in_progress.insert(id);
                }
                Some(QuestProgressionState::Completed) => {
                    self.completed.insert(id);
                }
                Some(QuestProgressionState::Finished) => {
                    self.finished.insert(id);
                }
                Some(QuestProgressionState::Failed) => {
                    // Failed quests are not tracked in fast access maps
                }
                None => {
                    self.available.insert(id);
                }
            }
        }
    }
}

fn hot_reload_quests(
    mut events: EventReader<LuaScriptReloaded>,
    quests: Res<QuestRegistry>,
    mut commands: Commands,
) {
    for _ in events.read() {
        for (_, quest) in quests.0.iter() {
            if 
                let Ok(true) = quest.table.get::<bool>("__hot_reload") &&
                let Ok(func) = quest.table.get::<Function>("HotReload")
            {
                    commands
                        .call_lua_method(func, quest.table.clone());
                }
            }
    }
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
            LuaQuestState::Available => "AVAILABLE".into_lua(lua),
            LuaQuestState::InProgress => "IN_PROGRESS".into_lua(lua),
            LuaQuestState::Completed => "COMPLETED".into_lua(lua),
            LuaQuestState::Finished => "FINISHED".into_lua(lua),
            LuaQuestState::Unavailable => "UNAVAILABLE".into_lua(lua),
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn insert_questlog_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let quest_api = lua.create_table().unwrap();
    runtime.register_native("questlog", quest_api.clone()).unwrap();

    quest_api.set("MarkQuestAvailable", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut questlogs: Query<&mut QuestLog>,
        quests: Res<QuestRegistry>,
    | -> WorldResult<()> {
        let mut log = questlogs.get_mut(owner.entity()?)
            .map_err(|_| anyhow!("object not found"))?;

        let Some(quest) = quests.0.get(&quest_id) else {
            return Err(WorldError::Other(anyhow!("unknown quest id: {}", quest_id)));
        };

        log.mark_available(quest.clone());

        Ok(())
    })?)?;

    quest_api.set("FailQuest", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        commands.send_event(FailQuest { 
            player: owner.entity()?,
            quest_id,
        });

        Ok(())
    })?)?;

    quest_api.set("GetQuestState", lua.create_bevy_function(world, |
        In((owner, quest_id)): In<(Table, i32)>,
        questlogs: Query<&QuestLog>,
    | -> WorldResult<LuaQuestState> {
        let questlog = questlogs.get(owner.entity()?)
            .map_err(|e| WorldError::Other(anyhow!("Failed to get quest log: {}", e)))?;

        let Some(quest) = questlog.quests.get(&quest_id) else {
            return Ok(LuaQuestState::Unavailable);
        };

        match quest.state.as_ref() {
            None => Ok(LuaQuestState::Available),
            Some(state) => 
                match state.state {
                    QuestProgressionState::Active => Ok(LuaQuestState::InProgress),
                    QuestProgressionState::Completed => Ok(LuaQuestState::Completed),
                    QuestProgressionState::Finished => Ok(LuaQuestState::Finished),
                    QuestProgressionState::Failed => Ok(LuaQuestState::Unavailable),
                }
        }
    })?)?;

    quest_api.set("UpdateQuestMarker", lua.create_bevy_function(world, |
        In((player_tbl, target_tbl, quest, state)): In<(Table, Table, Table, i32)>,
        targets: Query<&Children, With<NonClientBaseTag>>,
        markers: Query<&QuestPlayer, With<AttachedQuest>>,
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
            commands.entity(entity)
                .remove::<(QuestAvailable, AttachedQuest)>();
        } else {
            let quest_ent = commands.spawn(
                (
                    AttachedQuest { quest_id },
                    QuestPlayer(player_tbl.entity()?),
                    ChildOf(target_entity)
                )
            ).id();

            if state == 1 {
                commands.entity(quest_ent)
                    .insert(QuestAvailable);
            }
        }

        Ok(())
    })?)?;

    quest_api.set("GetAttachedQuests", lua.create_bevy_function(world, |
        In((target, player)): In<(Table, Table)>,
        quest_target: Query<&Children, With<NonClientBaseTag>>,
        markers: Query<(&QuestPlayer, &AttachedQuest)>,
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

    quest_api.set("UpdateQuest", lua.create_bevy_function(world, |
        In(quest): In<Table>,
        mut players: Query<(&mut QuestLog, &PlayerController)>,
        mut quests: ResMut<QuestRegistry>,
        beacon_query: Query<&GameObjectData>,
        runtime: Res<LuaRuntime>,
        zone: Res<ZoneInstance>,
    | -> WorldResult<()> {
        let info: Result<(i32, String), mlua::Error> = try {
            (
                quest.get::<i32>("id")?,
                quest.get::<String>("world")?,
            )
        };

        match info {
            Ok((id, world)) => {
                // Update global quest registry
                let Some(world_def) = block_in_place(|| zone.manager.get_world_def_by_name(&world)) else {
                    return Err(WorldError::Other(anyhow!("Quest references unknown world: {}", world)));
                };

                let template = Arc::new(Quest {
                    id,
                    table: quest,
                    owned: world == world_def.name(),
                    world_def,
                });

                quests.0.insert(id, template.clone());

                // Update clients
                for (mut quest_log, controller) in players.iter_mut() {
                    if 
                        let Some(progress) = quest_log.quests.get_mut(&id) &&
                        progress.state.is_some()
                    {
                        progress.template = template.clone();

                        send_quest(runtime.vm(), controller, &template, &zone, &beacon_query);
                    }
                }
            },
            Err(err) => {
                return Err(WorldError::Other(anyhow!("Quest does not have a valid id: {:?}", err)));
            }
        }

        Ok(())
    })?)?;

    quest_api.set("UpdateQuestProgress", lua.create_bevy_function(world, |
        In((player_obj, quest_id, condition_id, action, value)): In<(Table, i32, i32, String, i32)>,
        mut commands: Commands,
    | -> WorldResult<()> {
        commands.send_event(QuestConditionUpdate {
            player: player_obj.entity()?,
            quest_id,
            condition_id,
            update: match action.as_str() {
                "ADD" => ConditionUpdate::Added(value),
                "REMOVE" => ConditionUpdate::Removed(value),
                "SET" => ConditionUpdate::Set(value),
                _ => return Err(WorldError::Other(anyhow!("Unknown action: {}", action))),
            },
        });

        Ok(())
    })?)?;

    quest_api.set("GetConditionProgress", lua.create_bevy_function(world, |
        In((quest, player, condition_id)): In<(Table, Table, i32)>,
        players: Query<&QuestLog>,
    | -> WorldResult<i32> {
        let Ok(quest_id) = quest.get::<i32>("id") else {
            return Err(WorldError::Other(anyhow!("Quest does not have a valid id")));
        };

        let Ok(quest_log) = players.get(player.entity()?) else {
            return Err(WorldError::Other(anyhow!("Player does not have a valid quest log")));
        };

        let Some(progress) = quest_log.quests.get(&quest_id).and_then(|q| q.state.as_ref()) else {
            return Err(WorldError::Other(anyhow!("Player has not started quest {}", quest_id)));
        };

        let Some(condition) = progress.conditions.iter().find(|c| c.id == condition_id) else {
            return Err(WorldError::Other(anyhow!("Quest {} does not have condition {}", quest_id, condition_id)));
        };

        Ok(condition.current_count)
    })?)?;

    Ok(())
}

fn command_accept_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.send_event(AcceptQuest { 
            player: ent,
            quest_id,
        });
    }
}

fn command_complete_quest(
    In((_ent, args)): In<(Entity, Vec<NativeParam>)>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(_quest_id)) = args.next() {
        todo!("Implement command_complete_quest");
    }
}

fn command_finish_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.send_event(ReturnQuest { 
            player: ent,
            quest_id,
        });
    }
}

fn command_fail_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.send_event(FailQuest { 
            player: ent,
            quest_id,
        });
    }
}

#[allow(clippy::type_complexity)]
fn update_available_quests(
    mut events: EventReader<QuestStateUpdated>,
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
}

fn handle_db_quest_update(
    In((player, quest_id, db_state)): In<(Entity, i32, Option<realm_api::QuestState>)>,
    mut players: Query<(&PlayerController, &mut QuestLog)>,
    mut commands: Commands,
) {
    let Ok((controller, mut quest_log)) = players.get_mut(player) else {
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

            commands.send_event(QuestStateUpdated {
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
            commands.send_event(QuestStateUpdated {
                player,
                quest_id,
                state: QuestState::Abandoned,
            });
        }

        quest_log.update_fast_access_maps();
    }
}

fn transmit_questlog(
    query: Query<(&QuestLog, &PlayerController), Added<QuestLog>>,
) {
    for (quest_log, controller) in query.iter() {
        for (_, quest) in quest_log.quests.iter() {
            if let Some(state) = &quest.state {
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
    avatars: Query<&AvatarInfo>,
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
        },
        OaPktQuestRequestRequest::RequestNext => {
            commands.send_event(RequestNextQuest {
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
pub struct QuestPlayer(Entity);

fn quest_accepter(
    mut events: EventReader<AcceptQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    instance: Res<ZoneInstance>,
    systems: Res<QuestSystems>,
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
            let realm_api = instance.realm_api.clone();
            let controller = player_controller.clone();

            let mut state = realm_api.create_empty_queststate(controller.character_id(), quest_id, QuestProgressionState::Active);

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

            commands.run_system_async(async move {
                match realm_api.create_queststate(&state).await {
                    Ok(state) => {
                        debug!("Player {} accepted quest {}", player, quest_id);

                        (player, quest_id, Some(state))
                    },
                    Err(e) => {
                        error!("Failed to save quest state for player {}: {}", controller.character_id(), e);
                        controller.close();
                        (player, quest_id, None)
                    }
                }
            }, systems.quest_updated);
        } else {
            warn!("Player {} tried to accept unavailable quest {}", player, quest_id);
        }
    }
}

fn quest_returner(
    mut events: EventReader<ReturnQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    systems: Res<QuestSystems>,
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

            commands.run_system_async(async move {
                match state.update_state(QuestProgressionState::Finished).await {
                    Ok(_) => {
                        debug!("Player {} finished quest {}", player, quest_id);

                        (player, quest_id, Some(state))
                    },
                    Err(e) => {
                        error!("Failed to save quest state for player {}: {}", controller.character_id(), e);
                        controller.close();
                        (player, quest_id, None)
                    }
                }
            }, systems.quest_updated);
        } else {
            warn!("Player {} tried to finish uncompleted quest {}", player, quest_id);
        }
    }
}

fn quest_abandoner(
    mut events: EventReader<AbandonQuest>,
    players: Query<(&QuestLog, &PlayerController)>,
    systems: Res<QuestSystems>,
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

            commands.run_system_async(async move {
                match state.delete().await {
                    Ok(_) => {
                        debug!("Player {} abandoned quest {}", player, quest_id);

                        (player, quest_id, None)
                    },
                    Err(e) => {
                        error!("Failed to save quest state for player {}: {}", controller.character_id(), e);
                        controller.close();
                        (player, quest_id, None)
                    }
                }
            }, systems.quest_updated);
        } else {
            warn!("Player {} tried to abandon unstarted quest {}", player, quest_id);
        }
    }
}

fn handle_quest_state_changes(
    mut events: EventReader<QuestStateUpdated>,
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
            commands.send_event(RequestNextQuest { player });
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
    mut events: EventReader<QuestConditionUpdate>,
    players: Query<(&QuestLog, &PlayerController)>,
    systems: Res<QuestSystems>,
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

        commands.run_system_async(async move {
            let res = quest_state.update_condition(condition_id, 
                match update {
                    ConditionUpdate::Added(_) => realm_api::ConditionUpdate::Increment,
                    ConditionUpdate::Removed(_) => realm_api::ConditionUpdate::Increment,
                    ConditionUpdate::Set(_) => realm_api::ConditionUpdate::Set,
                },
                match update {
                    ConditionUpdate::Added(v) => v,
                    ConditionUpdate::Removed(v) => -v,
                    ConditionUpdate::Set(v) => v,
                }).await;

            match res {
                Ok(_) => {
                    debug!("Updated quest condition {} for quest {} for player {}", condition_id, quest_id, player);

                    (player, quest_id, Some(quest_state))
                },
                Err(e) => {
                    error!("Failed to update quest condition {} for quest {} for player {}: {:#?}", condition_id, quest_id, player, e);
                    controller.close();
                    (player, quest_id, None)
                }
            }
        }, systems.quest_updated);
    }
}

fn quest_segue_handler(
    mut events: EventReader<RequestNextQuest>,
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