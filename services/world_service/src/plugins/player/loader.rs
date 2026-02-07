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
use bevy::{ecs::{error::{BevyError, Result}, query::With, system::EntityCommands, world::EntityWorldMut}, math::{Quat, Vec3}, time::{Time, Virtual}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{debug, error, warn};
use obj_params::{ContentRefList, GameObjectData, GenericParamSet, NonClientBase, ParamFlag, ParamWriter, Player, Portal, tags::{PortalTag, SpawnNodeTag, StartingPointTag}};
use protocol::{AbilityBarReference, CPktAvatarUpdate, CPktBlob, CPktServerNotify, MoveManagerInit, Physics, PhysicsState, oaAbilityBarReferences, oaPlayerClassData};
use realm_api::{Character, RealmApi};
use toolkit::{OtherlandQuatExt, types::Uuid};

use crate::{instance::ZoneInstance, plugins::{Avatar, AvatarLoader, CombatStyle, ComponentLoaderCommandsTrait, ContentInfo, CooldownGroups, Factions, FactionsParameters, LoadContext, LoadableComponent, Movement, Navmesh, PlayerController, QuestLog, Skillbook, SkillbookParams}, proto::TravelMode};

impl AvatarLoader {
    pub async fn load_player_character(character_id: Uuid) -> Result<Character> {
        let mut character = RealmApi::get()
            .get_character(&character_id).await?
            .ok_or(anyhow!("Character not found"))?;
        
        let ability_bar = RealmApi::get()
            .get_or_create_ability_bar(character_id).await?;

        // Build ability bar in the clients binary format
        let main_skill_bar = [()]
            .repeat(7)
            .into_iter()
            .enumerate()
            .map(|(i, _)| {
                if let Some(entry) = ability_bar.slots.get(i) {
                    AbilityBarReference { id: entry.id, skill: entry.ability.clone() }
                } else {
                    AbilityBarReference { id: -1, ..Default::default() }
                }
            })
            .collect::<Vec<_>>();

        character.data_mut().set(Player::CurrentAbilityBarReferences, oaAbilityBarReferences {
            class_hash: 0xFE0D0DC2,
            version: 1,
            count: main_skill_bar.len() as u32,
            main_skill_bar,
            single_slot_bar: AbilityBarReference {
                id: ability_bar.single_slot.id,
                skill: ability_bar.single_slot.ability,
            },
        }.to_bytes());

        Ok(character)
    }

    pub fn load_player_character_dependencies(&mut self, _: &mut EntityCommands<'_>, context: &mut LoadContext<<AvatarLoader as LoadableComponent>::ContextData>, character: &mut Character) -> Result<()> {
        context
            .load_dependency::<Skillbook>(SkillbookParams {
                character_id: *character.id(),
                level: *character.data().get::<_, i32>(Player::Lvl).unwrap(),
                combat_style: CombatStyle::from_id(
                    *character.data().get::<_, i32>(Player::CombatStyle).unwrap()
                ),
            })
            .load_dependency::<Factions>(FactionsParameters {
                factions: character.data().get::<_, ContentRefList>(Player::Faction).unwrap().clone(),
            });
            
        Ok(())
    }

    pub fn on_load_player_character(&mut self, commands: &mut EntityCommands<'_>, mut character: Character) -> Result<()> {
        let id = *character.id();

        commands
            .queue(|mut entity: EntityWorldMut<'_>| {
                Self::prepare_character_spawn(&mut entity, &mut character);

                let controller: &PlayerController = entity.get::<PlayerController>()
                    .expect("PlayerController component missing");

                let cooldowns = entity.resource::<CooldownGroups>();

                let collision_extent: Vec3 = *character.data().get::<_, Vec3>(Player::CollisionExtent).unwrap();

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

                entity.insert((
                    Avatar {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    cooldowns.create_cooldowns(),
                    movement,
                ));
            })
            .queue(Self::begin_loading_sequence)
            .load_component::<QuestLog>(id);

        Ok(())
    }

    fn prepare_character_spawn(entity: &mut EntityWorldMut<'_>, character: &mut Character) {
        let obj: &mut GameObjectData = character.data_mut();

        // Update zone info in player data
        {
            let instance = entity.resource::<ZoneInstance>();

            obj.set(Player::WorldMapGuid, instance.world_def.guid().to_string());
            obj.set(Player::ZoneGuid, instance.zone.guid().to_string());
            obj.set(Player::InstanceZoneKey, instance.instance_id.map(|v| v.to_string()).unwrap_or_default());

            obj.set(Player::ClientReady, false);
            obj.set(Player::PlayerLoading, true);
        }

        // Lookup the entrypoint
        let (entrypoint_pos, entrypoint_rot) = {
            if 
                let Some(mut entrypoints) = entity.world()
                    .try_query_filtered::<&GameObjectData, With<StartingPointTag>>() &&
                let Some(entrypoint) = entrypoints.iter(entity.world()).next() 
            {
                (
                    *entrypoint.get::<_, Vec3>(NonClientBase::Pos).unwrap(),
                    *entrypoint.get::<_, Vec3>(NonClientBase::Rot).unwrap()
                )
            } else {
                error!("Starting point not found!");
                (Vec3::ZERO, Vec3::ZERO)
            }
        };

        // First time spawn setup
        if *obj.get(Player::FirstTimeSpawn).unwrap() {
            obj.set(Player::Pos, (0u32, entrypoint_pos));
            obj.set(Player::Rot, entrypoint_rot);
            obj.set(Player::FirstTimeSpawn, false);
            obj.set(Player::SpawnMode, 1);
        } else {
            let controller = entity.get::<PlayerController>().unwrap();

            match controller.travel_mode() {
                TravelMode::Login => {
                    obj.set(Player::SpawnMode, 2);
                },
                TravelMode::Portal { uuid } => {
                    let mut portals = entity.world()
                        .try_query_filtered::<(&ContentInfo, &GameObjectData), With<PortalTag>>().unwrap();

                    let mut exit_nodes = entity.world()
                        .try_query_filtered::<(&ContentInfo, &GameObjectData), With<SpawnNodeTag>>().unwrap();


                    if 
                        let Some((_, portal)) = portals.iter(entity.world())
                            .find(|(info, _)| info.placement_id == uuid) &&
                        let Some(exit_point_id) = portal.get::<_, String>(Portal::ExitPoint).ok()
                            .and_then(|s| s.parse::<Uuid>().ok()) &&
                        let Some((_, exit_point)) = exit_nodes.iter(entity.world())
                            .find(|(info, _)| info.placement_id == exit_point_id)
                    {
                        obj.set(Player::Pos, (0u32, *exit_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                        obj.set(Player::Rot, *exit_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                    } else {
                        warn!("No exit node found for portal {uuid}");

                        // Fallback to entrypoint
                        obj.set(Player::Pos, (0u32, entrypoint_pos));
                        obj.set(Player::Rot, entrypoint_rot);
                    }

                    obj.set(Player::SpawnMode, 4);
                },
                TravelMode::Position { pos, rot } => {
                    obj.set(Player::Pos, pos);
                    obj.set(Player::Rot, rot);
                    obj.set(Player::SpawnMode, 3);
                },
                TravelMode::EntryPoint => {
                    obj.set(Player::Pos, (0u32, entrypoint_pos));
                    obj.set(Player::Rot, entrypoint_rot);
                    obj.set(Player::SpawnMode, 3);
                },
            }
        }

        // Snap to floor
        {
            let navmesh = entity.resource::<Navmesh>();
            let collision_extent: Vec3 = *obj.get::<_, Vec3>(Player::CollisionExtent).unwrap();

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
    }

    fn begin_loading_sequence(entity: EntityWorldMut<'_>) {
        let avatar = entity.get::<Avatar>().unwrap();
        let controller = entity.get::<PlayerController>().unwrap();
        let movement = entity.get::<Movement>().unwrap();
        let obj = entity.get::<GameObjectData>().unwrap();
        let time = entity.resource::<Time<Virtual>>();

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