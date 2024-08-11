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

use std::{collections::{hash_map::Entry, HashMap}, path::PathBuf, str::FromStr};
use bevy::log::warn;
use log::debug;
use tokio::{fs, sync::OnceCell};
use yaml_rust2::{Yaml, YamlLoader};

use crate::{scripting::get_content_path, util::{AnotherlandError, AnotherlandResult}};

#[derive(Debug)]
pub struct DialogueInfo {
    pub id: i32,
    pub nodes: Vec<DialogueNode>,
    pub repeatable: bool,
}

#[derive(Debug)]
pub enum DialogueNode {
    Line {
        line_id: i32,
        animation: Option<String>,
        choices: Vec<DialogueChoice>,
        quest: Option<i32>,
    },
    Transition {
        dialogue_id: i32,
    },
    End,
}

#[derive(Debug)]
pub enum ChoiceIcon {
    Close,
    Approve,
    Reject,
    Next,
    TellMore,
}

impl FromStr for ChoiceIcon {
    type Err = AnotherlandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "close" => Ok(ChoiceIcon::Close),
            "approve" => Ok(ChoiceIcon::Approve),
            "reject" => Ok(ChoiceIcon::Reject),
            "next" => Ok(ChoiceIcon::Next),
            "tellMore" => Ok(ChoiceIcon::TellMore),
            _ => Err(AnotherlandError::app_err("unknown choice icon"))
        }
    }
}

#[derive(Debug)]
pub struct DialogueChoice {
    pub icon: ChoiceIcon,
    pub index: usize,
}

static DIALOGUES: OnceCell<HashMap<i32, DialogueInfo>> = OnceCell::const_new();

pub async fn read_dialogues() -> AnotherlandResult<()> {
    let mut dialogues = HashMap::new();
    let dialogue_dir = get_content_path().join("dialogue");

    let mut directory_cursor = fs::read_dir(dialogue_dir.as_path()).await?;
    while let Some(entry) = directory_cursor.next_entry().await? {
        if 
            entry.file_type().await?.is_file() && 
            let Some(extension) = entry.path().extension() &&
            extension == "yaml"
        {
            let read_dialogues = read_dialogue(entry.path()).await?;
            for dialogue in read_dialogues {
                if let Entry::Vacant(e) = dialogues.entry(dialogue.id) {
                    e.insert(dialogue);
                } else {
                    warn!("Duplicated dialogue id: {}", dialogue.id);
                }
            }
        }
    }

    if DIALOGUES.set(dialogues).is_err() {
        panic!("read_dialogues called twice!");
    }

    Ok(())
}

async fn read_dialogue(path: PathBuf) -> AnotherlandResult<Vec<DialogueInfo>> {
    let mut dialogues = Vec::new();
    let content = String::from_utf8(fs::read(&path).await?)
        .expect("invalid utf-8 characters in dialogue file");

    debug!("Reading dialogue file {}...", path.file_name().unwrap().to_str().unwrap());

    for root in YamlLoader::load_from_str(&content)? {
        dialogues.push(parse_dialogue_yaml(&root)?);
    }

    Ok(dialogues)
}

fn parse_dialogue_yaml(root: &Yaml) -> AnotherlandResult<DialogueInfo> {
    let id = i32::try_from(
        root["id"]
        .as_i64()
        .ok_or(AnotherlandError::app_err("id expected"))?
    ).map_err(|_| AnotherlandError::app_err("invalid dialogue id range"))?;

    let repeatable =  root["repeatable"]
        .as_bool()
        .unwrap_or_default();

    let mut parsed_nodes = root["flow"]
        .as_vec()
        .ok_or(AnotherlandError::app_err("dialogue node array expected"))?
        .iter()
        .map(parse_dialogue_node)
        .collect::<AnotherlandResult<Vec<ParsedNode>>>()?;

    // dialogue end at the end of a dialogue flow is implied
    parsed_nodes.push(ParsedNode::End);

    let mut branch_idx = HashMap::new();

    if let Some(branches) = root["branches"].as_hash() {
        for (id, branch) in branches.into_iter() {
            let branch_id = id
                .as_i64()
                .ok_or(AnotherlandError::app_err("branch id expected"))?;

            // store beginning index of branch in node array
            branch_idx.insert(branch_id, parsed_nodes.len());

            // append branch nodes to node array
            parsed_nodes.append(
                &mut
                branch.as_vec()
                    .ok_or(AnotherlandError::app_err("dialogue node array expected"))?
                    .iter()
                    .map(parse_dialogue_node)
                    .collect::<AnotherlandResult<Vec<ParsedNode>>>()?
            );

            // dialogue end at the end of a branch flow is implied
            parsed_nodes.push(ParsedNode::End);
        }
    }

    let nodes = parsed_nodes
        .into_iter()
        .enumerate()
        .map(|(idx, node)| {
            match node {
                ParsedNode::Line { line_id, animation, choices, quest } => {
                    let mut dialogue_choices = Vec::new();

                    if choices.is_empty() && quest.is_none() {
                        dialogue_choices.push(DialogueChoice {
                            icon: ChoiceIcon::TellMore,
                            index: idx + 1,
                        });
                    } else {
                        dialogue_choices = choices
                            .into_iter()
                            .map(|ParsedChoice { icon, branch }| {
                                if let Some(branch) = branch {
                                    let index = *branch_idx
                                        .get(&branch)
                                        .ok_or(AnotherlandError::app_err("branch id not found"))?;

                                    Ok(DialogueChoice { icon, index })
                                } else {
                                    Ok(DialogueChoice {
                                        icon,
                                        index: idx + 1,
                                    })
                                }
                            })
                            .collect::<AnotherlandResult<Vec<DialogueChoice>>>()?
                    }
                    
                    Ok(DialogueNode::Line { 
                        line_id, 
                        animation, 
                        choices: dialogue_choices, 
                        quest,
                    })
                },
                ParsedNode::Transition { dialogue_id } => Ok(DialogueNode::Transition { dialogue_id }),
                ParsedNode::End => Ok(DialogueNode::End),
            }
        })
        .collect::<AnotherlandResult<Vec<DialogueNode>>>()?;

    Ok(DialogueInfo { 
        id, 
        nodes, 
        repeatable,
    })
}

pub enum ParsedNode {
    Line {
        line_id: i32,
        animation: Option<String>,
        choices: Vec<ParsedChoice>,
        quest: Option<i32>,
    },
    Transition {
        dialogue_id: i32,
    },
    End,
}

pub struct ParsedChoice {
    pub icon: ChoiceIcon,
    pub branch: Option<i64>,
}

fn parse_dialogue_node(node: &Yaml) -> AnotherlandResult<ParsedNode> {
    if let Some(dialogue_id) = node["dialogueId"].as_i64() {
        Ok(ParsedNode::Transition { 
            dialogue_id: i32::try_from(dialogue_id)
                .map_err(|_| AnotherlandError::app_err("invalid dialogue id"))?
        })
    } else if let Some(line_id) = node["line"].as_i64() {
        let choices = node["choices"]
            .as_vec()
            .unwrap_or(&vec![])
            .iter()
            .map(|choice| {
                if let Some(icon) = choice["type"].as_str() {
                    Ok(ParsedChoice { 
                        icon: icon.parse()?, 
                        branch: choice["branch"].as_i64(), 
                    })
                } else {
                    Err(AnotherlandError::app_err("malformed choice node"))
                }
            })
            .collect::<AnotherlandResult<Vec<ParsedChoice>>>()?;
        
        Ok(ParsedNode::Line { 
            line_id: i32::try_from(line_id)
                .map_err(|_| AnotherlandError::app_err("invalid line id"))?, 
            animation: node["animation"].as_str().map(|s| s.to_owned()), 
            choices, 
            quest: node["quest"]
                .as_i64()
                .and_then(|quest| i32::try_from(quest).ok())
        })
    } else if let Some(line_id) = node.as_i64() {
        Ok(ParsedNode::Line { 
            line_id: i32::try_from(line_id)
                .map_err(|_| AnotherlandError::app_err("invalid line id"))?, 
            animation: None, 
            choices: vec![], 
            quest: None
        })
    } else {
        Err(AnotherlandError::app_err("malformed dialogue node"))
    }
}

pub fn lookup_dialogue_info(id: i32) -> Option<&'static DialogueInfo> {
    DIALOGUES.get()
        .and_then(|dialogues| dialogues.get(&id))
}
