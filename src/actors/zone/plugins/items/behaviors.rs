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

use atlas::{ItemBaseComponent, ItemBaseParams, NativeParam, ParamBox, PlayerComponent, PlayerParams, Slot, Uuid};
use bevy_ecs::{query::With, system::{Commands, In, Query, Res}};
use bson::doc;
use log::{debug, error, warn};

use crate::{actors::{zone::{plugins::{Behavior, BehaviorArguments, LoadoutBuilder}, resources::Tasks}, RealmDatabase}, db::{get_cached_item, get_cached_item_by_id, DatabaseRecord, InventoryEntry}};

use super::PlayerInventory;

pub fn update_inventory_item_pos(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut inventory: Query<&mut PlayerInventory>,
    mut items: Query<&mut ParamBox, With<ItemBaseComponent>>,
) {
    if 
        let Behavior::String(_, args) = behavior && 
        let Ok(mut inventory) = inventory.get_mut(instigator) &&
        let Some((tab, _, entity)) = inventory.lookup_item_id(Uuid::parse_str(&args[0]).unwrap())
    {
        let dst_idx: usize = args[1].parse().unwrap();

        let inventory_tab = inventory.get_tab_mut(tab);
        let (src_idx, _) = inventory_tab.iter().enumerate().find(|(_, e)| **e == Some(entity)).unwrap();

        // swap slots
        inventory_tab.swap(src_idx, dst_idx);

        // update item params to new slots as needed
        if let Some(item) = inventory_tab[src_idx] &&
            let Ok(mut item) = items.get_mut(item) {

            item.get_impl_mut::<dyn ItemBaseParams>().unwrap().set_inventory_slot_index(src_idx as i32);
        }

        if let Some(item) = inventory_tab[dst_idx] &&
            let Ok(mut item) = items.get_mut(item) {

            item.get_impl_mut::<dyn ItemBaseParams>().unwrap().set_inventory_slot_index(dst_idx as i32);
        }
    }
}

pub fn discard_item(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut inventory: Query<&mut PlayerInventory>,
    mut cmds: Commands,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
) {
    if 
        let Behavior::String(_, args) = behavior && 
        let Ok(mut inventory) = inventory.get_mut(instigator) 
    {
        let item_id = Uuid::parse_str(&args[0]).unwrap();
        if let Some(entity) = inventory.remove_item(item_id) {
            // remove the item from the game
            cmds.entity(entity).despawn();

            // todo: move this into the persistance plugin
            let db = db.0.clone();

            let _guard = tasks.handle.enter();
            tasks.tasks.spawn(async move {
                let collection = InventoryEntry::collection(db);
                if let Err(e) = collection.delete_one(doc!("id": {"$eq": item_id}), None).await {
                    error!("Item remove failed: {:?}", e);
                }
            });
        }
    }
}

pub fn do_vendor_execute(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut players: Query<(&mut ParamBox, &mut PlayerInventory), With<PlayerComponent>>,
    mut cmds: Commands,
) {
    if 
        let Behavior::Binary(_, args) = behavior &&
        let NativeParam::Struct(attrib) = args &&
        let Ok((mut params, mut inventory)) = players.get_mut(instigator) &&
        let Some(player_params) = params.get_impl_mut::<dyn PlayerParams>()
    {
        debug!("Attrib 28: {:?}", attrib[28]);

        if let Some(required_voucher) = attrib[28].to_string()
            .ok()
            .and_then(|s| if !s.is_empty() { Some(s) } else { None }) 
            .and_then(|s| Uuid::parse_str(s).ok())
        {
            debug!("Required voucher: {}", required_voucher);

            if let Some(entity) = inventory.remove_item_of_kind(required_voucher) {
                cmds.entity(entity).despawn();
            } else {
                warn!("Player tried to metamorph without required voucher!");
                return;
            }
        }

        player_params.set_customization_gender(attrib[0].to_f32().unwrap());
        player_params.set_customization_height(attrib[1].to_f32().unwrap());
        player_params.set_customization_fat(attrib[2].to_f32().unwrap());
        player_params.set_customization_skinny(attrib[3].to_f32().unwrap());
        player_params.set_customization_muscular(attrib[4].to_f32().unwrap());
        player_params.set_customization_bust_size(attrib[5].to_f32().unwrap());
        player_params.set_race(attrib[6].to_i32().unwrap());
        player_params.set_customization_brow_angle(attrib[7].to_f32().unwrap());
        player_params.set_customization_eye_brow_pos(attrib[8].to_f32().unwrap());
        player_params.set_customization_eye_pos_spacing(attrib[9].to_f32().unwrap());
        player_params.set_customization_eye_pos(attrib[10].to_f32().unwrap());
        player_params.set_customization_eye_size_length(attrib[11].to_f32().unwrap());
        player_params.set_customization_eye_size_width(attrib[12].to_f32().unwrap());
        player_params.set_customization_eyes_pretty(attrib[13].to_f32().unwrap());
        player_params.set_customization_mouth_pos(attrib[14].to_f32().unwrap());
        player_params.set_customization_mouth_width(attrib[15].to_f32().unwrap());
        player_params.set_customization_mouth_lower_lip_thic(attrib[16].to_f32().unwrap());
        player_params.set_customization_mouth_upper_lip_thic(attrib[17].to_f32().unwrap());
        player_params.set_customization_mouth_expression(attrib[18].to_f32().unwrap());
        player_params.set_customization_nose_pos_length(attrib[19].to_f32().unwrap());
        player_params.set_customization_nose_pos_width(attrib[20].to_f32().unwrap());
        player_params.set_customization_nose_portude(attrib[21].to_f32().unwrap());
        player_params.set_customization_ear_size(attrib[22].to_f32().unwrap());
        player_params.set_customization_ear_elf(attrib[23].to_f32().unwrap());
        player_params.set_customization_cheek_bone(attrib[24].to_f32().unwrap());
        player_params.set_customization_cheek(attrib[25].to_f32().unwrap());
        player_params.set_customization_chin_portude(attrib[26].to_f32().unwrap());
        player_params.set_customization_jaw_chubby(attrib[27].to_f32().unwrap());
        debug!("Attrib 29: {:#?}", attrib[29]);

        let mut visible_item_builder = LoadoutBuilder::new();

        debug!("Visible items: {:?}", player_params.visible_item_info());

        // add currently equiped items
        if let Some(visible_items) = player_params.visible_item_info() {
            visible_items.iter().for_each(|id| { visible_item_builder.add(*id); });
        }

        // add newly equipped items
        attrib[30..]
            .iter()
            .filter_map(|a| a.to_uuid().ok())
            .filter_map(|u| get_cached_item(&u))
            .map(|i| i.id as i32)
            .for_each(|id| { visible_item_builder.add(id); });

        // apply items       
        player_params.set_visible_item_info(
            visible_item_builder.build()
                .into_iter()
                .map(|item| item.id as i32)
                .collect()
        );

        debug!("Visible items: {:?}", player_params.visible_item_info());

        // build default items based on the current equipped slots
        if let Some(visible_items) = player_params.visible_item_info() {
            player_params.set_default_items_content_guid(
                visible_items.iter()
                .filter_map(|id| get_cached_item_by_id(*id))
                .filter_map(|item| item.data.as_ref().map(|data| (item.id as i32, data)))
                .filter_map(|(id, data)| data.get_impl::<dyn ItemBaseParams>().map(|params| (id, params)))
                .filter_map(|(id, params)| params.slot_mapping().map(|slot| (id, slot)))
                .filter_map(|(id, slot)| slot.parse::<Slot>().ok().map(|slot| (id, slot)))
                .filter(|(_, slot)| slot.is_base_appearance())
                .map(|(id, _)| id)
                .collect()
                /* lol */
            );
        }

    }
}

pub fn request_equip(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut players: Query<(&mut ParamBox, &PlayerInventory), With<PlayerComponent>>
) {
    if 
        let Behavior::String(_, args) = behavior &&
        let Ok((mut params, inventory)) = players.get_mut(instigator)
    {
        if let Some(equip_item) = args.first()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| inventory.lookup_item_id(id))
            .map(|(_, id, _)| id)
        {
            let player_params = params.get_impl_mut::<dyn PlayerParams>().unwrap();

            let mut loadout = LoadoutBuilder::new();
            if let Some(visible_items) = player_params.visible_item_info() {
                for item in visible_items {
                    loadout.add(*item);
                }
            }

            loadout.add_by_uuid(equip_item);

            let loadout = loadout.build();

            player_params.set_visible_item_info(
                loadout
                .iter()
                .map(|item| item.id as i32)
                .collect()
            );

            
        }
    }
}
