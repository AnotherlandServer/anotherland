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

use atlas::{get_item_category, BundleItemClass, BundleItemParams, ClassId, ClassItemClass, EdnaBaseClass, EdnaFunctionClass, EdnaModuleClass, EdnaModuleParams, ItemBaseComponent, ItemBaseParams, ItemEdnaClass, ItemMyLandThemeClass, ItemSubCategory, MinigameItemClass, ParamBox, ParamClass, PlayerComponent, PortalItemClass, SomaforgeItemClass, Uuid};
use bevy::utils::hashbrown::HashMap;
use bevy_ecs::{component::Component, entity::Entity, event::EventReader, query::{With, Without}, system::{Commands, EntityCommands, Query}};
use log::warn;

use crate::{actors::zone::plugins::CreationPending, db::ItemContent, util::{AnotherlandError, AnotherlandResult}};

use super::{Item, ItemEquipped, ItemUnequipped};

static CASH_ITEM_CATEGORIES: &[ItemSubCategory] = &[
    ItemSubCategory::Metamorph,
    ItemSubCategory::Bundle,
    ItemSubCategory::CostumeBundle,
    ItemSubCategory::MetaMorphVoucher,
];

static RECIPE_ITEM_CATEGORIES: &[ItemSubCategory] = &[
    ItemSubCategory::TorsoSchematic,
    ItemSubCategory::LegsSchematic,
    ItemSubCategory::HeadSchematic,
    ItemSubCategory::AugmentSchematic,
    ItemSubCategory::ConsumableSchematic,
    ItemSubCategory::BundleSchematic,
];

#[derive(Clone, Copy)]
pub enum InventoryTab {
    Gear,
    Quests,
    Cash,
    Recipe,
}

impl InventoryTab {
    pub fn for_item(item: &ParamBox) -> Self {
        if item.get_impl::<dyn BundleItemParams>().is_some() {
            Self::Cash
        } else if 
            let Some(edna_module) = item.get_impl::<dyn EdnaModuleParams>() &&
            edna_module.is_sku() 
        {
            Self::Cash
        } else if item.get_impl::<dyn ItemBaseParams>().unwrap().is_recipe() {
            Self::Recipe
        } else if item.get_impl::<dyn ItemBaseParams>().unwrap().is_quest_item() {
            Self::Quests
        } else {
            // check item category
            if let Some(category) = item.get_impl::<dyn ItemBaseParams>()
                .and_then(|id| get_item_category(id.category())) {

                if CASH_ITEM_CATEGORIES.contains(&category.main()) {
                    Self::Cash
                } else if RECIPE_ITEM_CATEGORIES.contains(&category.main()) {
                    Self::Recipe
                } else {
                    Self::Gear
                }
            } else {
                Self::Gear
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct PlayerInventory {
    items: HashMap<Uuid, (InventoryTab, Uuid, Entity)>,

    gear: Vec<Option<Entity>>,
    quests: Vec<Option<Entity>>,
    cash: Vec<Option<Entity>>,
    recipes: Vec<Option<Entity>>,
}

impl PlayerInventory {
    pub fn new(slots: usize) -> Self {
        PlayerInventory {
            items: HashMap::new(),
            gear: [None].repeat(slots),
            quests: [None].repeat(30),
            cash: [None].repeat(30),
            recipes: [None].repeat(30),
        }
    }

    pub fn lookup_item_id(&self, id: Uuid) -> Option<(InventoryTab, Uuid, Entity)> {
        self.items.get(&id).cloned()
    }

    pub fn remove_item(&mut self, id: Uuid) -> Option<Entity> {
        if let Some((tab, _, entity)) = self.items.remove(&id) {
            self.get_tab_mut(tab).iter_mut().for_each(|ent| {
                if *ent == Some(entity) {
                    *ent = None;
                }
            });

            Some(entity)
        } else {
            None
        }
    }

    pub fn remove_item_of_kind(&mut self, template_id: Uuid) -> Option<Entity> {
        let id = self.items
            .iter()
            .find(|(_, (_, template, _))| {
                *template == template_id
            })
            .map(|(id, _)| *id);

        if 
            let Some(id) = id &&
            let Some((tab, _, entity)) = self.items.remove(&id) 
        {
            self.get_tab_mut(tab).iter_mut().for_each(|ent| {
                if *ent == Some(entity) {
                    *ent = None;
                }
            });

            Some(entity)
        } else {
            None
        }
    }

    pub fn get_tab(&self, tab: InventoryTab) -> &[Option<Entity>] {
        match tab {
            InventoryTab::Gear => &self.gear,
            InventoryTab::Quests => &self.quests,
            InventoryTab::Cash => &self.cash,
            InventoryTab::Recipe => &self.recipes,
        }
    }

    pub fn get_tab_mut(&mut self, tab: InventoryTab) -> &mut [Option<Entity>] {
        match tab {
            InventoryTab::Gear => &mut self.gear,
            InventoryTab::Quests => &mut self.quests,
            InventoryTab::Cash => &mut self.cash,
            InventoryTab::Recipe => &mut self.recipes,
        }
    }

    fn next_free_slot(&mut self, tab: InventoryTab) -> Option<(usize, &mut Option<Entity>)> {
        match tab {
            InventoryTab::Gear => self.gear
                .iter_mut()
                .enumerate()
                .find(|(_, i)| i.is_none()),
            InventoryTab::Quests |
            InventoryTab::Cash |
            InventoryTab::Recipe => {
                // Quest, cash and recipe tabs seem not limited in size.
                // Do we need to resize the tab?
                if self.get_tab(tab).iter().all(Option::is_some) {
                    match tab {
                        InventoryTab::Quests => self.quests.resize(self.quests.len() + 6, None),
                        InventoryTab::Cash => self.cash.resize(self.cash.len() + 6, None),
                        InventoryTab::Recipe => self.recipes.resize(self.recipes.len() + 6, None),
                        _ => unreachable!(),
                    }
                }

                self.get_tab_mut(tab)
                    .iter_mut()
                    .enumerate()
                    .find(|(_, i)| i.is_none())
            },
        }
    }

    pub fn insert_new(&mut self, owner: Entity, cmds: &mut Commands, mut item: ItemContent) -> AnotherlandResult<Entity> {
        let mut params = item.data.take().unwrap();
        let tab = InventoryTab::for_item(&params);

        // store item in a free slot
        if let Some((idx, slot)) = self.next_free_slot(tab) {
            // store slot index in item params, so the client can display the item in the
            // correct slot.
            if let Some(base) = params.get_impl_mut::<dyn ItemBaseParams>() {
                base.set_inventory_slot_index(idx as i32);
                base.set_container_id(0);
            }

            let item_id = Uuid::new();
            let entity = spawn_inventory_entry(cmds, params)
                .insert(Item {
                    id: item_id,
                    template: item.guid,
                    owner,
                })
                .insert(CreationPending)
                .id();

            *slot = Some(entity);
            
            self.items.insert(item_id, (tab, item.guid, entity));
            Ok(entity)
        } else {
            Err(AnotherlandError::app_err("#ItemAction.NotEnoughInventorySlots#"))
        }
    }

    pub fn insert_at(&mut self, tab: InventoryTab, id: Uuid, idx: usize, template_id: Uuid, entity: Entity) {
        self.items.insert(id, (tab, template_id, entity));

        match tab {
            InventoryTab::Gear => {
                if let Some(slot) = self.gear.get_mut(idx) {
                    *slot = Some(entity);
                } else {
                    panic!("Tried to insert outside inventory bounds. Idx {} size {}", idx, self.gear.len());
                }
            },
            InventoryTab::Quests |
            InventoryTab::Cash |
            InventoryTab::Recipe => {
                // Quest, cash and recipe tabs seem not limited in size.
                // Do we need to resize the tab?
                if self.get_tab(tab).len() <= idx {
                    let required_size = (idx / 6 + 1) * 6;

                    match tab {
                        InventoryTab::Quests => self.quests.resize(required_size, None),
                        InventoryTab::Cash => self.cash.resize(required_size, None),
                        InventoryTab::Recipe => self.recipes.resize(required_size, None),
                        _ => unreachable!(),
                    }
                }

                self.get_tab_mut(tab)[idx] = Some(entity);
            },
        }
    }

    pub fn insert(&mut self, id: Uuid, template_id: Uuid, entity: Entity, params: &ParamBox) -> AnotherlandResult<usize> {
        let tab = InventoryTab::for_item(&params);
        self.items.insert(id, (tab, template_id, entity));

        if let Some((idx, slot)) = self.next_free_slot(tab) {
            *slot = Some(entity);

            Ok(idx)
        } else {
            Err(AnotherlandError::app_err("#ItemAction.NotEnoughInventorySlots#"))
        }
    }
}

pub fn spawn_inventory_entry<'w, 's, 'a>(commands: &'a mut Commands<'w, 's>, item: ParamBox) -> EntityCommands<'w, 's, 'a> {
    match item.class_id() {
        ClassId::ClassItemClass => commands.spawn(item.take::<ClassItemClass>().unwrap().into_bundle()),
        ClassId::PortalItemClass => commands.spawn(item.take::<PortalItemClass>().unwrap().into_bundle()),
        ClassId::ItemMyLandThemeClass => commands.spawn(item.take::<ItemMyLandThemeClass>().unwrap().into_bundle()),
        ClassId::MinigameItemClass => commands.spawn(item.take::<MinigameItemClass>().unwrap().into_bundle()),
        ClassId::EdnaBaseClass => commands.spawn(item.take::<EdnaBaseClass>().unwrap().into_bundle()),
        ClassId::EdnaFunctionClass => commands.spawn(item.take::<EdnaFunctionClass>().unwrap().into_bundle()),
        ClassId::EdnaModuleClass => commands.spawn(item.take::<EdnaModuleClass>().unwrap().into_bundle()),
        ClassId::SomaforgeItemClass => commands.spawn(item.take::<SomaforgeItemClass>().unwrap().into_bundle()),
        ClassId::BundleItemClass => commands.spawn(item.take::<BundleItemClass>().unwrap().into_bundle()),
        ClassId::ItemEdnaClass => commands.spawn(item.take::<ItemEdnaClass>().unwrap().into_bundle()),
        _ => unimplemented!(),
    }
}
