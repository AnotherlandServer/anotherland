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

use atlas::{NonClientBaseComponent, NpcBaseComponent, NpcBaseParams, ParamBox, PlayerComponent, PlayerParams, Uuid};
use bevy::{app::{Plugin, Update}, utils::{HashMap, HashSet}};
use bevy_ecs::{component::Component, entity::Entity, event::{Event, EventReader}, query::{Added, Changed, With, Without}, schedule::IntoSystemConfigs, system::{Commands, Query}};
use log::debug;
use mongodb::Database;

use crate::{actors::AvatarComponent, db::{self, Character}, scripting::quest::{lookup_quest_info, quest_iterator, QuestInfo}, util::AnotherlandResult};

use super::{PlayerController, SubjectiveParamSet};

pub struct QuestsPlugin;

#[derive(Component)]
pub struct QuestGiverStatus(HashMap<Entity, Status>);

impl QuestGiverStatus {
    pub fn get(&self, ent: Entity) -> Status {
        *self.0.get(&ent)
            .unwrap_or(&Status::None)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    None,
    InProgress,
    Available,
    Completed,
}

#[derive(Event)]
pub struct QuestAccepted(pub Entity, pub i32);

#[derive(Event)]
pub struct QuestAbandoned(pub Entity, pub i32);

#[derive(Event)]
pub struct QuestCompleted(pub Entity, pub i32);

#[derive(Event)]
pub struct QuestFinished(pub Entity, pub i32);

#[derive(Component, Debug)]
pub struct QuestLog {
    pub available: Vec<i32>,
    pub in_progress: Vec<QuestProgress>,
    pub completed: Vec<QuestProgress>,
    pub finished: Vec<i32>,
}

impl QuestLog {
    pub fn load_from_character(character: &Character) -> QuestLog {
        QuestLog {
            available: vec![],
            in_progress: character.questlog.in_progress.iter()
                .filter_map(|db_progress| {
                    lookup_quest_info(db_progress.id)
                        .map(QuestProgress::new)
                        .map(|mut progress| {
                            for &(condition_id, count) in &db_progress.condition_progress {
                                progress.condition_progress.insert(condition_id, count);
                            }

                            progress
                        })
                })
                .collect(),
            completed: character.questlog.completed.iter()
                .filter_map(|db_progress| {
                    lookup_quest_info(db_progress.id)
                        .map(QuestProgress::new)
                        .map(|mut progress| {
                            for &(condition_id, count) in &db_progress.condition_progress {
                                progress.condition_progress.insert(condition_id, count);
                            }

                            progress
                        })
                })
                .collect(),
            finished: character.questlog.finished.clone(),
        }
    }

    pub fn check_quests_available(&self, quests: &[i32]) -> bool {
        self.available
            .iter()
            .any(|id| quests.contains(id))
    }

    pub fn check_quests_in_progress(&self, quests: &[i32]) -> bool {
        self.in_progress
            .iter()
            .any(|progress| quests.contains(&progress.info.id))
    }

    pub fn check_quests_completed(&self, quests: &[i32]) -> bool {
        self.completed
            .iter()
            .any(|progress| quests.contains(&progress.info.id))
    }

    pub fn check_quests_finished(&self, quests: &[i32]) -> bool {
        self.finished
            .iter()
            .any(|id| quests.contains(id))
    }
}

#[derive(Debug)]
pub struct QuestProgress {
    pub info: &'static QuestInfo,
    pub condition_progress: HashMap<i32, i32>,
}

impl QuestProgress {
    pub fn new(quest: &'static QuestInfo) -> QuestProgress {
        QuestProgress {
            info: quest,
            condition_progress: quest.conditions
                .iter()
                .map(|condition| {
                    (condition.id, 0)
                })
                .collect()
        }
    }
}

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<QuestAccepted>();
        app.add_event::<QuestAbandoned>();
        app.add_event::<QuestCompleted>();
        app.add_event::<QuestFinished>();

        app.add_systems(Update, (
            update_quest_status,
            update_npc_quest_state
        ));
    }
}

pub fn update_quest_status(
    mut accepted_events: EventReader<QuestAccepted>,
    mut abandoned_events: EventReader<QuestAbandoned>,
    mut completed_events: EventReader<QuestCompleted>,
    mut finished_events: EventReader<QuestFinished>,
    spawned_players: Query<Entity, Added<PlayerComponent>>,
    mut players: Query<(&mut ParamBox, &mut QuestLog), (With<PlayerComponent>, Without<NonClientBaseComponent>)>,
) {
    let mut updated_players = HashSet::<Entity>::new();

    for QuestAccepted(player, quest_id) in accepted_events.read() {
        if 
            let Ok((mut params, mut progress)) = players.get_mut(*player) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>() &&
            let Some(quest) = lookup_quest_info(*quest_id)
        {
            params.set_my_quest_track([params.my_quest_track(), &[*quest_id]].concat());
            progress.in_progress.push(QuestProgress::new(quest));

            updated_players.insert(*player);
        }
    }

    for QuestAbandoned(player, quest_id) in abandoned_events.read() {
        if 
            let Ok((mut params, mut progress)) = players.get_mut(*player) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>()
        {
            params.set_my_quest_track(
                params.my_quest_track()
                .iter()
                .filter(|&id| id != quest_id)
                .cloned()
                .collect()
            );

            progress.in_progress.retain(|progress| progress.info.id != *quest_id);
            progress.completed.retain(|progress| progress.info.id != *quest_id);

            updated_players.insert(*player);
        }
    }

    for QuestCompleted(player, quest_id) in completed_events.read() {
        if 
            let Ok((mut params, mut progress)) = players.get_mut(*player) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>()
        {
            let completed = progress.in_progress
                .extract_if(|progress| progress.info.id != *quest_id)
                .collect::<Vec<_>>();
            progress.completed.extend(completed);

            updated_players.insert(*player);
        }
    }

    for QuestFinished(player, quest_id) in finished_events.read() {
        if 
            let Ok((mut params, mut progress)) = players.get_mut(*player) &&
            let Some(params) = params.get_impl_mut::<dyn PlayerParams>()
        {
            params.set_my_quest_track(
                params.my_quest_track()
                .iter()
                .filter(|&id| id != quest_id)
                .cloned()
                .collect()
            );

            progress.in_progress.retain(|progress| progress.info.id != *quest_id);
            progress.completed.retain(|progress| progress.info.id != *quest_id);
            progress.finished.push(*quest_id);

            updated_players.insert(*player);
        }
    }

    updated_players.extend(spawned_players.iter());

    // update available quests for players
    for player in updated_players {
        if let Ok((_, mut progress)) = players.get_mut(player) {
            progress.available = quest_iterator()
                .filter(|quest| {
                    if let Some(prerequisites) = &quest.prerequisites {
                        progress.check_quests_finished(prerequisites)
                    } else {
                        true
                    }
                })
                .map(|info| info.id)
                .collect();
            
            debug!("Player: {:?} - {:#?}", player, progress);
        }

    }
}

pub fn update_npc_quest_state(
    players: Query<(Entity, &QuestLog), Changed<QuestLog>>,
    npcs: Query<(Entity, &ParamBox, Option<&SubjectiveParamSet>), With<NpcBaseComponent>>,
    mut cmds: Commands,
) {
    for (player, questlog) in players.iter() {
        let mut status = HashMap::new();

        for (npc, params, subjective_params) in npcs.iter() {
            let params = params.get_impl::<dyn NpcBaseParams>().unwrap();

            // get the current set of dialogs
            let dialogs = subjective_params
                .and_then(|params| params.get_params(player))
                .and_then(|params| params.get_param("Dialogs"))
                .and_then(|dialogs| dialogs.try_into().ok())
                .unwrap_or(params.dialogs());

            // check for available quests
            for &quest_id in &questlog.available {
                if 
                    let Some(quest_info) = lookup_quest_info(quest_id) &&
                    let Some(dialogue_id) = quest_info.intro_dialogue_id &&
                    dialogs.contains(&dialogue_id)
                {
                    debug!("{:#?}", quest_info);

                    status.insert(npc, Status::Available);
                }
            }

            // check for quests in progress
            for quest_progress in &questlog.in_progress {
                if 
                    let Some(dialogue_id) = quest_progress.info.intermediate_dialogue_id &&
                    dialogs.contains(&dialogue_id)
                {
                    status.insert(npc, Status::InProgress);
                }
            }

            // check for completed quests
            for quest_progress in &questlog.completed {
                if 
                    let Some(dialogue_id) = quest_progress.info.outro_dialogue_id &&
                    dialogs.contains(&dialogue_id)
                {
                    status.insert(npc, Status::Completed);
                }
            }
        }

        debug!("Status: {:?}", status);

        cmds.entity(player)
            .insert(QuestGiverStatus(status));
    } 
}