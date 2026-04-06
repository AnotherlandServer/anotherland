// Copyright (C) 2026 AnotherlandServer
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

use bevy::ecs::{entity::Entity, message::MessageReader, system::{Commands, In, Query}};
use log::debug;
use obj_params::{GameObjectData, LootScatterContainer, Player};
use realm_api::{ItemRef, RealmApi};
use scripting::{EntityScriptCommandsExt, LuaEntity};
use toolkit::types::{AvatarId, UUID_NIL, Uuid};

use crate::plugins::{AsyncOperationEntityCommandsExt, Avatar, Interaction, InteractionEvent, Inventory, RemoveObject, StorageResult, StringBehavior, apply_storage_result, player_error_handler_system};

pub(super) fn handle_interactions(
    mut events: MessageReader<InteractionEvent>,
    mut commands: Commands,
) {
    for &InteractionEvent { source, target, interaction } in events.read() {
        if let Interaction::CastComplete = interaction {
            commands
                .entity(target)
                .call_named_lua_method("HandleInteraction", LuaEntity(source));
        }
    }
}

pub(super) fn behavior_loot_scatter_container_interact(
    In((player_ent, target_ent, _behavior)): In<(Entity, Entity, StringBehavior)>,
    players: Query<(&Avatar, &GameObjectData, &Inventory)>,
    container: Query<&GameObjectData>,
    mut commands: Commands,
) {
    let Ok((player_avatar, player_data, inventory)) = players.get(player_ent) else {
        return;
    };

    let Ok(container_data) = container.get(target_ent) else {
        return;
    };

    let allow_avatar = container_data.get::<_, AvatarId>(LootScatterContainer::AllowAvatar).cloned().unwrap_or_default();
    let allow_party = container_data.get::<_, Uuid>(LootScatterContainer::AllowParty).cloned().unwrap_or_default();

    let player_party = player_data.get::<_, Uuid>(Player::PartyGuid).cloned().unwrap_or_default();

    // Check if the player is allowed to loot this container
    if 
        (allow_avatar.is_none() || allow_avatar == player_avatar.id) &&
        (allow_party == UUID_NIL || allow_party == player_party)
    {
        let storage_id = inventory.id;
        let item_name = container_data.get::<_, String>(LootScatterContainer::ItemContentName).cloned().unwrap_or_default();
        let _item_count = container_data.get::<_, i32>(LootScatterContainer::ItemCount).cloned().unwrap_or_default();

        commands
            .entity(player_ent)
            .perform_async_operation(async move {
                debug!("Loot item {item_name}");

                StorageResult::from_result(
                    RealmApi::get()
                        .item_storage_access(&storage_id)
                        .insert_item(ItemRef::Name(&item_name), Some(player_ent.to_string()))
                        .await?
                ).await
            })
            .on_finish_run_system(apply_storage_result)
            .on_error_run_system(player_error_handler_system);
    }

    commands
        .entity(target_ent)
        .trigger(RemoveObject);
}