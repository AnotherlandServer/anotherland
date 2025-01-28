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

use std::{fs, iter::repeat_n, path::PathBuf, sync::Arc, time::Duration};

use anyhow::anyhow;
use bevy::{app::{Plugin, PostUpdate, Update}, prelude::{Added, App, Commands, Component, DetectChangesMut, Entity, In, IntoSystemConfigs, ParamSet, Query, Res, Resource, With}, reflect::List, time::common_conditions::on_timer, utils::hashbrown::HashMap};
use bitstream_io::{ByteWriter, LittleEndian};
use derive_builder::Builder;
use log::debug;
use obj_params::{tags::{ItemBaseTag, PlayerTag}, EdnaFunction, GameObjectData, GenericParamSet, ItemBase, ParamReader, ParamWriter, Player};
use protocol::{oaPktItemStorage, CPktItemUpdate, ItemStorageParams, OaPktItemStorageUpdateType};
use realm_api::ObjectTemplate;
use saphyr::Yaml;
use toolkit::{types::Uuid, NativeParam};

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance};

use super::{CommandExtPriv, ConnectionState, ContentInfo, CurrentState, PlayerController};

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
        app.add_systems(Update, (
            init_client_inventory,
            send_initial_items.run_if(on_timer(Duration::from_secs(1)))
        ).chain());

        app.register_command("add_item", command_add_item);
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
    Item(Entity),
}

pub enum Soma {
    
}

#[derive(Component)]
pub struct Inventory {
    id: Uuid,
    name: String,

    id_lookup: HashMap<Uuid, Entity>,

    equipped_items: Vec<Entity>,
    misc_items: Vec<Entity>,
    quest_items: Vec<Entity>,
    cash_items: Vec<Entity>,
    schema_items: Vec<Entity>,

    bling: Option<i32>,
    max_slots: i32,
}

impl Inventory {
    fn new(id: Uuid, name: String, bling: Option<i32>, max_slots: i32) -> Self {
        Self {
            id,
            name,
            id_lookup: HashMap::new(),

            equipped_items: Vec::new(),
            misc_items: Vec::new(),
            quest_items: Vec::new(),
            cash_items: Vec::new(),
            schema_items: Vec::new(),

            bling,
            max_slots,
        }
    }
}

fn load_player_inventory(
    query: Query<(Entity, &GameObjectData, &PlayerController), Added<PlayerTag>>,
    instance: Res<ZoneInstance>,
    mut commands: Commands,
) {
    for (ent, obj, controller) in query.iter() {
        commands
            .entity(ent)
            .insert(
                Inventory::new(
                    Uuid::new(),
                    "inventory".to_string(),
                    obj.get(Player::Bling).ok().cloned(),
                    *obj.get(Player::InventorySize).unwrap()
                )
            );

        // For now, each player get's a sword
        let controller = controller.clone();
        let object_cache = instance.object_cache.clone();

        instance.spawn_task(async move {
            let obj = object_cache.get_object_by_name("2H_Sword0003Default0004").await.unwrap().unwrap();
            let mut item = GameObjectData::instantiate(&obj.data);

            item.set(EdnaFunction::ContainerId, 1i32);
            item.set(EdnaFunction::InventorySlotIndex, -1i32);
            item.set(EdnaFunction::SlotId, 0i32);

            let mut writer = ByteWriter::<Vec<u8>, LittleEndian>::new(vec![]);
            item.write_to_client(&mut writer).unwrap();

            debug!("Sending test item");

            controller.send_packet(CPktItemUpdate {
                id: Uuid::new(),
                avatar_id: controller.avatar_id(),
                class_id: item.class().id() as u32,
                use_template: 1,
                template_id: Some(obj.id),
                params: writer.writer().to_vec(),
                ..Default::default()
            });
        });

    }
}

fn command_add_item(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    instance: Res<ZoneInstance>,
    mut player: Query<(&PlayerController, &mut Inventory)>,
    items: Query<&GameObjectData, With<ItemBaseTag>>,
) {
    let mut args = args.into_iter();

    if 
        let Ok((controller, mut inventory)) = player.get_mut(ent) &&
        let Some(NativeParam::String(item_name)) = args.next()
    {
        let controller = controller.clone();
        let object_cache = instance.object_cache.clone();

        //debug!("{} {} {} {}", item_name, container, inv_slot, slot);

        /*instance.handle.spawn(async move {
            let obj = object_cache.get_object_by_name(&item_name).await.unwrap().unwrap();
            let mut item = GameObjectData::instantiate(&obj.data);

            item.set(ItemBase::ContainerId, container.parse::<i32>().unwrap_or_default());
            item.set(ItemBase::InventorySlotIndex, inv_slot.parse::<i32>().unwrap_or_default());
            item.set(ItemBase::SlotId, slot.parse::<i32>().unwrap_or_default());

            let mut params = Vec::new();
            let mut writer = ByteWriter::endian(&mut params, LittleEndian);
            item.write_to_client(&mut writer).unwrap();
            
            debug!("Sending test item");
            debug!("{:#?}", Box::<dyn GenericParamSet>::from_slice(item.class(), &params));

            controller.send_packet(CPktItemUpdate {
                id: Uuid::new(),
                avatar_id: controller.avatar_id(),
                class_id: item.class().id() as u32,
                use_template: 1,
                template_id: Some(obj.id),
                params,
                ..Default::default()
            });
        });*/
    }
}

#[derive(Component)]
pub struct InitialInventoryTransfer(Vec<Entity>);

fn init_client_inventory(
    inventories: Query<(&PlayerController, &Inventory), Added<Inventory>>
) {
    for (controller, inventory) in inventories.iter() {
        controller.send_packet(oaPktItemStorage {
            storage_id: Uuid::new(),
            update_type: OaPktItemStorageUpdateType::Unknown004,
            data: ItemStorageParams {
                storage_name: "inventory".to_string(),
                storage_size: inventory.max_slots,
                bling_amount: inventory.bling
                    .unwrap_or(-1),
                has_bling: inventory.bling.is_some(),
            }.to_bytes(),
            ..Default::default()
        });
    }
}

fn send_initial_items(
    mut transfer_queues: Query<(Entity, &PlayerController, &mut InitialInventoryTransfer, &mut CurrentState)>,
    items: Query<(&ContentInfo, &GameObjectData), With<ItemBaseTag>>,
    mut commands: Commands,
) {
    for (entity, controller, mut queue, mut state) in transfer_queues.iter_mut() {
        for item_ent in queue.0.drain(..10) {
            if let Ok((content, item)) = items.get(item_ent) {
                let mut data = Vec::new();
                {
                    let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                    item.write_to_privileged_client(&mut writer).unwrap();
                }

                controller.send_packet(CPktItemUpdate {
                    avatar_id: controller.avatar_id(),
                    id: content.placement_id,
                    use_template: 1,
                    template_id: Some(content.template.id),
                    class_id: item.class().id() as u32,
                    params: data,
                    ..Default::default()
                });
            }
        }

        if queue.0.is_empty() {
            commands.entity(entity)
                .remove::<InitialInventoryTransfer>();

            // Re-trigger change of initial interests loaded, 
            // so client can be spawned if interests transfer finished
            // before item transfer.
            // TODO: Find a better way to sync these two async operations
            // (interest transfer and inventory transfer) in bevy
            if matches!(state.state, ConnectionState::InitialInterestsLoaded) {
                state.set_changed();
            }
        }
    }
}