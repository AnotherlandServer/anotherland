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
use bevy::ecs::{entity::Entity, query::Changed, system::{Commands, In, Query, Res}, world::World};
use futures::TryStreamExt;
use log::{debug, error};
use mlua::{Function, Lua, Table};
use obj_params::Portal;
use protocol::{OaPktS2xconnectionStateState, oaPktConfirmTravel, oaPktS2XConnectionState};
use realm_api::{ObjectPlacement, RealmApi};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::types::Uuid;

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance, plugins::{Active, Avatar, ConnectionState, ContentCache, ContentCacheRef, CurrentState, FutureCommands, MessageType, Movement, PlayerController, ServerAction, WeakCache, player::PlayerSystems}, proto::TravelMode};

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
                commands.entity(ent).insert(Active);

                Ok(())
            } else {
                Err(anyhow!("Player not found!").into())
            }
        })?)?;

    player_api.set("ApplyClassItem", lua.create_bevy_function(world,
        |
            In((player, class_item, clear_inventory, callback)): In<(Table, String, bool, Option<Function>)>,
            query: Query<(Entity, &PlayerController)>,
            systems: Res<PlayerSystems>,
            mut commands: Commands
        | -> WorldResult<()> {
            let (ent, controller) = query.get(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let character_id = controller.character_id();

            commands.run_system_async(async move {
                (ent, RealmApi::get().character_apply_class_item(&character_id, &class_item, clear_inventory).await, callback)
            }, systems.apply_class_item_result);

            Ok(())
        })?)?;

    player_api.set("TravelToZone", lua.create_bevy_function(world,
        |
            In((player, zone)): In<(Table, String)>,
            query: Query<&PlayerController>,
            instance: Res<ZoneInstance>
        | -> WorldResult<()> {
            let ent = player.entity()?;
            
            if let Ok(controller) = query.get(ent).cloned() {
                instance.spawn_task(async move {
                    match 
                        RealmApi::get()
                            .query_zones()
                            .zone(zone.clone())
                            .query()
                            .await 
                    {
                        Ok(mut cursor) => {
                            match cursor.try_next().await {
                                Ok(Some(zone)) => {
                                    controller
                                        .request_travel(*zone.guid(), None, TravelMode::EntryPoint);
                                },
                                Ok(None) => {
                                    controller.send_message(MessageType::IllegalZone, "Travel failed. Zone not found!");
                                },
                                Err(e) => {
                                    error!("Failed to travel to zone '{zone}': {e:?}");
                                    controller.send_message(MessageType::IllegalZone, "Travel failed. Server error!");
                                }
                            }
                        },
                        Err(e) => {
                            error!("Failed to travel to zone '{zone}': {e:?}");
                            controller.send_message(MessageType::IllegalZone, "Travel failed. Server error!");
                        }
                    }
                });
            }

            Ok(())
        })?)?;

    player_api.set("TravelToPortal", lua.create_bevy_function(world,
        |
            In((player, portal_guid)): In<(Table, String)>,
            systems: Res<PlayerSystems>,
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

            commands.run_system_async(async move {
                match load_object( portal_guid).await {
                    Ok(Some(portal)) => {
                        if let Ok(exit_point) = portal.data.get::<_, Uuid>(Portal::ExitPoint).cloned() {
                            (ent, Ok(Some(portal)), load_object(exit_point).await)
                        } else {
                            (ent, Ok(Some(portal)), Ok(None))
                        }
                    },
                    Ok(None) => (ent, Ok(None), Ok(None)),
                    Err(e) => {
                        (ent, Err(e), Ok(None))
                    }
                }
            }, systems.travel_to_portal);

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

    Ok(())
}