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

use bevy::{app::{First, Last, Plugin, PostUpdate, Update}, math::{Quat, Vec3, VectorSpace}, prelude::{in_state, Added, Changed, Commands, Entity, In, IntoSystemConfigs, Query, Res, ResMut, With}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{debug, error, warn};
use obj_params::{tags::{PlayerTag, PortalTag, SpawnNodeTag, StartingPointTag}, GameObjectData, GenericParamSet, NonClientBase, Param, ParamFlag, ParamSet, ParamWriter, Player, Portal, Value};
use protocol::{oaPktItemStorage, oaPktS2XConnectionState, CPkt, CPktAvatarUpdate, CPktBlob, ItemStorageParams, MoveManagerInit, OaPktItemStorageUpdateType, OaPktS2xconnectionStateState, OtherlandPacket, Physics, PhysicsState};
use realm_api::Character;
use scripting::LuaRuntime;
use tokio::sync::mpsc::{self, Receiver, Sender};
use toolkit::{types::{Uuid, UUID_NIL}, OtherlandQuatExt};

use crate::{instance::{InstanceState, ZoneInstance}, plugins::{Active, ForeignResource}, proto::TravelMode};

use super::{init_gameobjects, AvatarIdManager, AvatarInfo, ConnectionState, ContentInfo, CurrentState, Inventory, Movement, NetworkExtPriv, PlayerController, QuestLog, ServerAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (character_sender, character_receiver) = mpsc::channel::<(Entity, Character)>(10);

        app.insert_resource(ForeignResource(character_sender));
        app.insert_resource(ForeignResource(character_receiver));

        app.add_systems(First, (
            request_player_characters, 
            insert_player_characters.before(init_gameobjects)
        ).run_if(in_state(InstanceState::Running)));

        app.add_systems(Update, spawn_player);
        app.add_systems(Last, (
            begin_loading_sequence,
            save_player_data
        ));
        
        app.register_message_handler(handle_avatar_update);
    }
}

fn request_player_characters(
    query: Query<(Entity, &PlayerController), Added<PlayerController>>,
    instance: Res<ZoneInstance>,
    sender: Res<ForeignResource<Sender<(Entity, Character)>>>,
) {
    for (entity, controller) in query.iter() {
        let realm_api = instance.realm_api.clone();
        let sender = sender.clone();
    
        let state = controller.state().clone();
    
        instance.handle.spawn(async move {
            if let Ok(Some(character)) = realm_api.get_character(state.character()).await {
                let _ = sender.send((entity, character)).await;
            }
        });
    }
}

fn insert_player_characters(
    mut receiver: ResMut<ForeignResource<Receiver<(Entity, Character)>>>,
    controller: Query<&PlayerController>,
    starting_points: Query<&GameObjectData, With<StartingPointTag>>,
    portals: Query<(&ContentInfo, &GameObjectData), With<PortalTag>>,
    exit_nodes: Query<(&ContentInfo, &GameObjectData), With<SpawnNodeTag>>,
    instance: Res<ZoneInstance>,
    mut commands: Commands,
) {
    while let Ok((entity, mut character)) = receiver.try_recv() {
        if let Ok(controller) = controller.get(entity) {

            // Update zone info in character data
            {
                let obj: &mut GameObjectData = character.data_mut();

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

                    obj.set(Player::FirstTimeSpawn, false);

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
                                warn!("No exit node found for portal {}", uuid);

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
                mode: PhysicsState::Walking,
                mover_type: 1,
                mover_replication_policy: 7,
                version: 0,
            };

            // Insert character into world
            commands.entity(entity)
                .insert((
                    AvatarInfo {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    movement,
                    QuestLog::default(),
                ));
        }
    }
}

fn begin_loading_sequence(
    query: Query<(&PlayerController, &AvatarInfo, &GameObjectData, &Movement), Added<Inventory>>,
) {
    for (controller, avatar, obj, movement) in query.iter() {
        debug!("Starting spawning sequence for character: {}", avatar.name);

        let mut serialized = Vec::new();
        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

        obj.write_to_privileged_client(&mut writer).unwrap();

        // Send character to client, so it begins loading the level
        if matches!(controller.travel_mode(), TravelMode::Login) {
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
            controller.send_packet(CPktAvatarUpdate {
                full_update: true,
                avatar_id: Some(controller.avatar_id()),
                name: Some(avatar.name.clone()),
                class_id: Some(obj.class().id() as u32),
                movement: Some(MoveManagerInit {
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
                }.to_bytes().into()),
                params: serialized.into(),
                ..Default::default()
            });
        }

        controller.send_packet(oaPktItemStorage {
            storage_id: Uuid::new(),
            update_type: OaPktItemStorageUpdateType::Unknown004,
            data: ItemStorageParams {
                storage_name: "inventory".to_string(),
                storage_size: 30,
                bling_amount: 0,
                has_bling: false,
            }.to_bytes(),
            ..Default::default()
        });
    }
}

fn spawn_player(
    mut query: Query<(Entity, &AvatarInfo, &Movement, &mut PlayerController, &mut CurrentState), Changed<CurrentState>>,
    mut commands: Commands
) {
    for (ent, info, movement, controller, mut state) in query.iter_mut() {
        if matches!(state.state, ConnectionState::InitialInterestsLoaded) {
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
        }
    }
}

fn save_player_data(
    query: Query<(&PlayerController, &GameObjectData), (Changed<GameObjectData>, With<PlayerTag>)>,
    instance: Res<ZoneInstance>,
) {
    for (controller, obj) in query.iter() {
        let id = *controller.state().character();
        let diff = obj.changes()
            .filter(|(attr, _)| attr.has_flag(&ParamFlag::Persistent))
            .collect::<Box<dyn GenericParamSet>>();
        let realm_api = instance.realm_api.clone();

        if !diff.is_empty() {
            // We probably should move this into it's own task and just 
            // send a (blocking) message here, se we can have
            // backpressure in case our updates don't go trough.
            // Also, errors are not really handled here.
            instance.handle.spawn(async move {
                if let Err(e) = realm_api.update_character_data_diff(&id, diff).await {
                    error!("Character update failed: {:?}", e);
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

        obj.apply(params.as_mut());
    }
}

