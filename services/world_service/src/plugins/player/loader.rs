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
use bevy::{ecs::{error::Result, query::With, system::EntityCommands, world::EntityWorldMut}, math::{Quat, Vec3}};
use log::{error, warn};
use obj_params::{ContentRefList, GameObjectData, NonClientBase, Player, Portal, tags::{PortalTag, SpawnNodeTag, StartingPointTag}};
use protocol::{AbilityBarReference, PhysicsState, oaAbilityBarReferences, oaPlayerClassData};
use realm_api::{Character, RealmApi};
use toolkit::{OtherlandQuatExt, types::Uuid};

use crate::{instance::ZoneInstance, plugins::{Avatar, AvatarLoader, CombatStyle, ComponentLoaderCommandsTrait, ContentInfo, CooldownGroups, Factions, LoadContext, Movement, Navmesh, PlayerController, Skillbook, SkillbookParams}, proto::TravelMode};

impl AvatarLoader {
    pub async fn load_player_character(realm_api: RealmApi, character_id: Uuid) -> Result<Character> {
        let mut character = realm_api
            .get_character(&character_id).await?
            .ok_or(anyhow!("Character not found"))?;
        
        let ability_bar = realm_api
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

    pub fn on_load_player_character(&mut self, commands: &mut EntityCommands<'_>, _: &mut LoadContext, mut character: Character) -> Result<()> {
        commands
            .load_component::<Skillbook>(SkillbookParams {
                realm_api: self.realm_api.clone(),
                character_id: *character.id(),
                level: *character.data().get::<_, i32>(Player::Lvl).unwrap(),
                combat_style: CombatStyle::from_id(
                    *character.data().get::<_, i32>(Player::CombatStyle).unwrap()
                ),
            })
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

                let mut factions = Factions::default();

                for faction in character.data().get::<_, ContentRefList>(Player::Faction).unwrap().iter() {
                    factions.add_faction(faction.id);
                }

                entity.insert((
                    Avatar {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    cooldowns.create_cooldowns(),
                    factions,
                    movement,
                ));
            });
            
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
}