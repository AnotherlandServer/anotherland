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

use std::{collections::HashMap, iter::repeat_n, ops::DerefMut, sync::Arc};

use database::{DatabaseError, DatabaseRecord};
use futures_util::future::join_all;
use log::{debug, warn};
use mongodb::{bson::{self, doc}, options::{ReadConcern, ReadPreference, SelectionCriteria, TransactionOptions, WriteConcern}, ClientSession, Database};
use obj_params::{GameObjectData, ItemBase, ItemEdna};
use thiserror::Error;
use tokio::sync::Mutex;
use toolkit::{anyhow::anyhow, types::Uuid, NativeParam};

use crate::{db::{ItemStorage, ObjectTemplate, StorageOwner}, equipment_slots::{EquipmentType, SlotType, EQUIPMENT_SLOTS}};

#[derive(Error, Debug)]
pub enum ItemStorageSessionError {
    #[error(transparent)]
    MongodbError(#[from] mongodb::error::Error),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    BsonError(#[from] bson::ser::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("client error: {0}")]
    ClientError(&'static str, Option<NativeParam>),
}

#[derive(Debug, Clone)]
struct Item {
    id: Uuid,
    base_item: (Uuid, i32),
    instance: GameObjectData,
}

trait ItemAccess {
    async fn id(&self) -> Uuid;

    async fn is_disguise(&self) -> bool;
    #[allow(unused)]
    async fn container_id(&self) -> i32;
    #[allow(unused)]
    async fn inventory_slot_index(&self) -> i32;
    #[allow(unused)]
    async fn slot_id(&self) -> i32;
    async fn slot_mapping(&self) -> Option<Arc<EquipmentType>>;
    async fn set_container_id(&self, id: i32);
    async fn set_inventory_slot_index(&self, idx: i32);
    async fn set_slot_id(&self, id: i32);
}

impl ItemAccess for Mutex<Item> {
    async fn id(&self) -> Uuid {
        self.lock().await.id
    }

    async fn is_disguise(&self) -> bool {
        let item = self.lock().await;
        *item.instance.get::<_, i32>(ItemEdna::Disguise).ok().unwrap_or(&1) == 0
    }

    async fn container_id(&self) -> i32 {
        *self.lock().await.instance.get::<_, i32>(ItemBase::ContainerId).unwrap()
    }

    async fn inventory_slot_index(&self) -> i32 {
        *self.lock().await.instance.get::<_, i32>(ItemBase::InventorySlotIndex).unwrap()
    }

    async fn slot_id(&self) -> i32 {
        *self.lock().await.instance.get::<_, i32>(ItemBase::SlotId).unwrap()
    }

    async fn slot_mapping(&self) -> Option<Arc<EquipmentType>> {
        self.lock().await.instance.get::<_, String>(ItemBase::SlotMapping).ok()
            .and_then(|slot_mapping| EQUIPMENT_SLOTS.get_equipment_type(slot_mapping))
    }

    async fn set_container_id(&self, id: i32) {
        self.lock().await.instance.set(ItemBase::ContainerId, id);
    }

    async fn set_inventory_slot_index(&self, idx: i32) {
        self.lock().await.instance.set(ItemBase::InventorySlotIndex, idx);
    }

    async fn set_slot_id(&self, id: i32) {
        self.lock().await.instance.set(ItemBase::SlotId, id);
    }


}

struct ItemTab {
    can_grow: bool,
    slots: Vec<Slot>,
}

type Slot = Option<Arc<Mutex<Item>>>;

struct Equipment(HashMap<Arc<SlotType>, Vec<Slot>>);

impl Equipment {
    fn new() -> Self {
        let mut slots = HashMap::new();
        for slot in EQUIPMENT_SLOTS.slot_types() {
            slots.insert(slot.clone(), 
                repeat_n(None, slot.total_slots())
                    .collect()
            );
        }

        Self(slots)
    }
}

pub struct ItemStorageSession {
    db: Database,
    session: Arc<Mutex<ClientSession>>,

    id: Uuid,
    name: String,
    owner: StorageOwner,

    items: HashMap<Uuid, Arc<Mutex<Item>>>,

    equipment: Equipment,
    disguise: Equipment,

    misc_items: ItemTab,
    quest_items: ItemTab,
    cash_items: ItemTab,
    schema_items: ItemTab,

    bling: Option<i32>,
    game_cash: Option<i32>,
    capacity: i32,

    removed_items: Vec<Uuid>,
    sub_sessions: Vec<ItemStorageSession>,
}

impl ItemStorageSession {
    async fn init(db: &Database, session: Arc<Mutex<ClientSession>>, id: Uuid) -> Result<Self, ItemStorageSessionError> {
        let mut session_s = session.lock().await;
        let col_storage = ItemStorage::collection(db);

        // Read storage
        let storage = col_storage.find_one(doc! { "id": id })
            .session(session_s.deref_mut())
            .await?
            .ok_or(ItemStorageSessionError::Other(anyhow!("no storage found for id {}", id)))?;

        let mut storage_session = Self { 
            db: db.clone(), 
            session: session.clone(),

            id,
            name: storage.name,
            owner: storage.owner,

            items: HashMap::new(),

            equipment: Equipment::new(),
            disguise: Equipment::new(),

            misc_items: ItemTab {
                can_grow: false,
                slots: Vec::new()
            },
            quest_items: ItemTab {
                can_grow: true,
                slots: Vec::new()
            },
            cash_items: ItemTab {
                can_grow: true,
                slots: Vec::new()
            },
            schema_items: ItemTab {
                can_grow: true,
                slots: Vec::new()
            },

            bling: storage.bling,
            game_cash: storage.game_cash,
            capacity: storage.capacity,

            removed_items: Vec::new(),
            sub_sessions: Vec::new(),
        };

        // misc_items has a fixed capacity, other lists grow as needed
        storage_session.misc_items.slots = repeat_n(None, storage.capacity as usize)
            .collect();

        let mut unplaceable_items = Vec::new();

        // Asynchronously load, place and normalize all items
        join_all(
            storage.items.into_iter()
                .map(|entry| async move {
                    if let Some(item) = ObjectTemplate::get(db, &entry.template_id).await? {
                        let mut instance = entry.instance.0;
                        instance.set_parent(Some(Arc::new(item.data)));
                        instance.clear_changes();
                        
                        Ok::<_, ItemStorageSessionError>((
                            entry.id, 
                            Some(Item {
                                id: entry.id,
                                base_item: (item.id, item.numeric_id),
                                instance,
                            })
                        ))
                    } else {
                        Ok((entry.id, None))
                    }
                })
                .collect::<Vec<_>>()
            ).await
            .into_iter()
            .collect::<Result<Vec<_>, ItemStorageSessionError>>()?
            .into_iter()
            .filter_map(|(id, item)| {
                if let Some(item) = item {
                    Some(item)
                } else {
                    storage_session.removed_items.push(id);
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|mut item| {
                match *item.instance.get::<_, i32>(ItemBase::ContainerId).unwrap() {
                    // Inventory
                    0 => {
                        let tab = storage_session.get_item_tab(&item.instance);
                        let id = item.id;
                        let idx = *item.instance.get::<_, i32>(ItemBase::InventorySlotIndex).unwrap() as usize;

                        debug!("Loading time {:#?}", item);
                        
                        if let Some(slot) = tab.slots.get_mut(idx) {
                            let item = Arc::new(Mutex::new(item));
                            *slot = Some(item.clone());
                            storage_session.items.insert(id, item);
                        } else if tab.can_grow {
                            let item = Arc::new(Mutex::new(item));

                            tab.slots.resize(idx + 1, None);
                            tab.slots[idx] = Some(item.clone());

                            storage_session.items.insert(id, item);
                        } else {
                            // Either a slot is already occupied or the index was outside the inventory
                            // bounds. Store item to be placed later.
                            unplaceable_items.push(item);
                        }
                    },

                    // Equipped
                    1 => {
                        let slot_mapping = item.instance.get::<_, String>(ItemBase::SlotMapping).unwrap().clone();

                        if
                            let Some(eq_type) = EQUIPMENT_SLOTS.get_equipment_type(&slot_mapping) &&
                            let Some(group) = storage_session.equipment.0.get_mut(&eq_type.slot_type) &&
                            group.iter().enumerate()
                                .all(|(idx, slot)| {
                                    if eq_type.slots.contains(&idx) {
                                        slot.is_none()
                                    } else {
                                        true
                                    }
                                })
                        {
                            let id = item.id;

                            if eq_type.slots.is_empty() {
                                let idx = *item.instance.get::<_, i32>(ItemBase::SlotId).unwrap() as usize;

                                if let Some(slot) = group.get_mut(idx) {
                                    let item = Arc::new(Mutex::new(item));
                                    *slot = Some(item.clone());
                                    storage_session.items.insert(id, item);
                                } else {
                                    unplaceable_items.push(item);
                                }
                            } else {
                                item.instance.set(ItemBase::SlotId, *eq_type.slots.first().unwrap() as i32);

                                let item = Arc::new(Mutex::new(item));

                                for idx in &eq_type.slots {
                                    group[*idx] = Some(item.clone());
                                }

                                storage_session.items.insert(id, item);
                            }

                        } else {
                            unplaceable_items.push(item);
                        }
                    },

                    _ => {
                        warn!("item {:?} has invalid container id {}", item.id, *item.instance.get::<_, i32>(ItemBase::ContainerId).unwrap());
                        unplaceable_items.push(item);
                    },
                }
            });

        // Place unplaceable items
        'place: for mut item in unplaceable_items {
            let tab = storage_session.get_item_tab(&item.instance);
            let id = item.id;

            debug!("Placing unplacable item {:?}", id);

            // Find free slot
            for (idx, slot) in tab.slots.iter_mut().enumerate() {
                if slot.is_none() {
                    item.instance.set(ItemBase::ContainerId, 0);
                    item.instance.set(ItemBase::InventorySlotIndex, idx as i32);

                    let item = Arc::new(Mutex::new(item));
                    *slot = Some(item.clone());
                    storage_session.items.insert(id, item);
                    continue 'place;
                }
            }

            // Add item even if there is no free slot
            item.instance.set(ItemBase::ContainerId, 0);
            item.instance.set(ItemBase::InventorySlotIndex, tab.slots.len() as i32);

            let item = Arc::new(Mutex::new(item));
            tab.slots.push(Some(item.clone()));
            storage_session.items.insert(id, item);
        }

        Ok(storage_session)
    }

    pub async fn start(db: &Database, id: Uuid) -> Result<Self, ItemStorageSessionError> {
        // For inventory actions, we start a fairly strict session to
        // ensure transactional consistency.
        let mut session = db.client()
            .start_session()
            .default_transaction_options(TransactionOptions::builder()
                .read_concern(ReadConcern::majority())
                .write_concern(WriteConcern::majority())
                .selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::Primary))
                .build()
            )
            .causal_consistency(true)
            .await?;

        session.start_transaction().await?;

        Self::init(db, Arc::new(Mutex::new(session)), id).await
    }

    pub fn owner(&self) -> &StorageOwner {
        &self.owner
    }

    fn get_item_tab(&mut self, item: &GameObjectData) -> &mut ItemTab {
        if 
            item.get::<_, String>(ItemBase::SlotMapping).ok()
                .and_then(|slot_mapping| EQUIPMENT_SLOTS.get_equipment_type(slot_mapping))
                .map(|eq_type| eq_type.slot_type.name() != "QBoost" && eq_type.slot_type.name() != "Hidden")
                .unwrap_or(true) &&
            item.get::<_, String>(ItemEdna::Skuid).ok()
                .and_then(|skuid| skuid.parse::<Uuid>().ok())
                .is_none()
        {
            &mut self.misc_items
        } else if *item.get::<_, bool>(ItemBase::IsQuestItem).unwrap() {
            &mut self.quest_items
        } else if 
            *item.get::<_, bool>(ItemBase::IsInGlobalShop).unwrap() ||
            *item.get::<_, bool>(ItemEdna::IsSku).unwrap() ||
            item.get::<_, String>(ItemEdna::Skuid).ok()
                .and_then(|skuid| skuid.parse::<Uuid>().ok())
                .is_none()
        {
            &mut self.cash_items
        } else if 
            *item.get::<_, bool>(ItemBase::IsRecipe).unwrap() ||
            *item.get::<_, bool>(ItemEdna::IsTemplate).unwrap_or(&false)
        {
            &mut self.schema_items            
        } else {
            unreachable!()
        }
    }

    pub async fn insert_item(&mut self, base_item: ObjectTemplate, _insert_at: Option<i32>) -> Result<Uuid, ItemStorageSessionError> {
        let mut item: Item = Item {
            id: Uuid::new(),
            base_item: (base_item.id, base_item.numeric_id),
            instance: GameObjectData::instantiate(&Arc::new(base_item.data)),
        };

        let tab = self.get_item_tab(&item.instance);
        let id = item.id;

        for (idx, slot) in tab.slots.iter_mut().enumerate() {
            if slot.is_none() {
                item.instance.set(ItemBase::ContainerId, 0);
                item.instance.set(ItemBase::InventorySlotIndex, idx as i32);
                item.instance.set(ItemBase::SlotId, -1);

                let item = Arc::new(Mutex::new(item));
                *slot = Some(item.clone());
                self.items.insert(id, item);

                return Ok(id);
            }
        }

        if tab.can_grow {
            let idx = tab.slots.len() as i32;

            item.instance.set(ItemBase::ContainerId, 0);
            item.instance.set(ItemBase::InventorySlotIndex, idx);
            item.instance.set(ItemBase::SlotId, -1);

            let item = Arc::new(Mutex::new(item));
            tab.slots.push(Some(item.clone()));
            self.items.insert(id, item);

            return Ok(id);
        }

        Err(ItemStorageSessionError::ClientError("#ItemAction.NotEnoughInventorySlots#", None))
    }

    pub async fn destroy_item(&mut self, item_id: Uuid) -> Result<(), ItemStorageSessionError> {
        if let Some(item) = self.items.remove(&item_id) {
            self.removed_items.push(item_id);

            let item = item.lock().await;
            let tab = self.get_item_tab(&item.instance);

            let idx = *item.instance.get::<_, i32>(ItemBase::InventorySlotIndex).unwrap() as usize;
            tab.slots[idx] = None;

            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn move_item(&mut self, item_id: Uuid, new_slot: i32) -> Result<(), ItemStorageSessionError> {
        if let Some(item) = self.items.get(&item_id).cloned() {
            let item = item.lock().await;
            let tab = self.get_item_tab(&item.instance);
            let prev_slot = *item.instance.get::<_, i32>(ItemBase::InventorySlotIndex).unwrap() as usize;

            drop(item);

            if new_slot > tab.slots.len() as i32 {
                if tab.can_grow {
                    tab.slots.resize(new_slot as usize, None);
                } else {
                    return Ok(())
                }
            }

            tab.slots.swap(prev_slot, new_slot as usize);

            if let Some(item) = &tab.slots[new_slot as usize] {
                item.set_inventory_slot_index(new_slot).await;
            }

            if let Some(item) = &tab.slots[prev_slot] {
                item.set_inventory_slot_index(prev_slot as i32).await;
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn unequip_item(&mut self, item_id: Uuid) -> Result<(), ItemStorageSessionError> {
        if let Some(item) = self.items.get(&item_id).cloned() {
            let equipment = if item.is_disguise().await {
                &mut self.disguise
            } else {
                &mut self.equipment
            };

            // Remove item from any slot
            for slots in equipment.0.values_mut() {
                for slot in slots.iter_mut() {
                    if let Some(item) = slot {
                        if item.id().await == item_id {
                            *slot = None;
                        }
                    }
                }
            }

            // Insert item into inventory
            let tab = self.get_item_tab(&item.lock().await.instance);
            for (idx, slot) in tab.slots.iter_mut().enumerate() {
                if slot.is_none() {
                    
                    item.set_container_id(0).await;
                    item.set_inventory_slot_index(idx as i32).await;
                    item.set_slot_id(-1).await;

                    *slot = Some(item.clone());
                    return Ok(());
                }
            }

            // No free slot found. Check if we can resize the inventory
            if tab.can_grow {
                item.set_container_id(0).await;
                item.set_inventory_slot_index(tab.slots.len() as i32).await;
                item.set_slot_id(-1).await;

                tab.slots.push(Some(item.clone()));

                Ok(())
            } else {
                Err(ItemStorageSessionError::ClientError("#ItemAction.NotEnoughInventorySlots#", None))
            }
        } else {
            Ok(())
        }
    }

    pub async fn equip_item(&mut self, item_id: Uuid, idx: Option<i32>) -> Result<(), ItemStorageSessionError> {
        if 
            let Some(item) = self.items.get(&item_id).cloned() &&
            let Some(eq_type) = item.slot_mapping().await
        {
            {
                // Clear all slots that are going to be occupied by the item
                let equipment = if item.is_disguise().await {
                    &mut self.disguise
                } else {
                    &mut self.equipment
                };

                let slots = equipment.0.get(&eq_type.slot_type).unwrap().clone();
                if let Some(idx) = idx && eq_type.slots.is_empty() {
                    if let Some(Some(item)) = slots.get(idx as usize) {
                        self.unequip_item(item.id().await).await?;
                    }
                } else {
                    for idx in &eq_type.slots {
                        if let Some(Some(item)) = slots.get(*idx) {
                            self.unequip_item(item.id().await).await?;
                        }
                    }
                }
            }

            // Insert item into slots
            let equipment = if item.is_disguise().await {
                &mut self.disguise
            } else {
                &mut self.equipment
            };

            let slots = equipment.0.get_mut(&eq_type.slot_type).unwrap();
            if let Some(idx) = idx && eq_type.slots.is_empty() {
                if let Some(slot) = slots.get_mut(idx as usize) {
                    item.set_container_id(1).await;
                    item.set_inventory_slot_index(-1).await;
                    item.set_slot_id(idx).await;

                    *slot = Some(item);
                    Ok(())
                } else {
                    Err(ItemStorageSessionError::Other(anyhow!("invalid slot index {}", idx)))
                }

            } else {
                item.set_container_id(1).await;
                item.set_inventory_slot_index(-1).await;
                item.set_slot_id(*eq_type.slots.first().unwrap() as i32).await;

                for idx in &eq_type.slots {
                    if let Some(slot) = slots.get_mut(*idx) {
                        *slot = Some(item.clone());
                    }
                }

                Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[allow(unused)]
    pub async fn transfer_item(&mut self, _item_id: Uuid, _new_storage_id: Uuid, _new_slot: i32) -> Result<(), ItemStorageSessionError> {
        todo!()
    }

    async fn write(&self) -> Result<ItemStorageSessionResult, ItemStorageSessionError> {
        let mut result = ItemStorageSessionResult {
            id: self.id,
            bling: self.bling,
            game_cash: self.game_cash,
            _capacity: self.capacity,
            removed_items: self.removed_items.clone(),
            changed_items: Vec::new(),
        };

        let mut storage = ItemStorage {
            id: self.id,
            name: self.name.clone(),
            owner: self.owner,
            bling: self.bling,
            game_cash: self.game_cash,
            capacity: self.capacity,
            items: Vec::new(),
        };

        // Insert items into db structure
        for item in self.items.values() {
            let item = item.lock().await;
            let db_item = crate::db::Item {
                id: item.id,
                template_id: item.base_item.0,
                instance: async_graphql::Json(item.instance.clone()),
            };

            if item.instance.changes().next().is_some() {
                result.changed_items.push(db_item.clone());
            }

            storage.items.push(db_item);
        }

        let mut session = self.session.lock().await;

        ItemStorage::collection(&self.db)
            .find_one_and_update(doc! {"id": storage.id}, doc! { "$set": bson::to_bson(&storage)? })
            .upsert(true)
            .session(session.deref_mut())
            .await?;

        Ok(result)
    }

    pub async fn commit(self) -> Result<Vec<ItemStorageSessionResult>, ItemStorageSessionError> {
        let mut results = Vec::new();
        
        results.push(self.write().await?);
        for sub_session in self.sub_sessions {
            results.push(sub_session.write().await?);
        }

        self.session.lock().await.commit_transaction().await?;

        Ok(results)
    }

    pub async fn write_uncommitted(mut self) -> Result<(ClientSession, Vec<ItemStorageSessionResult>), ItemStorageSessionError> {
        let mut results = Vec::new();
        
        results.push(self.write().await?);
        for sub_session in self.sub_sessions.drain(..) {
            results.push(sub_session.write().await?);
        }

        Ok((Arc::into_inner(self.session).unwrap().into_inner(), results))
    }
}

pub struct ItemStorageSessionResult {
    pub id: Uuid,
    pub removed_items: Vec<Uuid>,
    pub changed_items: Vec<crate::db::Item>,
    pub bling: Option<i32>,
    pub game_cash: Option<i32>,
    pub _capacity: i32,
}