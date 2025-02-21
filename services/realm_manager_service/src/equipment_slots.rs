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

use std::{collections::HashMap, fs, hash::{Hash, Hasher}, path::PathBuf, sync::{Arc, LazyLock}};

use derive_builder::Builder;
use saphyr::Yaml;
use toolkit::anyhow::{self, anyhow};

use crate::error::{RealmError, RealmResult};

#[derive(Default)]
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

impl Eq for SlotType {}

impl PartialEq for SlotType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for SlotType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl SlotType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total_slots(&self) -> usize {
        self.total_slots
    }
}

#[derive(Builder)]
#[builder(private)]
pub struct EquipmentType { 
    pub name: String,
    pub slot_type: Arc<SlotType>,
    pub slots: Vec<usize>,
    pub te_ratio: f32,
    pub weight: f32,
    pub group: Option<String>,
    pub tick_order: i32,
    pub is_base_appearance: bool,
}

impl ItemManagement {
    pub fn from_file(path: impl Into<PathBuf>) -> RealmResult<Self> {
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
                                                let slot = v.as_i64().unwrap();

                                                if slot != -1 {
                                                    equipment_type_builder.slots(
                                                        vec![slot as usize]
                                                    );
                                                } else {
                                                    equipment_type_builder.slots(vec![]);
                                                }
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
            Err(RealmError::Other(anyhow!("unknown slot definition format")))
        }
    }

    pub fn get_slot_type(&self, name: &str) -> Option<Arc<SlotType>> {
        self.slot_types.get(name).cloned()
    }

    pub fn slot_types(&self) -> Vec<Arc<SlotType>> {
        self.slot_types.values().cloned().collect()
    }

    pub fn get_equipment_type(&self, name: &str) -> Option<Arc<EquipmentType>> {
        self.equipment_types.get(name).cloned()
    }
}

pub static EQUIPMENT_SLOTS: LazyLock<ItemManagement> = LazyLock::new(|| {
    let content_path = std::env::var("CONTENT_PATH")
        .ok()
        .and_then(|p| p.parse::<PathBuf>().ok())
        .or(std::env::current_dir().map(|p| p.join("content")).ok())
        .expect("content path inacessible");

    ItemManagement::from_file(content_path.join("misc/item_management.yaml"))
        .expect("failed to parse item slot definitions")
});