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

use atlas::{ItemBaseParams, ParamBox, PlayerClass, PlayerComponent, PlayerParams, Uuid};
use bevy_ecs::{entity::Entity, event::{Event, EventReader}, query::With, system::{Commands, Query}};
use log::debug;

use crate::{actors::zone::plugins::PlayerController, db::get_cached_item};

use super::PlayerInventory;

#[derive(Event)]
pub struct ItemPurchaseRequest(pub Entity, pub Uuid, pub i32);

#[derive(Event)]
pub struct ItemSellRequest(pub Entity, pub Uuid, pub u32);

pub fn process_buy_request(
    mut ev_purchase: EventReader<ItemPurchaseRequest>,
    mut players: Query<(&mut ParamBox, &mut PlayerInventory, &PlayerController), With<PlayerComponent>>,
    mut cmds: Commands,
) {
    for &ItemPurchaseRequest(entity, id, mut count) in ev_purchase.read() {
        debug!("Processing purchase request: {:?}-{}-{}", entity, id, count);

        if let Some(item) = get_cached_item(&id) &&
            let Ok((mut player, mut inventory,  controller)) = players.get_mut(entity) {

            let item_base = item.data.as_ref().and_then(|i| i.get_impl::<dyn ItemBaseParams>()).unwrap();

            if let Ok(player_params) = player.get_mut::<PlayerClass>() {
                loop {
                    // deduct price from player cash
                    if item_base.buy_price_bling() > 0 {
                        let mut price = item_base.buy_price_bling();
                        price -= f32::floor(price as f32 * item_base.buy_discount()) as i32;

                        if player_params.bling() < price {
                            break controller.send_shopping_result("#Shop.Purchase_NotEnoughBits#");
                        } else {
                            player_params.set_bling(player_params.bling() - price);
                        }
                    } else if item_base.buy_price_game_cash() > 0 {
                        let mut price = item_base.buy_price_game_cash();
                        price -= f32::floor(price as f32 * item_base.buy_discount()) as i32;

                        if player_params.game_cash() < price {
                            break controller.send_shopping_result("#Shop.Purchase_NotEnoughCash#");
                        } else {
                            player_params.set_game_cash(player_params.game_cash() - price);
                        }
                    }

                    if inventory.insert_new(entity, &mut cmds, item.clone()).is_err() {
                        break controller.send_shopping_result("#Shop.NotEnoughInventorySlots#");
                    }

                    count = count.saturating_sub(1);
                    if count == 0 {
                        break controller.send_shopping_result("#Shop.successful#");
                    }
                }
            }
        }
    }
}