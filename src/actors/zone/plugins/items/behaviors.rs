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

use atlas::{ItemBaseComponent, ItemBaseParams, ItemEdnaParams, NativeParam, ParamBox, PlayerComponent, PlayerParams, Slot, Uuid, UUID_NIL};
use bevy_ecs::{event::EventWriter, query::{With, Without}, system::{Commands, In, Query, Res}};
use bson::doc;
use log::{debug, error, warn};

use crate::{actors::{zone::{plugins::{Behavior, BehaviorArguments, PlayerLoadout, RemovalPending}, resources::Tasks}, RealmDatabase}, db::{get_cached_item, get_cached_item_by_id, DatabaseRecord, InventoryEntry}};

use super::{EquipItemTransaction, Equipped, Item, ItemReference, PlayerDisguise, PlayerInventory, RemoveItemTransaction, UnequipItemTransaction};

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
    mut ev: EventWriter<RemoveItemTransaction>,
) {
    if 
        let Behavior::String(_, args) = behavior
    {
        let item_id = Uuid::parse_str(&args[0]).unwrap();
        ev.send(RemoveItemTransaction { entity: instigator, id: item_id });
    }
}

pub fn do_vendor_execute(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut players: Query<(&mut ParamBox, &mut PlayerInventory, &mut PlayerLoadout, &PlayerDisguise), With<PlayerComponent>>,
    mut cmds: Commands,
) {
    if 
        let Behavior::Binary(_, args) = behavior &&
        let NativeParam::Struct(attrib) = args &&
        let Ok((mut params, mut inventory, mut loadout, disguise)) = players.get_mut(instigator) &&
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

        // add newly equipped items to loadout
        attrib[30..]
            .iter()
            .filter_map(|a| a.to_uuid().ok())
            .filter_map(|u| get_cached_item(&u))
            .map(|i| i.id as i32)
            .for_each(|id| { loadout.add(ItemReference::VisualOnly(id)); });

        // update visible items      
        player_params.set_visible_item_info(
            [loadout.compile_visual_items(), disguise.compile_visual_items()].concat()
        );

        debug!("Visible items: {:?}", player_params.visible_item_info());

        // build default items based on the current equipped slots
        player_params.set_default_items_content_guid(
            player_params.visible_item_info().iter()
            .filter_map(|id| get_cached_item_by_id(*id))
            .filter_map(|item| item.data.as_ref().map(|data| (item.id as i32, data)))
            .filter_map(|(id, data)| data.get_impl::<dyn ItemBaseParams>().map(|params| (id, params)))
            .map(|(id, params)| (id, params.slot_mapping()))
            .filter_map(|(id, slot)| slot.parse::<Slot>().ok().map(|slot| (id, slot)))
            .filter(|(_, slot)| slot.is_base_appearance())
            .map(|(id, _)| id)
            .collect()
            /* lol */
        );
    }
}

pub fn request_equip(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    players: Query<&PlayerInventory>,
    mut ev: EventWriter<EquipItemTransaction>,
) {
    if
        let Behavior::String(_, args) = behavior &&
        let Ok(inventory) = players.get(instigator) &&
        let Some(inventory_item_id) = args.first()
            .and_then(|s| Uuid::parse_str(s).ok()) &&
        let Some((_, _, item)) = inventory.lookup_item_id(inventory_item_id)
    {
        ev.send(EquipItemTransaction { player: instigator, item });
    }
}

pub fn request_unequip(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut players: Query<&PlayerInventory>,
    mut ev: EventWriter<UnequipItemTransaction>,
) {
    if 
        let Behavior::String(_, args) = behavior &&
        let Ok(inventory) = players.get_mut(instigator) &&
        let Some(inventory_item_id) = args.first()
            .and_then(|s| Uuid::parse_str(s).ok()) &&
        let Some((_, _, item)) = inventory.lookup_item_id(inventory_item_id)
    {
        ev.send(UnequipItemTransaction { player: instigator, item });
    }
}

pub fn request_select_weapon(
    In((instigator, _, behavior)): In<BehaviorArguments>,
    mut players: Query<(&PlayerInventory, &mut ParamBox)>,
) {
    if 
        let Behavior::String(_, args) = behavior &&
        let Ok((inventory, mut params)) = players.get_mut(instigator) &&
        let Some(inventory_item_id) = args.first()
            .and_then(|s| Uuid::parse_str(s).ok()) &&
        let Some(params) = params.get_impl_mut::<dyn PlayerParams>() &&
        inventory.lookup_item_id(inventory_item_id).is_some()
    {
        params.set_weapon((inventory_item_id, UUID_NIL));
    }
}
