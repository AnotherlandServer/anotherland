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
mod objects;
mod actions;
mod cache;
mod progress;

use futures::TryStreamExt;
use log::error;
pub use questlog::*;
pub use quest::*;
use network::*;
pub use conditions::*;
use lifecycle::*;
use realm_api::{QuestTemplate, RealmApi};
use scripting::LuaRuntime;
pub use objects::*;
pub use actions::*;
pub use progress::*;

use std::{path::PathBuf, sync::Arc};

use bevy::{app::{Plugin, PostStartup, PreUpdate, Update}, ecs::{error::Result, schedule::IntoScheduleConfigs, system::{In, Res, ResMut}}, platform::collections::HashMap, prelude::{App, Commands}, state::commands::CommandsStatesExt};

use crate::{instance::{InstanceState, ZoneInstance}, plugins::{AsyncOperationCommandsExt, CommandExtPriv, NetworkExtPriv, WeakCache, quests::{cache::QuestTemplateCache, commands::{command_accept_quest, command_complete_quest, command_fail_quest, command_finish_quest}, lua::{hot_reload_quests, insert_questlog_api}}}};
pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            hot_reload_quests,
            init_quest_visibility, 
            quest_accepter,
            quest_abandoner,
            quest_returner,
            add_npc_quest_tags,
        ));

        app.add_systems(Update, (
            transmit_questlog,
            handle_quest_state_changes,
            handle_quest_condition_update,
            interaction_event_listener,
            updated_timed_conditions,
            auto_return_quests,
            sync_quest_state.after(handle_quest_state_changes), 
            (
                update_available_quests, 
                sync_quest_markers,
            ).chain().after(handle_quest_state_changes),
        ));

        app.add_message::<QuestStateUpdated>();
        app.add_message::<QuestConditionUpdate>();
        app.add_message::<AcceptQuest>();
        app.add_message::<AbandonQuest>();
        app.add_message::<ReturnQuest>();
        app.add_message::<UpdateAvailableQuests>();

        app.init_resource::<Quests>();

        app.register_command("accept_quest", command_accept_quest);
        app.register_command("complete_quest", command_complete_quest);
        app.register_command("finish_quest", command_finish_quest);
        app.register_command("fail_quest", command_fail_quest);

        app.register_message_handler(handle_quest_request);
        app.register_message_handler(handle_quest_action_request);

        insert_questlog_api(app.world_mut()).unwrap();

        app.add_systems(PostStartup, 
            move |
                instance: Res<ZoneInstance>,
                commands: Commands,
            | {
                let world = instance.world_def.clone();

                commands
                    .perform_async_operation(async move {
                        let mut quests = RealmApi::get()
                            .query_quest_templates()
                            .world_id(*world.id())
                            .query()
                            .await?;

                        let mut templates = vec![];

                        while let Some(quest) = quests.try_next().await? {
                            templates.push(
                                QuestTemplateCache::get(&quest.id).await?
                                    .unwrap()
                            );
                        }

                        Ok(templates)
                    })
                    .on_finish_run_system(|
                        In(templates): In<Vec<Arc<QuestTemplate>>>, 
                        mut runtime: ResMut<LuaRuntime>,
                        mut commands: Commands
                    | {
                        commands.insert_resource(Quests::new(
                            templates.into_iter()
                                .map(|template| {
                                    let specific_script = format!("quests.{}", template.id);

                                    let script = if runtime.is_script_file(&specific_script) {
                                        specific_script.as_str()
                                    } else {
                                        "core.base_quest"
                                    };

                                    let class = runtime.load_class(script) 
                                            .and_then(|class| {
                                                let meta = runtime.vm().create_table()?;
                                                meta.set("__index", class)?;

                                                Ok(meta)
                                            })?;

                                    let obj = runtime.vm().create_table()?;

                                    obj.set_metatable(Some(class))?;

                                    Ok(Quest {
                                        id: template.id,
                                        obj,
                                        template,
                                    })
                                })
                                .filter_map(|res: Result<Quest>| {
                                    match res {
                                        Ok(quest) => Some((quest.id, quest)),
                                        Err(e) => {
                                            error!("Failed to load quest: {}", e);
                                            None
                                        }
                                    }
                                })
                                .collect::<HashMap<_, _>>()
                        ));
                        commands.set_state(InstanceState::ObjectLoad);
                    });
            });
    }
}
