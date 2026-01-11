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

use bevy::{ecs::{event::EventWriter, resource::Resource, system::SystemId}, math::{Quat, Vec3}, prelude::{Added, Changed, Commands, Entity, In, Or, Query, Res, With}};
use log::{debug, error, trace, warn};
use mlua::Function;
use obj_params::{tags::PlayerTag, AttributeInfo, GameObjectData, GenericParamSet, NonClientBase, ParamFlag, ParamSet, Player, Value};
use protocol::{oaAbilityBarReferences, CPktAvatarUpdate};
use realm_api::{AbilitySlot, EquipmentResult, ObjectPlacement, RealmApi, RealmApiResult};
use scripting::{EntityScriptCommandsExt, ScriptObject};
use toolkit::{NativeParam, OtherlandQuatExt};

use crate::{error::WorldResult, instance::ZoneInstance, plugins::{ConnectionState, CurrentState, HealthUpdateEvent, InitialInventoryTransfer, MessageType, Movement, PlayerController, ServerAction}, proto::TravelMode};

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
    mut event: EventWriter<HealthUpdateEvent>
) {
    event.write(HealthUpdateEvent::kill(ent, None));
}

#[derive(Resource)]
#[allow(clippy::type_complexity)]
pub(super) struct PlayerSystems {
    pub(super) apply_class_item_result: SystemId<In<(Entity, RealmApiResult<EquipmentResult>, Option<Function>)>>,
    pub(super) travel_to_portal: SystemId<In<(Entity, WorldResult<Option<ObjectPlacement>>, WorldResult<Option<ObjectPlacement>>)>>,
}

pub fn apply_class_item_result(
    In((ent, result, callback)): In<(Entity, RealmApiResult<EquipmentResult>, Option<Function>)>,
    mut query: Query<&mut GameObjectData>,
    mut commands: Commands,
) {
    match result {
        Ok(res) => {
            if let Some(mut changes) = res.character_update {
                if let Ok(mut data) = query.get_mut(ent) {
                    data.apply(changes.as_mut());
                } else {
                    error!("Player not found!");
                }

                if let Some(callback) = callback {
                    commands
                        .entity(ent)
                        .call_lua_method(callback, ());
                } else {
                    debug!("No callback")
                }
            }
        },
        Err(e) => {
            error!("Failed to apply class item: {e:?}");
            commands
                .entity(ent)
                .despawn();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn travel_to_portal(
    In((ent, portal, exit_point)): In<(Entity, WorldResult<Option<ObjectPlacement>>, WorldResult<Option<ObjectPlacement>>)>,
    mut query: Query<(&mut Movement, &PlayerController)>,
    instance: Res<ZoneInstance>,
) {
    if let Ok((mut movement, controller)) = query.get_mut(ent) {
        if let Ok(portal) = portal {
            if let Some(portal) = portal {
                controller.send_packet(ServerAction::Event("PortalDepart".to_string()).into_pkt());

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
                    controller.request_travel(portal.zone_guid, None, TravelMode::Portal { uuid: portal.id });
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

