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

use bevy::{ecs::{message::MessageWriter, world::World}, math::{Quat, Vec3}, prelude::{Added, Changed, Commands, Entity, In, Or, Query, Res, With}};
use futures::TryStreamExt;
use log::{debug, error, trace, warn};
use mlua::Function;
use obj_params::{AttributeInfo, GameObjectData, GenericParamSet, NonClientBase, ParamFlag, ParamSet, Player, Portal, Value, tags::PlayerTag};
use protocol::{oaAbilityBarReferences, CPktAvatarUpdate};
use realm_api::{AbilitySlot, ObjectPlacement, RealmApi, RealmApiResult};
use scripting::{EntityScriptCommandsExt, ScriptObject};
use toolkit::{NativeParam, OtherlandQuatExt, types::Uuid};

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance, plugins::{AsyncOperationEntityCommandsExt, ConnectionState, ContentCache, ContentCacheRef, CurrentState, EquipmentResult, HealthUpdateEvent, InitialInventoryTransfer, Inventory, MessageType, Movement, PlayerController, ServerAction, WeakCache, apply_equipment_result, player_error_handler_system}, proto::TravelMode};

#[allow(clippy::type_complexity)]
pub fn spawn_player(
    mut query: Query<(&mut CurrentState, Option<&InitialInventoryTransfer>, &ScriptObject), Changed<CurrentState>>,
    instance: Res<ZoneInstance>,
    mut commands: Commands
) {
    for (state, inventory_transfer, obj) in query.iter_mut() {
        if 
            matches!(state.state, ConnectionState::InitialInterestsLoaded) &&
            inventory_transfer.is_none()
        {
            commands
                .entity(instance.world_controller)
                .call_named_lua_method("SpawnPlayer", obj.object().clone());
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn save_player_data(
    query: Query<(&PlayerController, &GameObjectData), (Or<(Added<GameObjectData>, Changed<GameObjectData>)>, With<PlayerTag>)>,
    instance: Res<ZoneInstance>,
) {
    for (controller, obj) in query.iter() {
        let id = *controller.state().character();
        let volatile_diff = obj.changes()
            .filter(|(attr, _)| !attr.has_flag(&ParamFlag::Persistent))
            .collect::<Box<dyn GenericParamSet>>();

        if 
            let Some(Value::Any(ability_bar)) = volatile_diff.get_param(Player::CurrentAbilityBarReferences.name()) &&
            let Ok((_, current_ability_bar)) = oaAbilityBarReferences::from_bytes(ability_bar)
        {
            let mut ability_bar = RealmApi::get().create_empty_ability_bar(id);
            ability_bar.single_slot = AbilitySlot {
                id: current_ability_bar.single_slot_bar.id,
                ability: current_ability_bar.single_slot_bar.skill.clone(),
            };

            ability_bar.slots = current_ability_bar.main_skill_bar.iter()
                .map(|e| AbilitySlot {
                    id: e.id,
                    ability: e.skill.clone(),
                })
                .collect();

            instance.spawn_task(async move {
                let _ = ability_bar.save().await;
            });
        }

        let persistent_diff =  obj.changes()
            .filter(|(attr, _)| attr.has_flag(&ParamFlag::Persistent))
            .collect::<Box<dyn GenericParamSet>>();

        if !persistent_diff.is_empty() {
            trace!("Saving character update for: {id} - {persistent_diff:#?}");

            // We probably should move this into it's own task and just 
            // send a (blocking) message here, se we can have
            // backpressure in case our updates don't go trough.
            // Also, errors are not really handled here.
            instance.spawn_task(async move {
                if let Err(e) = RealmApi::get().update_character_data_diff(&id, persistent_diff).await {
                    error!("Character update failed: {e:?}");
                }
            });
        }  
    }
}

pub fn handle_avatar_update(
    In((ent, pkt)): In<(Entity, CPktAvatarUpdate)>,
    mut query: Query<(&PlayerController, &mut GameObjectData)>,
) {
    if 
        let Ok((controller, mut obj)) = query.get_mut(ent) &&
        let Ok((_, params)) = ParamSet::<Player>::from_slice(&pkt.params) &&

        // Ignore updates for any avatars other than the player avatar.
        pkt.avatar_id.unwrap_or_default() == controller.avatar_id() 
    {
        let mut params = params.into_iter()
            .filter(|(a, _)| !a.has_flag(&ParamFlag::ExcludeFromClient))
            .filter(|(a, v)| obj.get_named::<Value>(a.name()).unwrap() != v)
            .collect::<Box<dyn GenericParamSet>>();

        if 
            let Some(Value::Any(ability_bar)) = params.get_param(Player::CurrentAbilityBarReferences.name()) &&
            let Ok(ability_bar) = oaAbilityBarReferences::from_bytes(ability_bar)
        {
            debug!("{ability_bar:#?}");
        }

        obj.apply(params.as_mut());
    }
}

pub fn cmd_instant_kill(
    In((ent, _)): In<(Entity, Vec<NativeParam>)>,
    mut event: MessageWriter<HealthUpdateEvent>
) {
    event.write(HealthUpdateEvent::kill(ent, None));
}

pub fn cmd_travel_to_portal(
    In((ent, params)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands
) {
    if 
        let Some(NativeParam::String(portal_guid)) = params.first() &&
        let Ok(portal_guid) = portal_guid.parse::<Uuid>() 
    {
        async fn load_object(id: Uuid) -> WorldResult<Option<ObjectPlacement>> {
            if 
                let Some(mut object) = RealmApi::get().get_object_placement(id).await? &&
                let Some(template) = ContentCache::get(&ContentCacheRef::Uuid(object.content_guid)).await
                    .map_err(WorldError::BevyError)?
            {
                object.data.set_parent(Some(template));
                Ok(Some(object))
            } else {
                Ok(None)
            }
        }

        commands
            .entity(ent)
            .perform_async_operation(async move {
                match load_object( portal_guid).await {
                    Ok(Some(portal)) => {
                        if let Ok(exit_point) = portal.data.get::<_, Uuid>(Portal::ExitPoint).cloned() {
                            Ok((Ok(Some(portal)), load_object(exit_point).await))
                        } else {
                            Ok((Ok(Some(portal)), Ok(None)))
                        }
                    },
                    Ok(None) => Ok((Ok(None), Ok(None))),
                    Err(e) => {
                        Ok((Err(e), Ok(None)))
                    }
                }
            })
            .on_finish_run_system(travel_to_portal)
            .on_error_run_system(player_error_handler_system);
    }
}

pub fn apply_class_item_result(
    In((ent, (result, callback))): In<(Entity, (EquipmentResult, Option<Function>))>,
    mut query: Query<(&mut GameObjectData, Option<&Inventory>)>,
    mut commands: Commands,
) {
    if let Ok((mut data, inventory)) = query.get_mut(ent) {
        if inventory.is_some() {
            commands
                .run_system_cached_with(apply_equipment_result, (
                    ent,
                    result,
                ));
            
            if let Some(callback) = callback {
                commands
                    .queue(move |world: &mut World| {
                        world
                            .commands()
                            .entity(ent)
                            .call_lua_method(callback, ());
                    });
            }
        } else {
            if let Some(mut changes) = result.character_update {
                data.apply(changes.as_mut());
            }

            if let Some(callback) = callback {
                commands
                    .entity(ent)
                    .call_lua_method(callback, ());
            } else {
                debug!("No callback")
            }
        }
    } else {
        error!("Player not found!");
    }
}

#[allow(clippy::type_complexity)]
pub fn travel_to_portal(
    In((ent, (portal, exit_point))): In<(Entity, (WorldResult<Option<ObjectPlacement>>, WorldResult<Option<ObjectPlacement>>))>,
    mut query: Query<(&mut Movement, &PlayerController)>,
    instance: Res<ZoneInstance>,
) {
    if let Ok((mut movement, controller)) = query.get_mut(ent) {
        if let Ok(portal) = portal {
            if let Some(portal) = portal {
                controller.send_packet(ServerAction::RemoteEvent("PortalDepartDefault".to_string()).into_pkt());

                if *instance.zone.guid() == portal.zone_guid {
                    let exit_point = if let Ok(Some(exit_point)) = &exit_point {
                        debug!("Exit point found: {}", exit_point.id);
                        exit_point
                    } else {
                        warn!("Portal has no exit point, using portal position");
                        &portal
                    };

                    movement.position = *exit_point.data.get::<_, Vec3>(NonClientBase::Pos).unwrap();
                    movement.rotation = Quat::from_unit_vector(*exit_point.data.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                    movement.velocity = Vec3::ZERO;

                    controller.send_packet(
                        ServerAction::LocalPortal(controller.avatar_id(), movement.clone()).into_pkt()
                    );
                } else {
                    controller.request_travel(portal.zone_guid, None, TravelMode::Portal { uuid: portal.id }, None);
                }
            } else {
                controller.send_message(MessageType::Normal, "Travel failed. Portal not found.");
            }
        } else {
            controller.send_message(MessageType::Normal, "Travel failed. Please try again later.");
        }
    } else {
        error!("Player not found!");
    }
}

