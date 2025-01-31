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

use bevy::{app::Plugin, prelude::{App, Entity, In, Query, Res}};
use futures_util::TryStreamExt;
use log::debug;
use protocol::{oaPktCashItemVendorSyncAcknowledge, oaPktCashItemVendorSyncRequest, oaPktSKUBundleSyncAcknowledge, oaPktSKUBundleSyncRequest, CPkt, CashItemSKUBundleEntry, CashItemSKUItemEntry, CashItemVendorEntry};

use crate::instance::ZoneInstance;

use super::{NetworkExtPriv, PlayerController};

pub struct CashShopPlugin;

impl Plugin for CashShopPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_sku_bundle_sync_request);
        app.register_message_handler(handle_cash_item_vendor_sync_request);
    }
}

fn handle_sku_bundle_sync_request(
    In((ent, pkt)): In<(Entity, oaPktSKUBundleSyncRequest)>,
    controller: Query<&PlayerController>,
    instance: Res<ZoneInstance>,
) {
    if let Ok(controller) = controller.get(ent) {
        // We handle sync requests in a separate task,
        // to handle the big query more ergonomically.

        let controller = controller.clone();
        let realm_api = instance.realm_api.clone();

        instance.spawn_task(async move {
            let mut response = oaPktSKUBundleSyncAcknowledge::default();

            for item_request in pkt.sku_items {
                if 
                    let Ok(id) = item_request.id.parse() &&
                    let Ok(Some(item)) = realm_api.get_cash_shop_item(id).await
                {
                    response.sku_items.push(CashItemSKUItemEntry { 
                        sku_id: item.id.to_string(),
                        reference_item_guid: item.reference_item_guid.to_string(),
                        reference_item_name: item.reference_item_name,
                        rental_duration: item.rental_duration as u32,
                        sku_code: item.sku_code,
                        cash_price: item.cash_price as u32, 
                        is_in_stock: item.is_in_stock, 
                        is_hot: item.is_hot, 
                        is_new: item.is_new, 
                        version: item.version as u32, 
                        is_visible: item.is_visible, 
                        is_tradable: item.is_tradable, 
                        is_featured: item.is_featured, 
                        quantity: item.quantity as u32, 
                        discount: item.discount as u32, 
                        display_name: item.display_name, 
                        description: item.description, 
                        date_start: item.date_start
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or("invalid".to_owned()), 
                        date_end: item.date_end
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or("invalid".to_owned())
                    });
                } else {
                    response.deleted_item_ids.push(item_request.id);
                }
            }

            for bundle_request in pkt.bundle_items {
                if 
                    let Ok(id) = bundle_request.id.parse() &&
                    let Ok(Some(bundle)) = realm_api.get_cash_shop_item_bundle(id).await
                {
                    response.bundle_items.push(CashItemSKUBundleEntry { 
                        cash_price: bundle.cash_price as u32, 
                        is_in_stock: bundle.is_in_stock, 
                        is_hot: bundle.is_hot, 
                        is_new: bundle.is_new, 
                        version: bundle.version as u32, 
                        is_visible: bundle.is_visible, 
                        is_tradable: bundle.is_tradable, 
                        is_featured: bundle.is_featured, 
                        quantity: bundle.quantity as u32, 
                        discount: bundle.discount as u32, 
                        bundle_id: bundle.id.to_string(), 
                        display_name: bundle.display_name, 
                        description: bundle.description, 
                        icon: bundle.icon, 
                        item_list_and_count: bundle.item_list_and_count, 
                        date_start: bundle.date_start
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or("invalid".to_owned()), 
                        date_end: bundle.date_end
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or("invalid".to_owned())
                    });
                } else {
                    response.deleted_bundle_ids.push(bundle_request.id);
                }
            }

            response.sku_item_count = response.sku_items.len() as u32;
            response.bundle_item_count = response.bundle_items.len() as u32;
            response.deleted_items_count = response.deleted_item_ids.len() as u32;
            response.deleted_bundles_count = response.deleted_bundle_ids.len() as u32;

            debug!("Cash item sync complete. Items: {} Deleted: {} Bundles: {} Deleted: {}", response.sku_item_count, response.deleted_items_count, response.bundle_item_count, response.deleted_bundles_count);

            controller.send_packet(response);
        });
    }
}

fn handle_cash_item_vendor_sync_request(
    In((ent, pkt)): In<(Entity, oaPktCashItemVendorSyncRequest)>,
    controller: Query<&PlayerController>,
    instance: Res<ZoneInstance>,
) {
    if let Ok(controller) = controller.get(ent) {
        // We handle sync requests in a separate task,
        // to handle the big query more ergonomically.

        let controller = controller.clone();
        let realm_api = instance.realm_api.clone();

        instance.spawn_task(async move {
            let mut response = oaPktCashItemVendorSyncAcknowledge::default();

            for vendor_request in pkt.items {
                if 
                    let Ok(id) = vendor_request.id.parse() &&
                    let Ok(Some(vendor)) = realm_api.get_cash_shop_vendor(id).await
                {
                    response.items.push(CashItemVendorEntry {
                        vendor_id: vendor.id.to_string(),
                        vendor_name: vendor.vendor_name,
                        sku_list: vendor.sku_list.into_iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                        bundle_list: vendor.bundle_list.into_iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                        version: vendor.version as u32,
                    });
                } else {
                    response.deleted_ids.push(vendor_request.id);
                }
            }

            response.item_count = response.items.len() as u32;
            response.deleted_count = response.deleted_ids.len() as u32;

            debug!("Vendor sync complete. Records: {} Deleted: {}", response.item_count, response.deleted_count);

            controller.send_packet(response);
        });
    }
}
