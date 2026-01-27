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
mod quest;
mod network;
mod conditions;
mod lifecycle;
mod tags;
mod actions;
mod cache;

pub use questlog::*;
pub use quest::*;
use network::*;
pub use conditions::*;
use lifecycle::*;
pub use tags::*;
pub use actions::*;
use cache::*;

use std::path::PathBuf;

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, prelude::{App, Commands}, state::state::OnEnter};
use log::{error, info};
use mlua::Function;
use scripting::{LuaRuntime, ScriptCommandsExt};

use crate::{instance::{InstanceState, ZoneInstance}, plugins::{CommandExtPriv, NetworkExtPriv, quests::{commands::{command_accept_quest, command_complete_quest, command_fail_quest, command_finish_quest}, lua::{hot_reload_quests, insert_questlog_api}}}};
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
