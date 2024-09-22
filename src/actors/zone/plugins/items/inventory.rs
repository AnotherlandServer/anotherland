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

use atlas::{get_item_category, BundleItemClass, BundleItemParams, ClassId, ClassItemClass, EdnaBaseClass, EdnaFunctionClass, EdnaModuleClass, EdnaModuleParams, ItemBaseComponent, ItemBaseParams, ItemEdnaClass, ItemEdnaParams, ItemMyLandThemeClass, ItemSubCategory, MinigameItemClass, ParamBox, ParamClass, PlayerComponent, PlayerParams, PortalItemClass, Slot, SomaforgeItemClass, Uuid, UUID_NIL};
use bevy::utils::hashbrown::HashMap;
use bevy_ecs::{component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, query::{With, Without}, system::{Commands, EntityCommands, In, Query}};
use log::warn;

use crate::{actors::zone::plugins::{CreationPending, GameMessage, PlayerController, RemovalPending}, db::{get_cached_item, get_cached_item_by_id, ItemContent}, util::{AnotherlandError, AnotherlandResult}};

use super::{Equipped, Item, ItemEquipped, ItemReference, ItemUnequipped, PlayerDisguise, PlayerLoadout};

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

#[derive(Event)]
pub struct AwardItemTransaction { 
    pub entity: Entity,
    pub template_id: Uuid,
}


#[derive(Event)]
pub struct AwardItemAndEquipTransaction { 
    pub entity: Entity,
    pub template_id: Uuid,
}

#[derive(Event)]
pub struct EquipItemTransaction {
    pub player: Entity,
    pub item: Entity,
}

#[derive(Event)]
pub struct UnequipItemTransaction {
    pub player: Entity,
    pub item: Entity,
}

#[derive(Event)]
pub struct MoveItemTransaction {
    pub entity: Entity,
    pub id: Uuid, 
    pub dest_idx: usize,
}

#[derive(Event)]
pub struct RemoveItemTransaction { 
    pub entity: Entity,
    pub id: Uuid,
}

#[derive(Event)]
pub struct AwardStartEquipmentTransaction { 
    pub entity: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn remove_item(&mut self, tab: InventoryTab, entity: Entity) -> Option<Uuid> {
        if let Some((&id, _)) = self.items.iter().find(|(_, (inv_tab, _, ent))| *inv_tab == tab && *ent == entity) {
            self.get_tab_mut(tab).iter_mut().for_each(|ent| {
                if *ent == Some(entity) {
                    *ent = None;
                }
            });

            self.items.remove(&id);
            Some(id)
        } else {
            None
        }
    }

    pub fn remove_item_by_id(&mut self, id: Uuid) -> Option<Entity> {
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

    pub fn has_free_slots(&self, tab: InventoryTab) -> bool {
        self.get_tab(tab).iter()
            .any(|v| v.is_none())
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
            Err(AnotherlandError::app_err("No inventory slots"))
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

pub fn spawn_inventory_entry<'w, 's, 'a>(commands: &'a mut Commands<'w, 's>, item: ParamBox) -> EntityCommands<'a> {
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

pub fn perform_award_item_transactions(
    mut ev: EventReader<AwardItemTransaction>,
    mut players: Query<(&mut PlayerInventory, &PlayerController)>,
    mut cmds: Commands,
) {
    for &AwardItemTransaction { entity, template_id } in ev.read() {
        if 
            let Some(item) = get_cached_item(&template_id) &&
            let Ok((mut inventory, controller)) = players.get_mut(entity) &&
            inventory.insert_new(entity, &mut cmds, item.clone()).is_err()
        {
            // todo: Normally items should only be awarded when there is enough inventory space.
            //       In case we fail here, send item via mail.
            controller.send_game_message(GameMessage::PopUp("#ItemAction.NotEnoughInventorySlots#".to_string()));
        }
    }
}

pub fn perform_award_item_and_equip_transactions(
    mut ev: EventReader<AwardItemAndEquipTransaction>,
    mut ev_equip: EventWriter<EquipItemTransaction>,
    mut players: Query<(&mut PlayerInventory, &PlayerController)>,
    mut cmds: Commands,
) {
    for &AwardItemAndEquipTransaction { entity, template_id } in ev.read() {
        if 
            let Some(item) = get_cached_item(&template_id) &&
            let Ok((mut inventory, controller)) = players.get_mut(entity)
        {
            if let Ok(item) = inventory.insert_new(entity, &mut cmds, item.clone()) {
                // immediately equip item
                ev_equip.send(EquipItemTransaction { player: entity, item });
            } else {
                // todo: Normally items should only be awarded when there is enough inventory space.
                //       In case we fail here, send item via mail.
                controller.send_game_message(GameMessage::PopUp("#ItemAction.NotEnoughInventorySlots#".to_string()));
            }
        }
    }
}

pub fn perform_equip_item_transactions(
    mut ev: EventReader<EquipItemTransaction>,
    mut players: Query<(&mut ParamBox, &mut PlayerInventory, &mut PlayerLoadout, &mut PlayerDisguise, &PlayerController), (With<PlayerComponent>, Without<ItemBaseComponent>)>,
    mut items: Query<(&Item, &mut ParamBox), (With<ItemBaseComponent>, Without<PlayerComponent>)>,
    mut cmds: Commands,
) {
    for &EquipItemTransaction { player, item } in ev.read() {
        if 
            let Ok((mut params, mut inventory, mut loadout, mut disguise, controller)) = players.get_mut(player) &&
            let Ok((item_info, item_params)) = items.get(item)
        {
            // copy inventory and loadout state so we can safely
            // try to perform the requested action and don't need to rollback
            // in case it fails.
            let mut new_inventory_state = inventory.clone();
            let mut new_loadout_state = loadout.clone();
            let mut new_disguise_state = disguise.clone();

            
            // take item out of inventory
            let item_ent = if let Some(ent) = new_inventory_state.remove_item_by_id(item_info.id) {
                ent
            } else {
                warn!("Tried to equip item not in inventory! Id: {}", item_info.id);
                return
            };

            // lookup template item
            let template_item = if let Some(item) = get_cached_item(&item_info.template) {
                item
            } else {
                warn!("Tried to equip item, but item definition not found. Id: {}", item_info.id);
                return
            };

            // equip item
            let item_ref = ItemReference::InventoryItem((
                item_info.id, 
                template_item.id as i32, 
                item_ent
            ));

            let replaced_items = 
                if item_params.get_impl::<dyn ItemEdnaParams>().map(|params| params.disguise() == 0).unwrap_or_default() {
                    new_disguise_state.add(item_ref)
                } else {
                    new_loadout_state.add(item_ref)
                };

            // place replaced items back in inventory
            let mut inventory_slots = Vec::new();
            for item in replaced_items {
                if 
                    let ItemReference::InventoryItem((id, _, ent)) = item &&
                    let Ok((item_info, mut item_params)) = items.get_mut(ent)
                {
                    match new_inventory_state.insert(id, item_info.template, ent, &mut item_params) {
                        Ok(idx) => {
                            inventory_slots.push((ent, idx));
                        },
                        Err(e) => {
                            warn!("Failed to place unequipped item in inventory: {:?}", e);
                            controller.send_game_message(GameMessage::PopUp("#RequestUnequip.Your_inventory_is_full#".to_string()));
                            return;
                        }
                    }
                }
            }

            // apply changes
            new_inventory_state.clone_into(inventory.as_mut());
            new_loadout_state.clone_into(loadout.as_mut());
            new_disguise_state.clone_into(disguise.as_mut());

            for (ent, slot_idx) in inventory_slots {
                if 
                    let Ok((_, mut item)) = items.get_mut(ent) &&
                    let Some(item_base) = item.get_impl_mut::<dyn ItemBaseParams>() 
                {
                    item_base.set_container_id(0);
                    item_base.set_inventory_slot_index(slot_idx as i32);
                    item_base.set_slot_id(-1);
                }
            }

            if 
                let Ok((_, mut item_params)) = items.get_mut(item_ent) &&
                let Some(item_base) = item_params.get_impl_mut::<dyn ItemBaseParams>()
            {
                let item_slot = item_base.slot_mapping()
                    .parse::<Slot>()
                    .unwrap();

                item_base.set_container_id(1);
                item_base.set_inventory_slot_index(-1);
                item_base.set_slot_id(item_slot.id());
            }

            cmds.entity(item_ent).insert(Equipped);

            // update avatar visuals
            let player_params = params.get_impl_mut::<dyn PlayerParams>().unwrap();

            player_params.set_visible_item_info(
                [loadout.compile_visual_items(), disguise.compile_visual_items()].concat()
            );
            
            if let Some(ItemReference::InventoryItem((id, class, item_ent))) = loadout.get_weapon() {
                player_params.set_stat_wep_max_dmg(10.0);
                player_params.set_stat_weapon_dps(5.0);
                player_params.set_last_equipped_weapon(id);
            }
        }
    }
}


pub fn process_unequip_item_transactions(
    mut ev: EventReader<UnequipItemTransaction>,
    mut players: Query<(&mut ParamBox, &mut PlayerInventory, &mut PlayerLoadout, &mut PlayerDisguise, &PlayerController), (With<PlayerComponent>, Without<ItemBaseComponent>)>,
    mut items: Query<(&Item, &mut ParamBox), (With<ItemBaseComponent>, With<Equipped>, Without<PlayerComponent>)>,
    mut cmds: Commands,
) {
    for &UnequipItemTransaction { player, item } in ev.read() {
        if 
            let Ok((mut params, mut inventory, mut loadout, mut disguise, controller)) = players.get_mut(player) &&
            let Ok((item_info, mut item_params)) = items.get_mut(item)
        {
            // copy inventory and loadout state so we can safely
            // try to perform the requested action and don't need to rollback
            // in case it fails.
            let mut new_inventory_state = inventory.clone();
            let mut new_loadout_state = loadout.clone();
            let mut new_disguise_state = disguise.clone();

            // unequip item
            let item_ent = if let Some(ItemReference::InventoryItem((_, _, item_ent))) = new_loadout_state.remove_inventory_item(item_info.id) {
                item_ent
            } else if let Some(ItemReference::InventoryItem((_, _, item_ent))) = new_disguise_state.remove_inventory_item(item_info.id) {
                item_ent
            } else {
                warn!("Requested unequip of item not in loadout. Id: {}", item_info.id);
                return
            };

            // place item in inventory
            match new_inventory_state.insert(item_info.id, item_info.template, item_ent, &item_params) {
                Ok(idx) => {
                    if let Some(item_base) = item_params.get_impl_mut::<dyn ItemBaseParams>() {
                        item_base.set_container_id(0);
                        item_base.set_inventory_slot_index(idx as i32);
                        item_base.set_slot_id(-1);
                    }
                },
                Err(e) => {
                    warn!("Failed to place unequipped item in inventory: {:?}", e);
                    controller.send_game_message(GameMessage::PopUp("#RequestUnequip.Your_inventory_is_full#".to_string()));
                    return;
                }
            }
            
            // apply changes
            new_inventory_state.clone_into(inventory.as_mut());
            new_loadout_state.clone_into(loadout.as_mut());
            new_disguise_state.clone_into(disguise.as_mut());

            cmds.entity(item_ent).remove::<Equipped>();

            // update avatar visuals
            let player_params = params.get_impl_mut::<dyn PlayerParams>().unwrap();

            player_params.set_visible_item_info(
                [loadout.compile_visual_items(), disguise.compile_visual_items()].concat()
            );

            if loadout.get_weapon().is_none() {
                player_params.set_stat_wep_max_dmg(0.0);
                player_params.set_stat_weapon_dps(0.0);
                player_params.set_last_equipped_weapon(UUID_NIL);
            }
        }
    }
}

pub fn process_remove_item_transaction(
    mut ev: EventReader<RemoveItemTransaction>,
    mut players: Query<&mut PlayerInventory>,
    mut cmds: Commands,
) {
    for &RemoveItemTransaction { entity, id } in ev.read() {
        if 
            let Ok(mut inventory) = players.get_mut(entity) &&
            let Some(entity) = inventory.remove_item_by_id(id)
        {
            // mark item for removal
            cmds.entity(entity).insert(RemovalPending);
        }
    }
}

pub fn award_start_equipment(
    In(entity): In<Entity>,
    mut players: Query<(&mut PlayerInventory, &mut PlayerLoadout)>,
    mut ev_award_and_equip: EventWriter<AwardItemAndEquipTransaction>,
    mut ev_award: EventWriter<AwardItemTransaction>,
    mut cmds: Commands,
) {
    if let Ok((mut inventory, mut loadout)) = players.get_mut(entity) {
        // remove all currently equipped items
        for item_ref in loadout.remove_all_items() {
            if let ItemReference::InventoryItem((_, _, item)) = item_ref {
                cmds.entity(item)
                    .insert(RemovalPending);
            }
        }

        // clear gear inventory tab
        for item in inventory.get_tab(InventoryTab::Gear).to_vec().into_iter().flatten().clone() {
            inventory.remove_item(InventoryTab::Gear, item);
            cmds.entity(item)
                .insert(RemovalPending);
        }

        
        // add default gear
        ev_award_and_equip.send(AwardItemAndEquipTransaction { 
            entity, 
            template_id: Uuid::parse_str("1d48a935-7f91-4e7e-b1e9-f8cdb6f23b09").unwrap() // ShirtTextureOnly0003Default0001
        });

        ev_award_and_equip.send(AwardItemAndEquipTransaction { 
            entity, 
            template_id: Uuid::parse_str("a5d9d67e-9fcc-433e-bb04-e74562e0b674").unwrap() // PantsTextureOnly0011HackerBlack0001
        });

        ev_award_and_equip.send(AwardItemAndEquipTransaction { 
            entity, 
            template_id: Uuid::parse_str("0435719c-6ed0-40fb-b7c6-74acfaaa8160").unwrap() // ShoesTextureOnly0001Default0001
        });


        ev_award.send(AwardItemTransaction { 
            entity, 
            template_id: Uuid::parse_str("292837be-2ca6-49d6-b9d7-19b27fa9c642").unwrap()
        });

        /*ev_award_and_equip.send(AwardItemAndEquipTransaction { 
            entity, 
            template_id: Uuid::parse_str("292837be-2ca6-49d6-b9d7-19b27fa9c642").unwrap() // club
        });*/
    }
}