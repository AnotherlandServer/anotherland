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

use std::{collections::{hash_map::Entry, HashMap}, fmt::Debug, path::PathBuf};

use atlas::Uuid;
use log::{debug, warn};
use mongodb::Database;
use tokio::{fs, sync::OnceCell};
use yaml_rust2::{yaml, Yaml, YamlLoader};

use crate::{db::{realm_database, WorldDef}, scripting::get_content_path, util::{AnotherlandError, AnotherlandResult}};

pub struct QuestInfo {
    pub id: i32,
    pub chain_id: Option<i32>,
    pub level: i32,
    pub exp_reward: Option<i32>,
    pub bit_reward: Option<i32>,
    pub intro_dialogue_id: Option<i32>,
    pub intermediate_dialogue_id: Option<i32>,
    pub outro_dialogue_id: Option<i32>,
    pub world: Uuid,
    pub prerequisites: Option<Vec<i32>>,
    pub conditions: Vec<Condition>,
}

impl Debug for QuestInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuestInfo")
            .field("id", &self.id)
            .field("level", &self.level)
            .field("world", &self.world)
            .field("intro_dialogue_id", &self.intro_dialogue_id)
            .field("intermediate_dialogue_id", &self.intermediate_dialogue_id)
            .field("outro_dialogue_id", &self.outro_dialogue_id)
            .finish_non_exhaustive()
    }
}

pub struct Condition {
    pub id: i32,
    pub required_count: i32,
    pub triggers: Vec<Trigger>,
}

pub enum ObjectRef {
    InstanceGuid(Uuid),
    ContentGuid(Uuid),
}

pub enum Trigger {
    Interaction(ObjectRef),
    Kill(ObjectRef),
    Proximity(ObjectRef),
    Sojourn(ObjectRef),
    Dialogue(i32),
    Timeout(f32),
    Item(Uuid),
}

static QUESTS: OnceCell<HashMap<i32, QuestInfo>> = OnceCell::const_new();

pub async fn read_quests() -> AnotherlandResult<()> {
    let mut quests = HashMap::new();
    let quest_dir = get_content_path().join("quests");

    let mut directory_cursor = fs::read_dir(quest_dir.as_path()).await?;
    while let Some(entry) = directory_cursor.next_entry().await? {
        if 
            entry.file_type().await?.is_file() && 
            let Some(extension) = entry.path().extension() &&
            extension == "yaml"
        {
            let read_quests = read_quest(entry.path()).await?;
            for quest in read_quests {
                if let Entry::Vacant(e) = quests.entry(quest.id) {
                    e.insert(quest);
                } else {
                    warn!("Duplicated quest id: {}", quest.id);
                }
            }
        }
    }

    if QUESTS.set(quests).is_err() {
        panic!("read_quests called twice!");
    }

    Ok(())
}

async fn read_quest(path: PathBuf) -> AnotherlandResult<Vec<QuestInfo>> {
    let mut quests = Vec::new();
    let content = String::from_utf8(fs::read(&path).await?)
        .expect("invalid utf-8 characters in quest file");

    debug!("Reading quest file {}...", path.file_name().unwrap().to_str().unwrap());

    let db = realm_database().await;

    for root in YamlLoader::load_from_str(&content)? {
        quests.push(parse_quest_yaml(&root, &db).await?);
    }

    Ok(quests)
}

async fn parse_quest_yaml(root: &Yaml, db: &Database) -> AnotherlandResult<QuestInfo> {
    let id = i32::try_from(
        root["id"]
        .as_i64()
        .ok_or(AnotherlandError::app_err("id expected"))?
    ).map_err(|_| AnotherlandError::app_err("invalid quest id range"))?;

    let chain_id = root["chain"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let level = i32::try_from(
        root["level"]
        .as_i64()
        .ok_or(AnotherlandError::app_err("level expected"))?
    ).map_err(|_| AnotherlandError::app_err("invalid level range"))?;

    let exp_reward = root["exp_reward"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let bit_reward = root["bit_reward"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let intro_dialogue_id: Option<i32> = root["intro_dialogue"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let intermediate_dialogue_id = root["intermediate_dialogue"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let outro_dialogue_id = root["outro_dialogue"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok());

    let world = root["world"]
        .as_str()
        .ok_or(AnotherlandError::app_err("world expected"))?
        .to_owned();

    let worlddef = WorldDef::get_by_name(db.clone(), &world).await?
        .ok_or(AnotherlandError::app_err("world not found"))?;

    let prerequisites = root["prerequisites"]["quests"]
        .as_vec()
        .and_then(|quests| {
            quests
                .iter()
                .map(|quest_id| {
                    quest_id
                        .as_i64()
                        .and_then(|id| i32::try_from(id).ok())
                })
                .collect::<Option<Vec<_>>>()
        });

    let conditions = root["conditions"]
        .as_vec()
        .ok_or(AnotherlandError::app_err("quest condition array expected"))?
        .iter()
        .map(parse_condition_yaml)
        .collect::<AnotherlandResult<_>>()?;

    Ok(QuestInfo { 
        id, 
        chain_id, 
        level, 
        exp_reward, 
        bit_reward, 
        intro_dialogue_id, 
        intermediate_dialogue_id, 
        outro_dialogue_id, 
        world: worlddef.guid, 
        prerequisites,
        conditions,
    })
}

fn parse_condition_yaml(root: &Yaml) -> AnotherlandResult<Condition> {
    let id = i32::try_from(
        root["id"]
        .as_i64()
        .ok_or(AnotherlandError::app_err("id expected"))?
    ).map_err(|_| AnotherlandError::app_err("invalid quest condition id range"))?;

    let required_count = i32::try_from(
        root["required_count"]
        .as_i64()
        .ok_or(AnotherlandError::app_err("required_count expected"))?
    ).map_err(|_| AnotherlandError::app_err("invalid condition required count range"))?;

    let triggers = root["triggers"]
        .as_vec()
        .ok_or(AnotherlandError::app_err("trigger array expected"))?
        .iter()
        .map(parse_trigger_yaml)
        .collect::<AnotherlandResult<_>>()?;

    Ok(Condition { 
        id, 
        required_count, 
        triggers,
    })
}

fn parse_trigger_yaml(root: &Yaml) -> AnotherlandResult<Trigger> {
    debug!("{:?}", root);

    if let Some(interact) = root["interact"].as_hash() {
        Ok(Trigger::Interaction(parse_object_ref_yaml(interact)?))
    } else if let Some(kill) = root["kill"].as_hash() {
        Ok(Trigger::Kill(parse_object_ref_yaml(kill)?))
    } else if let Some(proximity) = root["proximity"].as_hash() {
        Ok(Trigger::Proximity(parse_object_ref_yaml(proximity)?))
    } else if let Some(sojourn) = root["sojourn"].as_hash() {
        Ok(Trigger::Sojourn(parse_object_ref_yaml(sojourn)?))
    } else if let Some(dialog) = root["dialog"].as_i64() {
        Ok(Trigger::Dialogue(dialog as i32))
    } else if let Some(timeout) = root["timeout"].as_i64() {
        Ok(Trigger::Timeout(timeout as f32))
    } else if let Some(item) = root["item"].as_str() {
        Ok(Trigger::Item(Uuid::parse_str(item).unwrap()))
    } else {
        Err(AnotherlandError::app_err("unknown trigger"))
    }
}

fn parse_object_ref_yaml(yaml: &yaml::Hash) -> AnotherlandResult<ObjectRef> {
    if let Some(instance_guid) = yaml[&Yaml::from_str("instance_guid")].as_str() {
        Ok(ObjectRef::InstanceGuid(Uuid::parse_str(instance_guid).unwrap()))
    } else if let Some(content_guid) = yaml[&Yaml::from_str("content_guid")].as_str() {
        Ok(ObjectRef::ContentGuid(Uuid::parse_str(content_guid).unwrap()))
    } else {
        Err(AnotherlandError::app_err("invalidy object ref"))
    }
}

pub fn lookup_quest_info(id: i32) -> Option<&'static QuestInfo> {
    QUESTS.get()
        .and_then(|quests| quests.get(&id))
}

pub fn quest_iterator() -> impl Iterator<Item = &'static QuestInfo> {
    QUESTS.get().unwrap()
        .iter()
        .map(|(_,v)| v)
}