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

use content::get_content_path;
use database::DatabaseRecord;
use futures_util::future::try_join_all;
use log::{debug, error, info};
use mongodb::{Database, bson::doc};
use notify::{EventKind, ReadDirectoryChangesWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer};
use serde::Deserialize;
use tokio::{runtime::Handle, task};
use crate::{db::{Choice, CombatStyle, QuestDialogue}, error::{RealmError, RealmResult}};

#[derive(Deserialize, Default)]
struct YamlDialogue {
    id: i32,
    branches: Vec<YamlDialogueBranch>,
}

#[derive(Deserialize, Default)]
struct YamlDialogueBranch {
    selector: YamlDialogueSelector,
    lines: Vec<YamlDialogueLine>,
}

#[derive(Deserialize, Default)]
struct YamlDialogueSelector {
    quests_available: Option<Vec<i32>>,
    quests_in_progress: Option<Vec<i32>>,
    quests_completed: Option<Vec<i32>>,
    quests_finished: Option<Vec<i32>>,
    combat_style: Option<String>,
    level: Option<i32>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum YamlDialogueLine {
    Simple(i32),
    Extended(YamlDialogueLineEx),
}

#[derive(Deserialize, Default)]
struct YamlDialogueLineEx {
    line: i32,
    animation: Option<String>,
    choice: Option<String>,
    quest: Option<i32>,
}

pub async fn import_dialogue_file(db: Database, path: impl AsRef<Path>) -> RealmResult<()> {
    debug!("Importing dialogue file {:?}", path.as_ref());

    let content = fs::read(path)
        .map_err(|e| RealmError::Other(e.into()))?;

    try_join_all(
        serde_saphyr::from_multiple::<YamlDialogue>(
            str::from_utf8(&content)
                .map_err(|e| RealmError::Other(e.into()))?
        )
        .map_err(|e| RealmError::Other(e.into()))?
        .into_iter()
        .map(|doc| import_dialogue_yaml(db.clone(), doc))
        .collect::<Vec<_>>()
    ).await?;

    Ok(())
}

async fn import_dialogue_yaml(db: Database, doc: YamlDialogue) -> RealmResult<()> {
    let dialogue = QuestDialogue {
        id: doc.id,
        branches: doc.branches.into_iter().map(|branch| {
            let selector = branch.selector;
            let dialogue_selector = crate::db::DialogueBranchSelector {
                quests_available: selector.quests_available.unwrap_or_default(),
                quests_in_progress: selector.quests_in_progress.unwrap_or_default(),
                quests_complete: selector.quests_completed.unwrap_or_default(),
                quests_finished: selector.quests_finished.unwrap_or_default(),
                level: selector.level.unwrap_or(0),
                combat_style: selector.combat_style.and_then(|s| {
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
            };

            let lines = branch.lines.into_iter().map(|line| {
                match line {
                    YamlDialogueLine::Simple(line_id) => crate::db::DialogueLine {
                        line_id,
                        animation_name: None,
                        choice: None,
                        quest_id: None,
                    },
                    YamlDialogueLine::Extended(ext) => crate::db::DialogueLine {
                        line_id: ext.line,
                        animation_name: ext.animation,
                        choice: ext.choice.and_then(|c| {
                            match c.to_lowercase().as_str() {
                                "close" => Some(Choice::Close),
                                "approve" => Some(Choice::Approve),
                                "reject" => Some(Choice::Reject),
                                "next" => Some(Choice::Next),
                                "tellmore" => Some(Choice::TellMore),
                                "offer" => Some(Choice::Offer),
                                _ => None,
                            }
                        }),
                        quest_id: ext.quest,
                    },
                }
            }).collect();

            if dialogue_selector.level == 0 
                && dialogue_selector.combat_style.is_none()
                && dialogue_selector.quests_available.is_empty()
                && dialogue_selector.quests_in_progress.is_empty()
                && dialogue_selector.quests_complete.is_empty()
                && dialogue_selector.quests_finished.is_empty()
            {
                crate::db::DialogueBranch {
                    selector: None,
                    lines,
                }
            } else {
                crate::db::DialogueBranch {
                    selector: Some(dialogue_selector),
                    lines,
                }
            }
        }).collect(),
    };

    debug!("Importing dialogue ID {}", dialogue.id);

    QuestDialogue::collection(&db)
        .find_one_and_replace(doc! { "id": dialogue.id }, &dialogue)
        .upsert(true)
        .await?;

    Ok(())
}

pub async fn import_dialogues(db: Database) -> RealmResult<()> {
    let dialogue_folder = get_content_path("dialogue")?;

    info!("Updating dialogues from folder {:?}", dialogue_folder);

    try_join_all(
        dialogue_folder
            .read_dir()
                .map_err(|e| RealmError::Other(e.into()))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("yaml"))
            .map(|entry| import_dialogue_file(db.clone(), entry.path()))
    ).await?;

    Ok(())
}

pub fn watch_dialogue_changes(db: Database) -> RealmResult<()> {
    let dialogue_folder = get_content_path("dialogue")?;
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
                                let db = db.clone();
                                task::spawn(async move {
                                    if let Err(e) = import_dialogue_file(db, &path).await {
                                        error!("Failed to import dialogue file {:?}: {:?}", path, e)
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

        debouncer.watch(dialogue_folder, RecursiveMode::Recursive).unwrap();
        debouncer
    });

    Ok(())
}