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

use bevy::{ecs::{component::Component, error::BevyError, event::EventWriter, lifecycle::RemovedComponents, resource::Resource, system::{EntityCommands, SystemId}, world::EntityWorldMut}, math::{Quat, Vec3}, platform::collections::HashMap, prelude::{Added, Changed, Commands, Entity, In, Or, Query, Res, With}, time::{Time, Virtual}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{debug, error, trace, warn};
use mlua::Function;
use obj_params::{tags::PlayerTag, AttributeInfo, GameObjectData, GenericParamSet, NonClientBase, ParamFlag, ParamSet, ParamWriter, Player, Value};
use protocol::{oaAbilityBarReferences, CPktAvatarUpdate, CPktBlob, CPktServerNotify, MoveManagerInit, Physics};
use realm_api::{AbilitySlot, EquipmentResult, ObjectPlacement, RealmApiResult};
use scripting::{EntityScriptCommandsExt, ScriptObject};
use toolkit::{NativeParam, OtherlandQuatExt};

use crate::{error::WorldResult, instance::ZoneInstance, plugins::{Avatar, ConnectionState, CurrentState, HealthUpdateEvent, InitialInventoryTransfer, MessageType, Movement, PlayerController, ServerAction}, proto::TravelMode};

#[derive(Component)]
pub struct EnableClientUpdates;

/*#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn insert_player_characters(
    mut receiver: ResMut<ForeignResource<Receiver<(Entity, Character, Skillbook)>>>,
    controller: Query<&PlayerController>,
    starting_points: Query<&GameObjectData, With<StartingPointTag>>,
    portals: Query<(&ContentInfo, &GameObjectData), With<PortalTag>>,
    exit_nodes: Query<(&ContentInfo, &GameObjectData), With<SpawnNodeTag>>,
    instance: Res<ZoneInstance>,
    navmesh: Res<Navmesh>,
    cooldown_groups: Res<CooldownGroups>,
    mut commands: Commands,
) {
    while let Ok((entity, mut character, skillbook)) = receiver.try_recv() {
        if let Ok(controller) = controller.get(entity) {
            let collision_extent;

            // Update zone info in character data
            {
                let obj: &mut GameObjectData = character.data_mut();

                collision_extent = *obj.get::<_, Vec3>(Player::CollisionExtent).unwrap();

                // First time spawn setup
                if *obj.get(Player::FirstTimeSpawn).unwrap() {
                    obj.set(Player::HpCur, obj.get::<_, Value>(Player::HpMax).unwrap().clone());

                    // Lookup the entrypoint
                    if let Some(starting_point) = starting_points.iter().next() {
                        obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                        obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                    } else {
                        error!("Starting point not found!");
                    }

                    //obj.set(Player::FirstTimeSpawn, false);

                    obj.set(Player::SpawnMode, 1);
                } else {
                    match controller.travel_mode() {
                        TravelMode::Login => {
                            obj.set(Player::SpawnMode, 2);
                        },
                        TravelMode::Portal { uuid } => {
                            if 
                                let Some((_, portal)) = portals.iter()
                                    .find(|(info, _)| info.placement_id == uuid) &&
                                let Some(exit_point_id) = portal.get::<_, String>(Portal::ExitPoint).ok()
                                    .and_then(|s| s.parse::<Uuid>().ok()) &&
                                let Some((_, exit_point)) = exit_nodes.iter()
                                    .find(|(info, _)| info.placement_id == exit_point_id)
                            {
                                obj.set(Player::Pos, (0u32, *exit_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                obj.set(Player::Rot, *exit_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                            } else {
                                warn!("No exit node found for portal {uuid}");

                                // Lookup the entrypoint
                                if let Some(starting_point) = starting_points.iter().next() {
                                    obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                    obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                                } else {
                                    error!("Starting point not found!");
                                }
                            }

                            obj.set(Player::SpawnMode, 4);
                        },
                        TravelMode::Position { pos, rot } => {
                            obj.set(Player::Pos, pos);
                            obj.set(Player::Rot, rot);
                            obj.set(Player::SpawnMode, 3);
                        },
                        TravelMode::EntryPoint => {
                            // Lookup the entrypoint
                            if let Some(starting_point) = starting_points.iter().next() {
                                obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                            } else {
                                error!("Starting point not found!");
                            }

                            obj.set(Player::SpawnMode, 3);
                        },
                    }
                }

                // Snap to floor
                {
                    let mut pos = obj.get::<_, (u32, Vec3)>(Player::Pos).unwrap().1;
                    pos.y = navmesh.get_floor_height(pos)
                        .unwrap_or_else(|| {
                            error!("Failed to get floor height for player at position {pos}");
                            pos.y
                        }) + collision_extent.y;

                    obj.set(Player::Pos, (0u32, pos));
                }

                // Update stance data
                obj.set(Player::ClassData, oaPlayerClassData {
                    class_hash: 0x9D35021A,
                    ..Default::default()
                }.to_bytes());

                // Update zone info in player data
                obj.set(Player::WorldMapGuid, instance.world_def.guid().to_string());
                obj.set(Player::ZoneGuid, instance.zone.guid().to_string());
                obj.set(Player::InstanceZoneKey, instance.instance_id.map(|v| v.to_string()).unwrap_or_default());

                obj.set(Player::ClientReady, false);
                obj.set(Player::PlayerLoading, true);
            }

            let movement = Movement {
                position: character.data().get::<_, (u32, Vec3)>(Player::Pos).unwrap().1,
                rotation: Quat::from_unit_vector(*character.data().get::<_, Vec3>(Player::Rot).unwrap()),
                velocity: Vec3::ZERO,
                radius: collision_extent.x.max(collision_extent.z),
                mode: PhysicsState::Walking,
                mover_type: 1,
                mover_replication_policy: 7,
                version: 0,
                mover_key: 0,
                seconds: 0.0,
            };

            let mut factions = Factions::default();

            for faction in character.data().get::<_, ContentRefList>(Player::Faction).unwrap().iter() {
                factions.add_faction(faction.id);
            }

            // Insert character into world
            commands.entity(entity)
                .insert((
                    AvatarInfo {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    movement,
                    cooldown_groups.create_cooldowns(),
                    skillbook,
                    factions
                ));
        }
    }
}*/

pub fn begin_loading_sequence(
    query: Query<(Entity, &PlayerController, &Avatar, &GameObjectData, &Movement), Added<GameObjectData>>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
) {
    for (ent, controller, avatar, obj, movement) in query.iter() {
        debug!("Starting spawning sequence for character: {}", avatar.name);

        let mut serialized = Vec::new();
        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

        // Send character to client, so it begins loading the level
        if matches!(controller.travel_mode(), TravelMode::Login) {
            obj.write_to_privileged_client(&mut writer).unwrap();

            controller.send_packet(CPktServerNotify {
                notify_type: protocol::CpktServerNotifyNotifyType::SyncGameClock,
                game_clock: Some(time.elapsed_secs_f64()),
                ..Default::default()
            });

            controller.send_packet(CPktBlob {
                avatar_id: controller.avatar_id(),
                avatar_name: avatar.name.clone(),
                class_id: obj.class().id() as u32,
                params: serialized.into(),
                movement: MoveManagerInit {
                    pos: movement.position.into(),
                    rot: movement.rotation.into(),
                    vel: movement.velocity.into(),
                    physics: Physics {
                        state: movement.mode,
                    },
                    mover_type: movement.mover_type,
                    mover_replication_policy: movement.mover_replication_policy,
                    version: movement.version,
                    ..Default::default()
                }.to_bytes().into(),
                has_guid: true,
                field_7: Some(*controller.session().id()),
                ..Default::default()
            });
        } else {
            let changes = obj.changes().
                filter(|(attr, _)| !attr.has_flag(&ParamFlag::ClientUnknown))
                .collect::<Box<dyn GenericParamSet>>();

            changes.write_to_privileged_client(&mut writer).unwrap();

            controller.send_packet(CPktAvatarUpdate {
                full_update: false,
                avatar_id: Some(controller.avatar_id()),
                name: Some(avatar.name.clone()),
                class_id: Some(obj.class().id() as u32),
                params: serialized.into(),
                ..Default::default()
            });
        }

        commands.entity(ent).insert(EnableClientUpdates);
    }
}

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
        let realm_api = instance.realm_api.clone();

        if 
            let Some(Value::Any(ability_bar)) = volatile_diff.get_param(Player::CurrentAbilityBarReferences.name()) &&
            let Ok((_, current_ability_bar)) = oaAbilityBarReferences::from_bytes(ability_bar)
        {
            let mut ability_bar = realm_api.create_empty_ability_bar(id);
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
                if let Err(e) = realm_api.update_character_data_diff(&id, persistent_diff).await {
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


#[derive(Component, Default)]
pub struct PlayerLocalSets(pub HashMap<Entity, Box<dyn GenericParamSet>>);

pub fn remove_local_changed(
    mut removed: RemovedComponents<PlayerTag>,
    mut query: Query<(Entity, &mut PlayerLocalSets)>,
) {
    for ent in removed.read() {
        if let Ok((_, mut changes)) = query.get_mut(ent) {
            changes.0.remove(&ent);
        }
    }
}

pub fn disconnect_player_error_handler(error: BevyError, commands: &mut EntityCommands<'_>) {
    error!("Error loading player component: {error:?}");

    commands
        .queue(|entity: EntityWorldMut| {
            if let Some(controller) = entity.get::<PlayerController>() {
                controller.close();
            }
        });
}
