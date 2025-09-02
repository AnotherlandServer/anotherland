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

use bevy::{app::{Plugin, PostUpdate, PreUpdate}, ecs::{event::{Event, EventReader, EventWriter}, query::{Changed, Or, With}, resource::Resource, schedule::IntoScheduleConfigs, system::{In, Res, SystemId}, world::World}, platform::collections::HashSet, prelude::{Added, App, Commands, Component, Entity, Query}};
use futures::TryStreamExt;
use log::{debug, error, info};
use mlua::{AsChunk, Function, Lua, Table};
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, GameObjectData, NonClientBase};
use protocol::{oaPktQuestEvent, CPktStream_165_7, OaPktQuestEventEvent};
use realm_api::QuestProgressionState;
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use anyhow::anyhow;
use toolkit::NativeParam;

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance, plugins::{AvatarInfo, CommandExtPriv, FutureCommands, Interests, PlayerController}};
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
            transmit_active_quests
        ));

        app.add_event::<QuestStateUpdated>();

        app.register_command("accept_quest", command_accept_quest);
        app.register_command("complete_quest", command_complete_quest);
        app.register_command("finish_quest", command_finish_quest);
        app.register_command("fail_quest", command_fail_quest);

        let mut quests = HashMap::new();
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

                                    quests.insert(id, Arc::new(Quest {
                                        id,
                                        table: quest,
                                        owned: world == world_def.name(),
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

#[derive(Resource)]
#[allow(clippy::type_complexity)]
struct QuestSystems {
    insert_questlog_for_player: SystemId<In<Option<(Entity, Vec<realm_api::QuestState>)>>>,
}

pub struct Quest {
    table: Table,
    id: i32,
    owned: bool,
}

#[derive(Resource, Default)]
pub struct QuestRegistry(pub HashMap<i32, Arc<Quest>>);

#[derive(Clone, Copy)]
pub enum QuestState {
    Initialized,
    Available,
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

                if let Some(quest_state_tracker) = quest.state.take() {
                    let player_controller = player_controller.clone();

                    player_controller.send_packet(oaPktQuestEvent {
                        field_1: player_controller.avatar_id(),
                        quest_id,
                        event: OaPktQuestEventEvent::QuestAbandoned,
                        ..Default::default()
                    });

                    instance.spawn_task(async move {
                        if let Err(e) = quest_state_tracker.delete().await {
                            error!("Failed to delete quest state: {}", e);
                            player_controller.close();
                        }
                    });
                }

                quest_log.available.insert(quest_id);
                quest_log.completed.remove(&quest_id);
                quest_log.in_progress.remove(&quest_id);
                quest_log.finished.remove(&quest_id);
            },
            QuestState::Progression(state) => {
                let quest_state_tracker = quest.state.get_or_insert_with(|| instance.realm_api.create_empty_queststate(
                    player_controller.character_id(), 
                    quest_id, 
                    state
                ));

                quest_state_tracker.state = state;

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
                        if let Some(mut quest_state) = realm_api.get_queststate(controller.character_id(), quest_id).await? {
                            quest_state.state = state;
                            quest_state.save().await?;
                        } else {
                            let new_quest_state = realm_api.create_empty_queststate(
                                    controller.character_id(), 
                                    quest_id, 
                                    state
                                );
                            let _ = realm_api.create_queststate(&new_quest_state).await?;
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
    mut players: Query<(&QuestLog, &mut QuestMarkerTracker, &ScriptObject, &Interests), Or<(Changed<QuestLog>, Changed<Interests>)>>,
) {
    
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
    | -> WorldResult<i32> {
        let log = questlog.get(owner.entity()?)
            .map_err(|e| WorldError::Other(anyhow!("Failed to get quest log: {}", e)))?;

        if log.completed.contains(&quest_id) {
            Ok(0)
        } else if log.finished.contains(&quest_id) {
            Ok(1)
        } else if log.in_progress.contains(&quest_id) {
            Ok(2)
        } else if log.available.contains(&quest_id) {
            Ok(3)
        } else {
            Ok(4)
        }
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

                let func = quest.table.get::<Function>("MarkAvailableConditional").unwrap();

                commands.call_lua_method(
                    func, 
                    (quest.table.clone(), script_object.object().clone())
                );
            }
        }
    }
}

fn transmit_active_quests(
    players: Query<(&QuestLog, &PlayerController), Changed<QuestLog>>,
) {
    for (questlog, player_controller) in players.iter() {       
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
            .insert((
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
                },
                QuestMarkerTracker::default(),
            ));

        commands.send_event(QuestStateUpdated {
            player: entity,
            scope: QuestUpdateScope::Player,
            state: QuestState::Initialized,
        });
    }
}

#[derive(Component, Default)]
pub struct QuestMarkerTracker(Vec<Entity>);

#[derive(Component)]
pub struct QuestMarker(Entity);
