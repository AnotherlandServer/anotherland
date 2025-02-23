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

use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use bevy::{app::{Last, Plugin, PostUpdate, Update}, ecs::{component::Component, query::Without, system::{Resource, SystemId}}, prelude::{Added, App, BuildChildren, Changed, Commands, DetectChangesMut, Entity, In, IntoSystemConfigs, Or, Parent, Query, Res, With}, time::common_conditions::on_timer, utils::hashbrown::{HashMap, HashSet}};
use bitstream_io::{ByteWriter, LittleEndian};
use futures::future::join_all;
use log::{debug, error, warn};
use obj_params::{tags::{ItemBaseTag, PlayerTag}, Class, GameObjectData, GenericParamSet, ItemEdna, ParamWriter, Player};
use protocol::{oaPktItemStorage, CPktItemNotify, CPktItemUpdate, ItemStorageParams, OaPktItemStorageUpdateType};
use realm_api::{Item, ItemRef, StorageOwner};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toolkit::{types::Uuid, NativeParam};

use crate::{error::WorldResult, instance::ZoneInstance, object_cache::CacheEntry, OBJECT_CACHE};

use super::{BehaviorExt, CommandExtPriv, ConnectionState, ContentInfo, CurrentState, FutureCommands, MessageType, PlayerController, StringBehavior};

#[derive(Resource)]
#[allow(clippy::type_complexity)]
struct InventorySystems {
    insert_item_storage: SystemId<In<WorldResult<(Entity, Inventory, Vec<(realm_api::Item, Arc<CacheEntry>)>)>>>,
    apply_storage_result: SystemId<In<(Entity, StorageResult)>>,
    apply_equipment_result: SystemId<In<(Entity, EquipmentResult)>>,
}

#[derive(Default)]
struct EquipmentResult {
    error: Option<(String, Option<NativeParam>)>,
    character_update: Option<Box<dyn GenericParamSet>>,
    storage_results: Vec<StorageResult>,
}

impl EquipmentResult {
    pub async fn from_result(result: realm_api::EquipmentResult) -> WorldResult<Self> {
        let storage_results = join_all(result.storage_results.into_iter()
            .map(StorageResult::from_result)
            .collect::<Vec<_>>()
        ).await.into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            error: result.error,
            character_update: result.character_update,
            storage_results,
        })
    }
}

#[derive(Default)]
struct StorageResult {
    storage_id: Uuid,
    bling: Option<i32>,
    game_cash: Option<i32>,
    changed_items: Option<Vec<(Item, Arc<CacheEntry>)>>,
    removed_items: Option<Vec<Uuid>>,
    error: Option<(String, Option<NativeParam>)>,
}

impl StorageResult {
    pub async fn from_result(result: realm_api::StorageResult) -> WorldResult<Self> {
        let changed_items = if let Some(changed_items) = result.changed_items {
            Some(
                join_all(changed_items.into_iter()
                    .map(|item| async {
                        if let Some(base_item) = OBJECT_CACHE.wait().get_object_by_guid(item.template_id).await? {
                            // Cache abilities for later use
                            if 
                                let Ok(abilities) = base_item.data.get::<_, Value>(ItemEdna::Abilities) &&
                                let Ok(abilities) = serde_json::from_value::<ItemEdnaAbilities>(abilities.to_owned())
                            {
                                for ability in abilities.0 {
                                    let _ = OBJECT_CACHE.wait().get_object_by_name(&ability.ability_name).await?;
                                }
                            }

                            Ok((item, base_item))
                        } else {
                            Err(anyhow!("Failed to load item template {}", item.template_id))
                        }
                    })
                    .collect::<Vec<_>>()
                ).await.into_iter()
                .collect::<Result<Vec<_>, _>>()?
            )
        } else {
            None
        };

        Ok(Self {
            storage_id: result.storage_id,
            bling: result.bling,
            game_cash: result.game_cash,
            changed_items,
            removed_items: result.removed_items,
            error: result.error,
        })
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(transparent, default)]
pub struct ItemEdnaAbilities(Vec<ItemEdnaAbility>);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemEdnaAbility {
    ability_name: String,
    auto_switch_group: i32,
    ability_info: String,
    target_ability_info: String,
    display_name: String,
}

#[derive(Resource, Default)]
struct StorageRegistry(HashMap<Uuid, Entity>);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        let inventory_systems = InventorySystems {
            insert_item_storage: app.register_system(insert_item_storage),
            apply_storage_result: app.register_system(apply_storage_result),
            apply_equipment_result: app.register_system(apply_equipment_result),
        };        

        app.insert_resource(inventory_systems);
        app.init_resource::<StorageRegistry>();

        app.add_systems(PostUpdate, load_player_inventory);
        app.add_systems(Update, (
            (
                init_client_inventory,
                send_initial_items.run_if(on_timer(Duration::from_secs(1))),
            ).chain(),
        ));
        app.add_systems(Last, send_item_updates);

        app.register_command("add_item", command_add_item);
        app.register_command("apply_item_template", command_apply_class_preset);

        app.register_string_behavior(Class::Player, "inventoryitempos", behavior_inventory_item_pos);
        app.register_string_behavior(Class::Player, "requestdiscarditem", behavior_inventory_discard_item);
        app.register_string_behavior(Class::Player, "requestequip", behavior_inventory_request_equip);
        app.register_string_behavior(Class::Player, "requestunequip", behavior_inventory_request_unequip);        

        app.world_mut().register_component_hooks::<Inventory>()
            .on_add(|mut world, entity, _| {  
                let storage_id = world.get_entity(entity).unwrap().get::<Inventory>().unwrap().id;
                let mut registry = world.get_resource_mut::<StorageRegistry>().unwrap();

                registry.0.insert(storage_id, entity);
            })
            .on_remove(|mut world, entity, _| {
                let storage_id = world.get_entity(entity).unwrap().get::<Inventory>().unwrap().id;
                let mut registry = world.get_resource_mut::<StorageRegistry>().unwrap();

                registry.0.remove(&storage_id);
            });
    }
}

#[allow(dead_code)]
struct CharacterPreset {
    combat_style: Option<i32>,
    level: Option<i32>,
    level_up_skills: Option<bool>,
    weapons: Vec<Arc<CacheEntry>>,
    armors: Vec<Arc<CacheEntry>>,
    qboost: Vec<Arc<CacheEntry>>,
} 

#[derive(Component)]
pub struct Inventory {
    id: Uuid,
    name: String,

    items: HashMap<Uuid, Entity>,

    bling: Option<i32>,
    game_cash: Option<i32>,
    max_slots: i32,

    observing_players: HashSet<Entity>,
}

impl Inventory {
    fn new(id: Uuid, name: String, bling: Option<i32>, game_cash: Option<i32>, max_slots: i32) -> Self {
        Self {
            id,
            name,
            items: HashMap::new(),

            bling,
            game_cash,
            max_slots,

            observing_players: HashSet::new(),
        }
    }
}

fn load_player_inventory(
    query: Query<(Entity, &PlayerController), Added<PlayerTag>>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    for (ent, controller) in query.iter() {
        let realm_api = instance.realm_api.clone();
        let character_id = controller.character_id();

        commands.run_system_async(async move {
            let storage = realm_api.get_or_create_item_storage(StorageOwner::Character(character_id), "inventory").await?;
            let mut items = vec![];

            // Load cached item templates
            for item in storage.items {
                if let Some(base_item) = OBJECT_CACHE.wait().get_object_by_guid(item.template_id).await? {
                    // Cache abilities for later use
                    if 
                        let Ok(abilities) = base_item.data.get::<_, Value>(ItemEdna::Abilities) &&
                        let Ok(abilities) = serde_json::from_value::<ItemEdnaAbilities>(abilities.to_owned())
                    {
                        for ability in abilities.0 {
                            let _ = OBJECT_CACHE.wait().get_object_by_name(&ability.ability_name).await?;
                        }
                    }

                    items.push((item, base_item));
                }
            }

            let mut inventory = Inventory::new(
                storage.id, 
                storage.name, 
                storage.bling, 
                storage.game_cash,
                storage.capacity,
            );

            inventory.observing_players.insert(ent);

            Ok((ent, inventory, items))
        }, systems.insert_item_storage);
    }
}

#[allow(clippy::type_complexity)]
fn insert_item_storage(
    In(result): In<WorldResult<(Entity, Inventory, Vec<(realm_api::Item, Arc<CacheEntry>)>)>>,
    ents: Query<Entity>,
    mut player: Query<&mut GameObjectData, With<PlayerTag>>,
    mut commands: Commands,
) {
    match result {
        Ok((ent, mut storage, items)) => {
            if let Ok(mut player) = player.get_mut(ent) {
                player.set(Player::Bling, storage.bling.unwrap_or(0));
                player.set(Player::GameCash, storage.game_cash.unwrap_or(0));
            }

            if let Ok(ent) = ents.get(ent) {
                for (item, template) in items {
                    let mut instance = item.instance;
                    instance.set_parent(Some(template.data.clone()));

                    let item_ent = commands.spawn((
                        ContentInfo {
                            placement_id: item.id,
                            template: template.clone(),
                        },
                        instance,
                    ))
                    .set_parent(ent)
                    .id();

                    storage.items.insert(item.id, item_ent);
                }

                commands.entity(ent)
                    .insert((
                        InitialInventoryTransfer(
                            storage.items.values().copied().collect()
                        ),
                        storage,
                    ));
            }
        },
        Err(e) => {
            error!("Failed to load player inventory: {}", e);
        }
    }
}

fn command_add_item(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    storage: Query<&Inventory>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if 
        let Some(NativeParam::String(item_name)) = args.next() &&
        let Ok(storage) = storage.get(ent)
    {
        let realm_api = instance.realm_api.clone();
        let storage_id = storage.id;

        commands.run_system_async(async move {
            debug!("Try inserting item {}", item_name);

            match realm_api.item_storage_access(&storage_id)
                .insert_item(ItemRef::Name(&item_name), Some(ent.to_string()))
                .await
            {
                Ok(res) => {
                    match StorageResult::from_result(res).await {
                        Ok(result) => (ent, result),
                        Err(e) => {
                            error!("Failed to insert item: {}", e);
                            (ent, StorageResult::default())
                        }
                    }
                },
                Err(e) => {
                    warn!("Failed to insert item: {}", e);
                    (ent, StorageResult::default())
                }
            }

        }, systems.apply_storage_result);
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct DefaultEquipment {
    #[serde(rename = "CombatStyle")]
    combat_style: Option<i32>,

    #[serde(rename = "Level")]
    level: Option<i32>,

    #[serde(rename = "levelUpSkills")]
    level_up_skills: Option<bool>,

    #[serde(rename = "Weapons")]
    weapons: Vec<String>,

    #[serde(rename = "QBoost")]
    qboost: Option<String>,

    #[serde(rename = "Armors")]
    armors: Vec<String>,

    #[serde(rename = "Abilities")]
    abilities: Vec<String>,
}

#[allow(clippy::type_complexity)]
fn command_apply_class_preset(
    In((_ent, _args)): In<(Entity, Vec<NativeParam>)>,
    mut _players: Query<&mut Inventory>,
    _instance: Res<ZoneInstance>,
    _systems: Res<InventorySystems>,
    mut _commands: Commands,
) {
    /*let mut args = args.into_iter();

    if let Some(NativeParam::String(template_name)) = args.next() {
        // Delete current equipment
        if let Ok(mut inventory) = players.get_mut(ent) {
            for item_ent in inventory.equipped_items.drain(..).collect::<Vec<_>>() {
                commands.entity(item_ent).despawn();
                inventory.id_lookup.retain(|_, v| *v != item_ent);
            }

            for item_ent in inventory.costume_items.drain(..).collect::<Vec<_>>() {
                commands.entity(item_ent).despawn();
                inventory.id_lookup.retain(|_, v| *v != item_ent);
            }
        }

        let object_cache = instance.object_cache.clone();

        commands.run_system_async(
            IoTaskPool::get()
            .spawn(async move {
                if 
                    let Some (preset) = object_cache.get_object_by_name(&template_name).await
                        .ok()
                        .flatten() &&
                    let Some(default_equipment) = preset.data.get::<_, Value>(ClassItem::DefaultEquipment).ok()
                        .and_then(|v| serde_json::from_value::<DefaultEquipment>(v.clone()).ok())
                {
                    let weapons = 
                        join_all(
                            default_equipment.weapons.iter()
                            .map(|name| object_cache.get_object_by_name(name))
                            .collect::<Vec<_>>()
                        ).await
                        .into_iter()
                        .flatten()
                        .flatten()
                        .collect::<Vec<_>>();

                    let armors = 
                        join_all(
                            default_equipment.armors.iter()
                            .map(|name| object_cache.get_object_by_name(name))
                            .collect::<Vec<_>>()
                        ).await
                        .into_iter()
                        .flatten()
                        .flatten()
                        .collect::<Vec<_>>();

                    (ent, Some(CharacterPreset {
                        combat_style: default_equipment.combat_style,
                        level: default_equipment.level,
                        level_up_skills: default_equipment.level_up_skills,
                        weapons,
                        armors,
                        qboost: vec![],
                    }))
                } else { 
                    (ent, None)
                }
            }), 
            systems.apply_character_preset
        );
    }*/
}

#[allow(dead_code)]
fn apply_character_preset(
    In((_ent, preset)): In<(Entity, Option<CharacterPreset>)>,
    _instance: Res<ZoneInstance>,
    mut _commands: Commands,
) {
    if let Some(_preset) = preset {
        /*let default_equipment = preset.data.get::<_, Value>(ClassItem::DefaultEquipment).unwrap();
        let mut items = vec![];

        if let Some(Value::Array(weapons)) = default_equipment.get("Weapons") {
            for weapon in weapons {
                items.push(weapon.as_str().unwrap().to_string());
            }
        }
        
        if let Some(Value::Array(armors)) = default_equipment.get("Armors") {
            for armor in armors {
                items.push(armor.as_str().unwrap().to_string());
            }
        }

        for item in items {
            let object_cache = instance.object_cache.clone();

            debug!("Loading item {}", item);

            commands.spawn(AsyncItemInsert {
                player: task.player,
                destination: ItemInsertDestination::Equipment,
                task: IoTaskPool::get()
                    .spawn(async move {
                        object_cache.get_object_by_name(&item).await
                            .ok()
                            .flatten()
                    })
            });
        }*/
    }
}

fn behavior_inventory_item_pos(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    inventories: Query<&Inventory>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    let mut args = behavior.args.into_iter();

    if 
        let Some(item_id) = args.next().and_then(|arg| arg.parse::<Uuid>().ok()) &&
        let Some(slot) = args.next().and_then(|arg| arg.parse::<i32>().ok()) &&
        let Ok(storage) = inventories.get(ent)
    {
        let realm_api = instance.realm_api.clone();
        let storage_id = storage.id;

        commands.run_system_async(async move {
            if let Ok(res) = realm_api.item_storage_access(&storage_id)
                .move_item(item_id, slot, Some(ent.to_string()))
                .await
            {
                match StorageResult::from_result(res).await {
                    Ok(result) => (ent, result),
                    Err(e) => {
                        error!("Failed to move item: {}", e);
                        (ent, StorageResult::default())
                    }
                }
            } else {
                (ent, StorageResult::default())
            }

        }, systems.apply_storage_result);
    }
}

fn behavior_inventory_discard_item(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    inventories: Query<&Inventory>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    let mut args = behavior.args.into_iter();

    if 
        let Some(item_id) = args.next().and_then(|arg| arg.parse::<Uuid>().ok()) &&
        let Ok(storage) = inventories.get(ent)
    {
        let realm_api = instance.realm_api.clone();
        let storage_id = storage.id;

        commands.run_system_async(async move {
            if let Ok(res) = realm_api.item_storage_access(&storage_id)
                .destroy_item(item_id, Some(ent.to_string()))
                .await
            {
                match StorageResult::from_result(res).await {
                    Ok(result) => (ent, result),
                    Err(e) => {
                        error!("Failed to discard item: {}", e);
                        (ent, StorageResult::default())
                    }
                }
            } else {
                (ent, StorageResult::default())
            }

        }, systems.apply_storage_result);
    }
}

fn behavior_inventory_request_equip(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    inventories: Query<&Inventory>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    let mut args = behavior.args.into_iter();

    if 
        let Some(item_id) = args.next().and_then(|arg| arg.parse::<Uuid>().ok()) &&
        let Some(slot) = args.next().and_then(|arg| arg.parse::<i32>().ok()) &&
        let Ok(storage) = inventories.get(ent)
    {
        let realm_api = instance.realm_api.clone();
        let storage_id = storage.id;

        commands.run_system_async(async move {
            if let Ok(res) = realm_api.item_storage_access(&storage_id)
                .equip_item(item_id, if slot != -1 { Some(slot) } else { None }, Some(ent.to_string()))
                .await
            {
                match EquipmentResult::from_result(res).await {
                    Ok(result) => (ent, result),
                    Err(e) => {
                        error!("Failed to equip item: {}", e);
                        (ent, EquipmentResult::default())
                    }
                }
            } else {
                (ent, EquipmentResult::default())
            }

        }, systems.apply_equipment_result);
    }
}

fn behavior_inventory_request_unequip(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    inventories: Query<&Inventory>,
    instance: Res<ZoneInstance>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    let mut args = behavior.args.into_iter();

    if 
        let Some(item_id) = args.next().and_then(|arg| arg.parse::<Uuid>().ok()) &&
        let Ok(storage) = inventories.get(ent)
    {
        let realm_api = instance.realm_api.clone();
        let storage_id = storage.id;

        commands.run_system_async(async move {
            if let Ok(res) = realm_api.item_storage_access(&storage_id)
                .unequip_item(item_id, Some(ent.to_string()))
                .await
            {
                match EquipmentResult::from_result(res).await {
                    Ok(result) => (ent, result),
                    Err(e) => {
                        error!("Failed to unequip item: {}", e);
                        (ent, EquipmentResult::default())
                    }
                }
            } else {
                (ent, EquipmentResult::default())
            }

        }, systems.apply_equipment_result);
    }
}

fn apply_equipment_result(
    In((instigator, result)): In<(Entity, EquipmentResult)>,
    mut players: Query<(&mut GameObjectData, &PlayerController), With<PlayerTag>>,
    systems: Res<InventorySystems>,
    mut commands: Commands,
) {
    if let Ok((mut player, controller)) = players.get_mut(instigator) {
        if let Some(err) = result.error {
            controller.send_message(MessageType::PopUp, err.0);
        }

        if let Some(mut character_update) = result.character_update {
            player.apply(character_update.as_mut());
        }

        for storage_result in result.storage_results {
            commands.run_system_with_input(systems.apply_storage_result, (instigator, storage_result));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_storage_result(
    In((instigator, result)): In<(Entity, StorageResult)>,
    mut storages: Query<&mut Inventory>,
    controllers: Query<&PlayerController>,
    mut players: Query<&mut GameObjectData, (With<PlayerTag>, Without<ItemBaseTag>)>,
    observers: Query<&PlayerController>,
    mut items: Query<&mut GameObjectData, (With<ItemBaseTag>, Without<PlayerTag>)>,
    registry: Res<StorageRegistry>,
    mut commands: Commands,
) {
    let (storage_ent, mut storage) = if 
        let Some(storage_ent) = registry.0.get(&result.storage_id) &&
        let Ok(storage) = storages.get_mut(*storage_ent)
    {
        (*storage_ent, storage)
    } else {
        return;
    };

    if let Some(err) = result.error {
        if let Ok(controller) = controllers.get(instigator) {
            controller.send_message(MessageType::PopUp, err.0);
        }
    } else {
        if let Some(bling) = result.bling {
            storage.bling = Some(bling);

            if let Ok(mut player) = players.get_mut(storage_ent) {
                player.set(Player::Bling, bling);
            }
        }

        if let Some(game_cash) = result.game_cash {
            storage.game_cash = Some(game_cash);

            if let Ok(mut player) = players.get_mut(storage_ent) {
                player.set(Player::GameCash, game_cash);
            }
        }

        if let Some(changed_items) = result.changed_items {
            for (item, template) in changed_items {
                let mut instance = item.instance;
                instance.set_parent(Some(template.data.clone()));

                let mut data = Vec::new();
                {
                    let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                    instance.write_to_privileged_client(&mut writer).unwrap();
                }

                for observer in &storage.observing_players {
                    if let Ok(controller) = observers.get(*observer) 
                    {
                        controller.send_packet(CPktItemUpdate {
                            avatar_id: controller.avatar_id(),
                            id: item.id,
                            use_template: 1,
                            template_id: Some(item.template_id),
                            class_id: instance.class().id() as u32,
                            params: data.clone(),
                            ..Default::default()
                        });
                    }
                }  

                if 
                    let Some(item_ent) = storage.items.get(&item.id) &&
                    let Ok(mut item_data) = items.get_mut(*item_ent)
                {
                    item_data.bypass_change_detection().apply(instance.into_set().as_mut());
                } else {
                    let item_ent = commands.spawn((
                        ContentInfo {
                            placement_id: item.id,
                            template: template.clone(),
                        },
                        instance,
                    ))
                    .set_parent(storage_ent)
                    .id();

                    storage.items.insert(item.id, item_ent);
                }
            }
        }

        if let Some(removed_items) = result.removed_items {
            for item in removed_items {
                if let Some(item_ent) = storage.items.remove(&item) {
                    for observer in &storage.observing_players {
                        if let Ok(controller) = observers.get(*observer) 
                        {
                            controller.send_packet(CPktItemNotify {
                                avatar_id: controller.avatar_id(),
                                id: item,
                                ..Default::default()
                            });
                        }
                    }                    
                    
                    commands.entity(item_ent).despawn();
                }
            }
        }
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
                storage_name: inventory.name.clone(),
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
        let count = queue.0.len().min(10);
        for item_ent in queue.0.drain(..count) {
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

#[allow(clippy::type_complexity)]
fn send_item_updates(
    item_updates: Query<(&GameObjectData, &ContentInfo, &Parent), Or<((Changed<GameObjectData>, With<ItemBaseTag>), Added<ItemBaseTag>)>>,
    players: Query<&PlayerController, Without<InitialInventoryTransfer>>,
) {
    for (item, content, player) in item_updates.iter() {
        if let Ok(ctrl) = players.get(player.get()) {
            let mut params = Vec::new();
            let mut writer = ByteWriter::endian(&mut params, LittleEndian);
            item.write_to_client(&mut writer).unwrap();
            
            ctrl.send_packet(CPktItemUpdate {
                avatar_id: ctrl.avatar_id(),
                id: content.placement_id,
                use_template: 1,
                template_id: Some(content.template.id),
                class_id: item.class().id() as u32,
                params,
                ..Default::default()
            });
        }
    }
}
