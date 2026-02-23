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

use anyhow::anyhow;
use bevy::{ecs::{entity::Entity, query::Changed, system::{Commands, In, Query}, world::World}};
use futures::TryStreamExt;
use log::debug;
use mlua::{Function, Lua, Table};
use obj_params::Portal;
use protocol::{OaPktS2xconnectionStateState, oaPktConfirmTravel, oaPktS2XConnectionState};
use realm_api::{ObjectPlacement, RealmApi, Zone};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::types::Uuid;

use crate::{error::{WorldError, WorldResult}, plugins::{Active, AsyncOperationEntityCommandsExt, Avatar, ConnectionState, ContentCache, ContentCacheRef, CurrentState, EquipmentResult, Movement, PlayerController, ServerAction, WeakCache, apply_class_item_result, player::loader::InGame, player_error_handler_system, travel_to_portal}, proto::TravelMode};

pub(super) fn insert_player_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let player_api = lua.create_table().unwrap();
    runtime.register_native("player", player_api.clone()).unwrap();

    player_api.set("Spawn", lua.create_bevy_function(world, 
        |
            In(player): In<Table>,
            mut query: Query<(Entity, &Avatar, &Movement, &mut PlayerController, &mut CurrentState), Changed<CurrentState>>,
            mut commands: Commands
        | -> WorldResult<()> {
            if let Ok((ent, info, movement, controller, mut state)) = query.get_mut(player.entity()?) {
                debug!("Spawning player: {}", info.name);

                state.state = ConnectionState::InGame;
    
                controller.send_packet(oaPktS2XConnectionState {
                    state: OaPktS2xconnectionStateState::InGame,
                    ..Default::default()
                });
    
                let spawn_action = match controller.travel_mode() {
                    TravelMode::Login => ServerAction::DirectTravel(info.id, Some(movement.clone())),
                    TravelMode::EntryPoint => ServerAction::NonPortalTravel(info.id, Some(movement.clone())),
                    TravelMode::Portal { .. } => ServerAction::Portal(info.id, Some(movement.clone())), 
                    TravelMode::Position { .. } => ServerAction::DirectTravel(info.id, Some(movement.clone())),
                };
    
                controller.send_packet(spawn_action.into_pkt());
                commands
                    .entity(ent)
                    .insert((Active, InGame));

                Ok(())
            } else {
                Err(anyhow!("Player not found!").into())
            }
        })?)?;

    player_api.set("ApplyClassItem", lua.create_bevy_function(world,
        |
            In((player, class_item, clear_inventory, callback)): In<(Table, String, bool, Option<Function>)>,
            query: Query<(Entity, &PlayerController)>,
            mut commands: Commands
        | -> WorldResult<()> {
            let (ent, controller) = query.get(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let character_id = controller.character_id();

            commands
                .entity(ent)
                .perform_async_operation(async move {
                    Ok((
                        EquipmentResult::from_result(
                            RealmApi::get()
                                .character_apply_class_item(
                                    &character_id, 
                                    &class_item, 
                                    clear_inventory
                                ).await?
                        ).await?, 
                        callback
                    ))
                })
                .on_finish_run_system(apply_class_item_result)
                .on_error_run_system(player_error_handler_system);

            Ok(())
        })?)?;

    player_api.set("TravelToZone", lua.create_bevy_function(world,
        |
            In((player, zone, movie)): In<(Table, String, Option<String>)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            let ent = player.entity()?;

            commands
                .entity(ent)
                .perform_async_operation(async move {
                    let mut cursor = RealmApi::get()
                        .query_zones()
                        .zone(zone.clone())
                        .query()
                        .await?;

                    match cursor.try_next().await {
                        Ok(Some(zone)) => Ok((zone, movie)),
                        Ok(None) => Err(anyhow!("Zone '{zone}' not found!").into()),
                        Err(e) => Err(anyhow!("Failed to travel to zone '{zone}': {e:?}").into())
                    }
                })
                .on_finish_run_system(|
                    In((ent, (zone, movie))): In<(Entity, (Zone, Option<String>))>,
                    query: Query<&PlayerController>,
                | {
                    if let Ok(controller) = query.get(ent) {
                        controller
                            .request_travel(*zone.guid(), None, TravelMode::EntryPoint, movie);
                    }
                })
                .on_error_run_system(player_error_handler_system);

            Ok(())
        })?)?;

    player_api.set("TravelToPortal", lua.create_bevy_function(world,
        |
            In((player, portal_guid)): In<(Table, String)>,
            mut commands: Commands
        | -> WorldResult<()> {
            let ent = player.entity()?;
            let portal_guid = portal_guid.parse::<Uuid>()?;

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

            Ok(())
        })?)?;

        player_api.set("ConfirmTravel", lua.create_bevy_function(world,
            |
                In(player): In<Table>,
                query: Query<&PlayerController>,
            | -> WorldResult<()> {
                if let Ok(controller) = query.get(player.entity()?) {
                    controller.send_packet(oaPktConfirmTravel {
                        state: 1,
                        ..Default::default()
                    });
                }
    
                Ok(())
            })?)?;

        player_api.set("RunCinematic", lua.create_bevy_function(world,
            |
                In((player, cinematic_name, level)): In<(Table, String, Option<String>)>,
                query: Query<(&Avatar, &PlayerController)>,
            | -> WorldResult<()> {
                if let Ok((avatar, controller)) = query.get(player.entity()?) {
                    controller.send_packet(
                        ServerAction::Cinematic { 
                            player: avatar.id,
                            name: cinematic_name, 
                            level,
                            position: None
                        }.into_pkt()
                    );
                }

                Ok(())
            })?)?;

        player_api.set("TriggerRemoteEvent", lua.create_bevy_function(world,
            |
                In((player, event)): In<(Table, String)>,
                query: Query<&PlayerController>,
            | -> WorldResult<()> {
                if let Ok(controller) = query.get(player.entity()?) {
                    controller.send_packet(
                        ServerAction::RemoteEvent(event).into_pkt()
                    );
                }

                Ok(())
            })?)?;


    Ok(())
}