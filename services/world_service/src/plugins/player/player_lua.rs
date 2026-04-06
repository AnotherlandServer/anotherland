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

use anyhow::anyhow;
use bevy::{app::App, ecs::{entity::Entity, query::Changed, system::{Commands, In, Query}}};
use futures::TryStreamExt;
use log::debug;
use mlua::Function;
use obj_params::Portal;
use protocol::{OaPktS2xconnectionStateState, oaPktConfirmTravel, oaPktS2XConnectionState};
use realm_api::{ObjectPlacement, RealmApi, Zone};
use scripting::{LuaEntity, ScriptAppExt};
use toolkit::{QuatWrapper, Vec3Wrapper, types::Uuid};

use crate::{error::{WorldError, WorldResult}, plugins::{Active, AsyncOperationEntityCommandsExt, Avatar, ConnectionState, ContentCache, ContentCacheRef, CurrentState, EquipmentResult, MessageType, Movement, PlayerController, ServerAction, WeakCache, apply_class_item_result, player::{loader::InGame, stance::Stance}, player_error_handler_system, travel_to_portal}, proto::TravelMode};

pub(super) fn insert_player_api(app: &mut App,) {
    app
        .add_lua_api("player", "Spawn",
        |
            In(player): In<LuaEntity>,
            mut query: Query<(&Avatar, &Movement, &mut PlayerController, &mut CurrentState), Changed<CurrentState>>,
            mut commands: Commands
        | -> WorldResult<()> {
            if let Ok((info, movement, mut controller, mut state)) = query.get_mut(player.entity()) {
                debug!("Spawning player: {}", info.name);

                state.state = ConnectionState::InGame;
    
                controller.send_packet(oaPktS2XConnectionState {
                    state: OaPktS2xconnectionStateState::InGame,
                    ..Default::default()
                });
    
                let spawn_action = controller.take_travel_mode()
                    .map(|mode| match mode {
                        TravelMode::Login => ServerAction::DirectTravel(info.id, Some((movement.position, movement.rotation))),
                        TravelMode::EntryPoint => ServerAction::NonPortalTravel(info.id, Some((movement.position, movement.rotation))),
                        TravelMode::Portal { .. } => ServerAction::Portal(info.id, Some((movement.position, movement.rotation))), 
                        TravelMode::Position { .. } => ServerAction::DirectTravel(info.id, Some((movement.position, movement.rotation))),
                    });
    
                if let Some(spawn_action) = spawn_action {
                    controller.send_packet(spawn_action.into_pkt());
                }

                commands
                    .entity(player.entity())
                    .insert((Active, InGame));

                Ok(())
            } else {
                Err(anyhow!("Player not found!").into())
            }
        })
        .add_lua_api("player", "ApplyClassItem",
        |
            In((player, class_item, clear_inventory, callback)): In<(LuaEntity, String, bool, Option<Function>)>,
            query: Query<&PlayerController>,
            mut commands: Commands
        | -> WorldResult<()> {
            let controller = query.get(player.entity())
                .map_err(|_| anyhow!("player not found"))?;

            let character_id = controller.character_id();

            commands
                .entity(player.entity())
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
        })
        .add_lua_api("player", "TravelToZone",
        |
            In((player, zone, movie)): In<(LuaEntity, String, Option<String>)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            commands
                .entity(player.entity())
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
                    query: Query<(&Avatar, &Movement, &PlayerController)>,
                | {
                    if let Ok((avatar, movement, controller)) = query.get(ent) {
                        controller.send_packet(ServerAction::Cinematic { 
                            player: avatar.id, 
                            name: "PortalDepartDefault".to_owned(), 
                            level: None, 
                            position: Some((movement.position, movement.rotation)), 
                        }.into_pkt());

                        controller
                            .request_travel(*zone.guid(), None, TravelMode::EntryPoint, movie);
                    }
                })
                .on_error_run_system(player_error_handler_system);

            Ok(())
        })
        .add_lua_api("player", "TravelToPortal",
        |
            In((player, portal_guid)): In<(LuaEntity, String)>,
            mut commands: Commands
        | -> WorldResult<()> {
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
                .entity(player.entity())
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
        })
        .add_lua_api("player", "ConfirmTravel",
            |
                In(player): In<LuaEntity>,
                query: Query<&PlayerController>,
            | -> WorldResult<()> {
                if let Ok(controller) = query.get(player.entity()) {
                    controller.send_packet(oaPktConfirmTravel {
                        state: 1,
                        ..Default::default()
                    });
                }
    
                Ok(())
            })
            .add_lua_api("player", "RunCinematic",
            |
                In((player, cinematic_name, level)): In<(LuaEntity, String, Option<String>)>,
                query: Query<(&Avatar, &Movement, &PlayerController)>,
            | -> WorldResult<()> {
                if let Ok((avatar, movement, controller)) = query.get(player.entity()) {
                    controller.send_packet(
                        ServerAction::Cinematic { 
                            player: avatar.id,
                            name: cinematic_name, 
                            level,
                            position: Some((movement.position, movement.rotation))
                        }.into_pkt()
                    );
                }

                Ok(())
            })
            .add_lua_api("player", "TriggerRemoteEvent",
            |
                In((player, event)): In<(LuaEntity, String)>,
                query: Query<(&Movement, &PlayerController)>,
            | -> WorldResult<()> {
                if let Ok((movement, controller)) = query.get(player.entity()) {
                    controller.send_packet(
                        ServerAction::RemoteEvent(event, (movement.position, movement.rotation)).into_pkt()
                    );
                }

                Ok(())
            })
            .add_lua_api("player", "SendMessage",
            |
                In((player, message, message_type)): In<(LuaEntity, String, Option<String>)>,
                query: Query<&PlayerController>,
            | -> WorldResult<()> {
                if let Ok(controller) = query.get(player.entity()) {
                    let msg_type = match message_type.as_deref() {
                        Some("Normal") => MessageType::Normal,
                        Some("Combat") => MessageType::Combat,
                        Some("Console") => MessageType::Console,
                        Some("Clan") => MessageType::Clan,
                        Some("Party") => MessageType::Party,
                        Some("Xp") => MessageType::Xp,
                        Some("Loot") => MessageType::Loot,
                        Some("Quest") => MessageType::Quest,
                        Some("PopUp") => MessageType::PopUp,
                        Some("IllegalZone") => MessageType::IllegalZone,
                        _ => MessageType::Normal
                    };

                    controller.send_message(msg_type, message);
                }

                Ok(())
            })
            .add_lua_api("player", "Respawn",
            |
                In((player, position, rotation)): In<(LuaEntity, Vec3Wrapper, QuatWrapper)>,
                query: Query<&PlayerController>,
            | -> WorldResult<()> {
                let Ok(controller) = query.get(player.entity()) else {
                    return Err(anyhow!("Player not found").into());
                };

                controller.send_packet(
                    ServerAction::Respawn(controller.avatar_id(), (
                        position.0, 
                        rotation.0
                    )).into_pkt()
                );

                Ok(())
            })
            .add_lua_api("player", "ChangeStance",
            |
                In((player, id, rank)): In<(LuaEntity, u8, u8)>,
                mut query: Query<&mut Stance>,
            | -> WorldResult<()> {
                let Ok(mut stance) = query.get_mut(player.entity()) else {
                    return Err(anyhow!("Player not found").into());
                };

                stance.update_stance(id, rank)
            });
}