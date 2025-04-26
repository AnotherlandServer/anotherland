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

use std::{collections::HashSet, sync::Arc};

use async_graphql::Enum;
use database::DatabaseRecord;
use log::debug;
use mongodb::{bson::{self, doc}, options::{Collation, CollationStrength, IndexOptions}, ClientSession, IndexModel};
use obj_params::{GameObjectData, GenericParamSet, ItemBase, ItemEdna, ParamSet, Player};
use serde::{Deserialize, Serialize};
use toolkit::{types::Uuid, GraphqlCrud};
use anyhow::anyhow;

use crate::equipment_slots::EQUIPMENT_SLOTS;

use super::{ItemStorage, ObjectTemplate};

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CombatStyle {
    Rage,
    Tech,
    Assassin,
    Energizer,
    Hacker,
    Cyber,
    None,
}

impl CombatStyle {
    pub async fn load_definition(&self) -> Result<content::CombatStyle, content::error::Error> {
        match self {
            CombatStyle::Rage => content::CombatStyle::load("rage").await,
            CombatStyle::Tech => content::CombatStyle::load("tech").await,
            CombatStyle::Assassin => content::CombatStyle::load("assassin").await,
            CombatStyle::Energizer => content::CombatStyle::load("energizer").await,
            _ => Ok(content::CombatStyle::default()),
        }
    }
}

impl From<CombatStyle> for i32 {
    fn from(style: CombatStyle) -> Self {
        match style {
            CombatStyle::Rage => 0,
            CombatStyle::Tech => 1,
            CombatStyle::Assassin => 2,
            CombatStyle::Energizer => 3,
            CombatStyle::Hacker => 4,
            CombatStyle::Cyber => 5,
            CombatStyle::None => 6,
        }
    }
}

impl TryFrom<i32> for CombatStyle {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CombatStyle::Rage),
            1 => Ok(CombatStyle::Tech),
            2 => Ok(CombatStyle::Assassin),
            3 => Ok(CombatStyle::Energizer),
            4 => Ok(CombatStyle::Hacker),
            5 => Ok(CombatStyle::Cyber),
            6 => Ok(CombatStyle::None),
            _ => Err(anyhow!("Invalid combat style value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, GraphqlCrud)]
#[graphql_crud(name = "character")]
pub struct Character {
    pub id: Uuid,
    #[graphql_crud(filter)]
    pub account: Uuid,
    #[graphql_crud(filter)]
    pub index: i32,
    #[graphql_crud(filter)]
    pub name: String,
    #[graphql_crud(serialize_as = serde_json::Value)]
    pub data: GameObjectData,
}

impl DatabaseRecord for Character {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn collection_name() -> &'static str {
        "characters"
    }

    fn relations() -> &'static[(&'static str, &'static str)] {
        &[
            ("skillbook", "id"),
            ("ability_bar", "id"),
            ("item_storage", "owner.Character"),
        ]
    }

    async fn build_index(db: &mongodb::Database) -> database::DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "name": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .collation(
                        Collation::builder()
                        .locale("en")
                        .strength(CollationStrength::Secondary)
                        .build()
                    )
                    .build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc! { "account": 1, "index": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        Ok(())
    }
}

impl Character {
    pub async fn update_equipment(db: &mongodb::Database, session: &mut ClientSession, character_id: Uuid, storage_id: Uuid) -> database::DBResult<Box<dyn GenericParamSet>> {
        #[derive(Debug)]
        struct Item {
            template_id: i32,
            instance: GameObjectData,
        }

        fn apply_equipment(item: Arc<Item>, body_slots: &mut [Option<Arc<Item>>]) {
            if let Some(eq_type) = item.instance.get::<_, String>(ItemBase::SlotMapping).ok()
                .and_then(|slot_mapping| EQUIPMENT_SLOTS.get_equipment_type(slot_mapping)) {
                
                // Free slots
                for &slot in &eq_type.slots {
                    if let Some(Some(item)) = body_slots.get(slot).cloned() {
                        for slot in body_slots.iter_mut() {
                            if 
                                let Some(item_slot) = slot &&
                                item_slot.template_id == item.template_id
                            {
                                *slot = None;   
                            }
                        }
                    }
                }

                // Insert item, block all required slots
                for &slot in &eq_type.slots {
                    if let Some(slot) = body_slots.get_mut(slot) {
                        *slot = Some(item.clone());
                    }
                }
            }
        }
        
        let mut character = Character::collection(db)
            .find_one(doc! { "id": character_id })
            .session(&mut * session)
            .await?
            .ok_or(anyhow!("Character not found"))?;

        let storage = ItemStorage::collection(db)
            .find_one(doc! { "id": storage_id })
            .session(&mut * session)
            .await?
            .ok_or(anyhow!("Storage not found"))?;

        let mut equipment = vec![];
        let mut visible_equipment = vec![];
        let mut disguises = vec![];
        let mut visible_slots = vec![];

        visible_slots.resize(EQUIPMENT_SLOTS.get_slot_type("Body").unwrap().total_slots(), None);

        debug!("Storage items: {:#?}", storage.items);

        for mut item in storage.items {
            if 
                *item.instance.0.get::<_, i32>(ItemBase::ContainerId).unwrap() == 1 &&
                let Some(template) = ObjectTemplate::get(db, &item.template_id).await?
            {
                item.instance.set_parent(Some(Arc::new(template.data)));

                if let Some(eq_type) = item.instance.get::<_, String>(ItemBase::SlotMapping).ok()
                    .and_then(|slot_mapping| EQUIPMENT_SLOTS.get_equipment_type(slot_mapping))
                {
                    let item = Item {
                        template_id: template.numeric_id,
                        instance: item.instance.0,
                    };

                    debug!("Slot type: {}", eq_type.slot_type.name());

                    if eq_type.slot_type.name() == "Body" {
                        if *item.instance.get::<_, i32>(ItemEdna::Disguise).unwrap() == 0 {
                            disguises.push(item);
                        } else {
                            visible_equipment.push(item);
                        }
                    } else {
                        equipment.push(item);
                    }
                }
            }
        }

        // Insert base items
        for item_id in character.data.get::<_, Vec<i32>>(Player::DefaultItemsContentGuid).unwrap_or(&vec![]) {
            let item = match ObjectTemplate::collection(db)
                .find_one(doc! { "numeric_id": item_id })
                .await? 
            {
                Some(item) => item,
                None => continue,
            };

            if 
                let Some(eq_type) = item.data.get::<_, String>(ItemBase::SlotMapping).ok()
                    .and_then(|slot_mapping| EQUIPMENT_SLOTS.get_equipment_type(slot_mapping)) &&
                eq_type.slot_type.name() == "Body"
            {
                let item = Arc::new(Item {
                    template_id: item.numeric_id,
                    instance: GameObjectData::instantiate(&Arc::new(item.data)),
                });

                for &slot in &eq_type.slots {
                    if let Some(slot) = visible_slots.get_mut(slot) {
                        *slot = Some(item.clone());
                    }
                }
            }
        }

        debug!("Equipment: {visible_equipment:?}");
        debug!("Disguises: {disguises:?}");

        // Insert equipped items
        for item in visible_equipment {
            apply_equipment(Arc::new(item), &mut visible_slots);
        }

        // Insert disguises
        for item in disguises {
            apply_equipment(Arc::new(item), &mut visible_slots);
        }

        debug!("Visible slots: {visible_slots:?}");

        // Todo: Update character stats

        // Compile visual items
        let mut visual_items = HashSet::new();
        for item in visible_slots.into_iter().flatten() {
            visual_items.insert(item.template_id);
        }

        character.data.set(Player::VisibleItemInfo, visual_items.into_iter().collect::<Vec<_>>());
        
        Character::collection(db)
            .update_one(
                Character::query_one(character.key()), 
                doc!{"$set": bson::to_bson(&character).unwrap().as_document().unwrap()},
            )
            .session(&mut * session)
            .await?;

        let mut changes = ParamSet::<Player>::new();
        character.data.changes()
            .for_each(|(key, value)| {
                changes.set_param(key.name(), value);
            });

        Ok(Box::new(changes))
    }
}