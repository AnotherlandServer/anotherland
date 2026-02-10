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

use std::{fs, path::Path, sync::OnceLock, time::Duration};

use anyhow::anyhow;
use content::get_content_path;
use futures_util::{TryStreamExt, future::try_join_all};
use log::{error, info};
use notify::{EventKind, ReadDirectoryChangesWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer};
use realm_api::{AvatarSelector, CombatStyle, Condition, Prerequisites, QuestTemplate, RealmApi};
use serde::Deserialize;
use tokio::{runtime::Handle, task};
use toolkit::types::Uuid;

use crate::{QuestCompilerError, Result};

#[derive(Deserialize, Default)]
struct YamlQuestTemplate {
    id: i32,
    chain: Option<i32>,
    level: i32,
    exp_reward: Option<i32>,
    bit_reward: Option<i32>,
    available_dialogue_id: Option<i32>,
    progress_dialogue_id: Option<i32>,
    completion_dialogue_id: Option<i32>,
    world: String,
    prerequisites: Option<YamlQuestPrerequisites>,
    stages: Vec<Vec<YamlQuestCondition>>,
}

#[derive(Deserialize, Default)]
struct YamlQuestPrerequisites {
    quests_finished: Option<Vec<i32>>,
    combat_style: Option<String>,
    level: Option<i32>,
}

#[derive(Deserialize)]
struct YamlQuestCondition {
    id: i32,
    beacon: Option<Uuid>,
    required_count: i32,
    #[serde(default)]
    hidden: bool,
    trigger: YamlQuestTrigger,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum YamlAvatarSelector {
    Instance { instance_guid: Uuid },
    Content { content_guid: Uuid },
    QuestTag { quest_tag: i32 },
    Item { item_id: Uuid },
    Dialogue { dialogue_id: i32 },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum YamlQuestTrigger {
    Interact { interact: YamlAvatarSelector },
    Kill { kill: YamlAvatarSelector },
    Proximity { proximity: ProximityTrigger },
    Sojourn { _sojourn: YamlAvatarSelector },
    Dialogue { dialogue: i32 },
    Timeout { timeout: f32 },
    Loot { loot: String },
}

#[derive(Deserialize)]
pub struct ProximityTrigger {
    pub avatar: YamlAvatarSelector,
    pub radius: f32,
}

impl TryFrom<YamlAvatarSelector> for AvatarSelector {
    type Error = QuestCompilerError;

    fn try_from(value: YamlAvatarSelector) -> Result<Self> {
        match value {
            YamlAvatarSelector::Instance { instance_guid } => Ok(AvatarSelector::InstanceId(instance_guid)),
            YamlAvatarSelector::Content { content_guid } => Ok(AvatarSelector::ContentId(content_guid)),
            YamlAvatarSelector::QuestTag { quest_tag } => Ok(AvatarSelector::QuestTag(quest_tag)),
            YamlAvatarSelector::Item { item_id } => Ok(AvatarSelector::LootItem(item_id)),
            YamlAvatarSelector::Dialogue { dialogue_id } => Ok(AvatarSelector::DialogId(dialogue_id)),
        }
    }
}

pub async fn import_quest_template_file(path: impl AsRef<Path>) -> Result<()> {
    info!("Importing quest template file {:?}", path.as_ref());

    let content = fs::read(path)
        .map_err(|e| QuestCompilerError::Other(e.into()))?;

    try_join_all(
        serde_saphyr::from_multiple::<YamlQuestTemplate>(
            str::from_utf8(&content)
                .map_err(|e| QuestCompilerError::Other(e.into()))?
        )
        .map_err(|e| QuestCompilerError::Other(e.into()))?
        .into_iter()
        .map(import_quest_template_yaml)
        .collect::<Vec<_>>()
    ).await?;

    Ok(())
}

async fn import_quest_template_yaml(doc: YamlQuestTemplate) -> Result<()> {
    let world = RealmApi::get()
        .query_worlddefs()
        .name(doc.world.clone())
        .query().await?
        .try_next().await?
        .ok_or(QuestCompilerError::Other(anyhow!("world not found")))?;
    
    let prerequisites = doc.prerequisites
        .and_then(|c| {
            let conditions = Prerequisites {
                level: c.level,
                combat_style: c.combat_style.and_then(|s| {
                    match s.to_lowercase().as_str() {
                        "rage" => Some(CombatStyle::Rage),
                        "tech" => Some(CombatStyle::Tech),
                        "assassin" => Some(CombatStyle::Assassin),
                        "energizer" => Some(CombatStyle::Energizer),
                        "hacker" => Some(CombatStyle::Hacker),
                        "cyber" => Some(CombatStyle::Cyber),
                        "none" => Some(CombatStyle::None),
                        _ => None,
                    }
                }),
                quests_finished: c.quests_finished,
            };

            if conditions.level.is_some() ||
                conditions.combat_style.is_some() ||
                conditions.quests_finished.is_some()
            {
                Some(conditions)
            } else {
                None
            }
        });

    let quest_template = QuestTemplate {
        id: doc.id,
        chain_id: doc.chain,
        level: doc.level,
        exp_reward: doc.exp_reward,
        bit_reward: doc.bit_reward,
        available_dialogue_id: doc.available_dialogue_id,
        progress_dialogue_id: doc.progress_dialogue_id,
        completion_dialogue_id: doc.completion_dialogue_id,
        world_id: *world.id() as i32,
        prerequisites,
        conditions: doc.stages
            .into_iter()
            .enumerate()
            .map(|(stage, conditions)| -> Result<Vec<Condition>> {
                    conditions
                        .into_iter()
                        .map(|c| {
                            match c.trigger {
                                YamlQuestTrigger::Interact { interact } => {
                                    Ok(Condition::Interact {
                                        id: c.id,
                                        stage: stage as i32,
                                        beacon: c.beacon,
                                        hidden: c.hidden,
                                        required_count: c.required_count,
                                        avatar_selector: interact.try_into()?,
                                    })
                                },
                                YamlQuestTrigger::Dialogue { dialogue } => {
                                    Ok(Condition::Dialogue {
                                        id: c.id,
                                        stage: stage as i32,
                                        beacon: c.beacon,
                                        hidden: c.hidden,
                                        required_count: c.required_count,
                                        dialogue_id: dialogue,
                                    })
                                },
                                YamlQuestTrigger::Timeout { timeout } => {
                                    Ok(Condition::Wait {
                                        id: c.id,
                                        stage: stage as i32,
                                        hidden: c.hidden,
                                        wait_time_seconds: timeout as f64,
                                    })
                                },
                                YamlQuestTrigger::Kill { kill } => {
                                    Ok(Condition::Kill {
                                        id: c.id,
                                        stage: stage as i32,
                                        beacon: c.beacon,
                                        hidden: c.hidden,
                                        required_count: c.required_count,
                                        avatar_selector: kill.try_into()?,
                                    })
                                },
                                YamlQuestTrigger::Loot { loot } => {
                                    Ok(Condition::Loot { 
                                        id: c.id,
                                        stage: stage as i32,
                                        beacon: c.beacon,
                                        hidden: c.hidden,
                                        required_count: c.required_count,
                                        item_name: loot,
                                    })
                                },
                                YamlQuestTrigger::Proximity { proximity: ProximityTrigger { avatar, radius } } => {
                                    Ok(Condition::Proximity {
                                        id: c.id,
                                        stage: stage as i32,
                                        beacon: c.beacon,
                                        hidden: c.hidden,
                                        required_count: c.required_count,
                                        avatar_selector: avatar.try_into()?,
                                        radius: radius as f64,
                                    })
                                },
                                _ => {
                                    unimplemented!("Quest trigger type not implemented yet")
                                }
                            }
                        })
                        .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    };

    if RealmApi::get()
        .get_quest_template(doc.id)
        .await?
        .is_some()
    {
        info!("Updating quest template ID {}", quest_template.id);

        quest_template.save().await?;
    } else {
        info!("Importing quest template ID {}", quest_template.id);
        RealmApi::get()
            .create_quest_template(quest_template)
            .await?;
    }

    Ok(())
}

pub async fn import_quest_templates() -> Result<()> {
    let quest_template_folder = get_content_path("quests")?;

    info!("Updating quest templates from folder {:?}", quest_template_folder);

    try_join_all(
        quest_template_folder
            .read_dir()
                .map_err(|e| QuestCompilerError::Other(e.into()))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("yaml"))
            .map(|entry| import_quest_template_file(entry.path()))
    ).await?;

    Ok(())
}

pub fn watch_quest_template_changes() -> Result<()> {
    let quest_template_folder = get_content_path("quests")?;
    let handle = Handle::current();

    static DEBOUNCER: OnceLock<Debouncer<ReadDirectoryChangesWatcher, FileIdMap>> = OnceLock::new();

    DEBOUNCER.get_or_init(move || {
        let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |result: DebounceEventResult| {
            let _guard = handle.enter();

            match result {
                Ok(events) => {
                    for ev in events {
                        if matches!(ev.event.kind, EventKind::Modify(_))
                            || matches!(ev.event.kind, EventKind::Create(_))
                        {
                            for path in ev.event.paths {
                                task::spawn(async move {
                                    if let Err(e) = import_quest_template_file(&path).await {
                                        error!("Failed to import quest template {:?}: {:?}", path, e)
                                    }
                                });
                            }
                        }
                    }
                },
                Err(errors) => {
                    for err in errors {
                        error!("{err:?}");
                    }
                },
            }
        }).unwrap();

        debouncer.watch(quest_template_folder, RecursiveMode::Recursive).unwrap();
        debouncer
    });

    Ok(())
}