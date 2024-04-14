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

use std::sync::mpsc::{self, Sender};

use atlas::{ItemBaseParams, ParamBox, PlayerComponent, PlayerParams, Slot, Uuid};
use bevy::app::{First, Plugin, PostUpdate};
use bevy_ecs::{component::Component, entity::Entity, event::{Event,Events}, query::{Added, With}, removal_detection::RemovedComponents, schedule::IntoSystemConfigs, system::{Commands, Query, Res, ResMut, Resource}};
use futures::TryStreamExt;
use log::{debug, error};

use crate::{actors::{zone::{plugins::BehaviorExt, resources::Tasks}, AvatarComponent, EntityType, EventChannelExtension}, db::{get_cached_item, get_cached_item_by_id, realm_database, InventoryEntry}};

use super::{discard_item, do_vendor_execute, process_buy_request, request_equip, request_unequip, spawn_inventory_entry, update_inventory_item_pos, Equipped, InventoryTab, ItemPurchaseRequest, ItemSellRequest, PlayerInventory, PlayerLoadout};

#[derive(Component)]
pub struct Item {
    pub(in crate::actors::zone::plugins::items) id: Uuid,
    pub(in crate::actors::zone::plugins::items) template: Uuid,
    pub(in crate::actors::zone::plugins::items) owner: Entity
}

impl Item {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn template_id(&self) -> &Uuid {
        &self.template
    }

    pub fn owner(&self) -> Entity {
        self.owner
    }
}

#[derive(Resource)]
struct InventoryQueryResultSender(Sender<InventoryQueryResult>);

#[derive(Event)]
struct InventoryQueryResult(Entity, Vec<InventoryEntry>);

#[derive(Event)]
pub struct ItemEquipped { 
    pub avatar: Entity,
    pub item: Entity,
}

#[derive(Event)]
pub struct ItemUnequipped { 
    pub avatar: Entity,
    pub item: Entity,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (entry_sender, entry_receiver) = mpsc::channel::<InventoryQueryResult>();

        app.add_systems(First, (
            load_player_inventory, 
            insert_player_inventory, 
            process_buy_request, 
            cleanup_player_inventory
        ));

        app.add_event::<ItemPurchaseRequest>();
        app.add_event::<ItemSellRequest>();

        app.add_event_channel(entry_receiver);

        app.insert_resource(InventoryQueryResultSender(entry_sender));

        app.add_behavior(EntityType::Player, "inventoryItemPos", update_inventory_item_pos);
        app.add_behavior(EntityType::Player, "RequestDiscardItem", discard_item);
        app.add_behavior(EntityType::Player, "RequestEquip", request_equip);
        app.add_behavior(EntityType::Player, "RequestUnequip", request_unequip);

        app.add_behavior(EntityType::NpcOtherland, "doVendorExecute", do_vendor_execute);
    }
}

fn load_player_inventory(
    tasks: Res<Tasks>,
    entry_query_sender: Res<InventoryQueryResultSender>,
    players: Query<(Entity, &AvatarComponent), Added<PlayerComponent>>
) {
    for (entity, avatar) in players.iter() {
        let character_id = avatar.record_id.unwrap();
        let entry_query_sender = entry_query_sender.0.clone();
        
        // asynchronously read the inventory database and emit records
        tasks.tasks.spawn(async move {
            match InventoryEntry::get_player_inventory(realm_database().await, character_id).await {
                Ok(mut cursor) => {
                    let mut entries = Vec::new();

                    while let Ok(Some(entry)) = cursor.try_next().await {
                        entries.push(entry);
                    }

                    debug!("Player inventory query finished");
                    let _ = entry_query_sender.send(InventoryQueryResult(entity, entries));
                },
                Err(e) => {
                    error!("Failed to read inventory for character {}: {:?}", character_id, e);
                }
            }
        });
    }
}

fn insert_player_inventory(
    mut entries: ResMut<Events<InventoryQueryResult>>,
    players: Query<&ParamBox, With<PlayerComponent>>,
    mut commands: Commands,
) {
    for InventoryQueryResult(entity, inventory_result) in entries.drain() {
        if let Ok(player) = players.get(entity)
            .map(|p| p.get_impl::<dyn PlayerParams>().unwrap()) {
            let mut inventory = PlayerInventory::new(player.inventory_size() as usize);
            let mut loadout = PlayerLoadout::new();

            for item in inventory_result {
                match item.params.get_impl::<dyn ItemBaseParams>().unwrap().container_id() {
                    // inventory
                    0 => {
                        // get inventory base
                        if let Ok(slot_idx) = usize::try_from(item.params.get_impl::<dyn ItemBaseParams>().unwrap().inventory_slot_index()) {
                            let tab = InventoryTab::for_item(&item.params);
                            let entity = spawn_inventory_entry(&mut commands, item.params)
                                .insert(Item {
                                    id: item.id,
                                    template: item.template,
                                    owner: entity,
                                })
                                .id();

                            inventory.insert_at(tab, item.id, slot_idx, item.template, entity);
                        }
                    },

                    // equipment
                    1 => {
                        let entity = spawn_inventory_entry(&mut commands, item.params)
                            .insert(Item {
                                id: item.id,
                                template: item.template,
                                owner: entity,
                            })
                            .insert(Equipped)
                            .id();

                        if let Some(template_item) = get_cached_item(&item.template) {
                            loadout.add(super::ItemReference::InventoryItem((item.id, template_item.id as i32, entity)));
                        }
                    },

                    _ => {
                        unimplemented!()
                    }
                }
            }

            // add visual only items to loadout
            player.visible_item_info()
                .unwrap_or(&[])
                .iter()
                .filter_map(|id| get_cached_item_by_id(*id))
                .filter_map(|item| item.data.as_ref().map(|data| (item.id as i32, data)))
                .filter_map(|(id, params)| params.get_impl::<dyn ItemBaseParams>().map(|params| (id, params)))
                .filter_map(|(id, params)| {
                    params.slot_mapping()
                        .and_then(|slot| slot.parse::<Slot>().ok())
                        .map(|slot| (id, slot))
                })
                .filter(|(id, slot)| slot.is_base_appearance())
                .for_each(|(id, _)| {
                    loadout.add(super::ItemReference::VisualOnly(id));
                });

            // add player inventory
            commands.entity(entity).insert((inventory, loadout));
        }
    }
}

fn cleanup_player_inventory(
    mut removals: RemovedComponents<PlayerInventory>,
    items: Query<(Entity, &Item)>,
    mut cmds: Commands,
) {
    for removed_player in removals.read() {
        for (ent, item) in items.iter() {
            if item.owner == removed_player {
                cmds.entity(ent).despawn();
            }
        }
    }
}


