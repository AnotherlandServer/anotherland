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

use std::{fs, iter::repeat_n, path::PathBuf, sync::Arc};

use anyhow::anyhow;
use bevy::{app::{Plugin, PostUpdate}, prelude::{Added, App, Commands, Component, Entity, Query, Resource}, utils::hashbrown::HashMap};
use derive_builder::Builder;
use obj_params::{tags::PlayerTag, GameObjectData, Player};
use realm_api::ObjectTemplate;
use saphyr::Yaml;

use crate::error::{WorldError, WorldResult};

pub struct InventoryPlugin {
    pub content_path: PathBuf,
}

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ItemManagement::from_file(self.content_path.join("misc/item_management.yaml"))
                .expect("failed to parse item slot definitions")
        );

        app.add_systems(PostUpdate, load_player_inventory);
    }
}

#[derive(Resource)]
pub struct ItemManagement {
    slot_types: HashMap<String, Arc<SlotType>>,
    equipment_types: HashMap<String, Arc<EquipmentType>>,
}

#[derive(Builder)]
#[builder(private)]
pub struct SlotType {
    name: String,
    total_slots: usize,
}

#[derive(Builder)]
#[builder(private)]
pub struct EquipmentType { 
    name: String,
    slot_type: Arc<SlotType>,
    slots: Vec<usize>,
    te_ratio: f32,
    weight: f32,
    group: Option<String>,
    tick_order: i32,
    is_base_appearance: bool,
}

impl ItemManagement {
    pub fn from_file(path: impl Into<PathBuf>) -> WorldResult<Self> {
        let docs = Yaml::load_from_str(
            &String::from_utf8(
                    fs::read(path.into())
                    .map_err(anyhow::Error::new)?
                )
                .map_err(anyhow::Error::new)?
        ).map_err(anyhow::Error::new)?;

        if let Yaml::Hash(doc) = &docs[0] {
            let mut slot_types = HashMap::new();
            let mut equipment_types = HashMap::new();

            for (k, v) in doc {
                match k.as_str() {
                    Some("slotTypes") => {
                        if let Some(yaml_slot_types) = v.as_vec() {
                            for yaml_slot in yaml_slot_types {
                                let yaml_slot = yaml_slot.as_hash()
                                    .ok_or(anyhow!("slot types must be a hash map"))?;
                                let mut slot_type_builder = SlotTypeBuilder::create_empty();

                                for (k, v) in yaml_slot {
                                    match k.as_str() {
                                        Some("name") => {
                                            slot_type_builder.name(
                                                v.as_str().ok_or(anyhow!("slot type name must be a string"))?
                                                .to_string()
                                            );
                                        },
                                        Some("totalSlots") => {
                                            slot_type_builder.total_slots(
                                                v.as_i64().ok_or(anyhow!("slot type name must be an integer"))?
                                                as usize
                                            );
                                        },
                                        _ => {},
                                    }
                                }

                                let slot_type = slot_type_builder.build()
                                    .map_err(anyhow::Error::new)?;
                                slot_types.insert(slot_type.name.clone(), Arc::new(slot_type));
                            }
                        }
                    },
                    Some("equipmentTypes") => {
                        if let Some(yaml_equipment_types) = v.as_vec() {
                            for yaml_eq_type in yaml_equipment_types {
                                let yaml_eq_type = yaml_eq_type.as_hash()
                                    .ok_or(anyhow!("equipment types must be a hash map"))?;
                                let mut equipment_type_builder = EquipmentTypeBuilder::create_empty();

                                for (k, v) in yaml_eq_type {
                                    match k.as_str() {
                                        Some("name") => {
                                            equipment_type_builder.name(
                                                v.as_str().ok_or(anyhow!("equipment type name must be a string"))?
                                                .to_string()
                                            );
                                        },
                                        Some("slotType") => {
                                            let slot_type_name = v.as_str()
                                                .ok_or(anyhow!("slot type must be a string"))?;

                                            equipment_type_builder.slot_type(
                                                slot_types.get(slot_type_name)
                                                    .ok_or(anyhow!("slot type '{}' not found", slot_type_name))?
                                                    .clone()
                                            );
                                        },
                                        Some("slots") => {
                                            if v.is_integer() {
                                                equipment_type_builder.slots(
                                                    vec![v.as_i64().unwrap() as usize]
                                                );
                                            } else if v.is_array() {
                                                equipment_type_builder.slots(
                                                    v.as_vec().unwrap()
                                                        .iter()
                                                        .filter(|v| v.is_integer())
                                                        .map(|v| v.as_i64().unwrap() as usize)
                                                        .collect()
                                                );
                                            } else {
                                                return Err(anyhow!("slots must be an integer or integer array").into());
                                            }
                                        },
                                        Some("teRatio") => {
                                            equipment_type_builder.te_ratio(
                                                v.as_f64()
                                                .ok_or(anyhow!("te ratio must be a float"))?
                                                as f32
                                            );
                                        },
                                        Some("weight") => {
                                            equipment_type_builder.weight(
                                                v.as_f64()
                                                .ok_or(anyhow!("weight must be a float"))?
                                                as f32
                                            );
                                        },
                                        Some("group") => {
                                            equipment_type_builder.group(
                                                v.as_str()
                                                .map(|s| s.to_string())
                                            );
                                        },
                                        Some("tickOrder") => {
                                            equipment_type_builder.tick_order(
                                                v.as_i64()
                                                .ok_or(anyhow!("weight must be an integer"))?
                                                as i32
                                            );
                                        },
                                        Some("isBaseAppearance") => {
                                            equipment_type_builder.is_base_appearance(
                                                v.as_bool()
                                                .ok_or(anyhow!("weight must be a boolean"))?
                                            );
                                        },
                                        _ => {},
                                    }
                                }

                                let equipment_type = equipment_type_builder.build()
                                    .map_err(anyhow::Error::new)?;
                                equipment_types.insert(equipment_type.name.clone(), Arc::new(equipment_type));
                            }
                        }
                    },
                    _ => {},
                }
            }

            Ok(Self {
                slot_types,
                equipment_types,
            })
        } else {
            Err(WorldError::Other(anyhow!("unknown slot definition format")))
        }
    }

    pub fn get_slot_type(&self, name: &str) -> Option<Arc<SlotType>> {
        self.slot_types.get(name).cloned()
    }

    pub fn get_equipment_type(&self, name: &str) -> Option<Arc<EquipmentType>> {
        self.equipment_types.get(name).cloned()
    }
}

#[derive(Clone)]
pub enum SlotEntry {
    Empty,
    VisualItem(Arc<ObjectTemplate>),
    Item(Entity),
}

pub enum Soma {
    
}

#[derive(Component)]
pub struct Inventory {
    equipment_slots: HashMap<String, Vec<SlotEntry>>,
    cosmetic_slots: Vec<SlotEntry>,
    
    misc_items: Vec<SlotEntry>,
    quest_items: Vec<SlotEntry>,
    cash_items: Vec<SlotEntry>,
    schema_items: Vec<SlotEntry>,
}

impl Inventory {
    fn new(inventory_size: i32) -> Self {
        Self {
            equipment_slots: HashMap::new(),
            cosmetic_slots: Vec::new(),

            misc_items: repeat_n((), inventory_size as usize)
                .map(|_| SlotEntry::Empty)
                .collect(),
            quest_items: Vec::new(),
            cash_items: Vec::new(),
            schema_items: Vec::new(),
        }
    }
}

fn load_player_inventory(
    query: Query<(Entity, &GameObjectData), Added<PlayerTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in query.iter() {
        commands
            .entity(ent)
            .insert(
                Inventory::new(*obj.get(Player::InventorySize).unwrap())
            );
    }
}