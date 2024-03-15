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

use std::sync::Arc;

use atlas::{BilliardBallClass, ChessMetaGameLogicClass, ChessPieceClass, CtfGameFlagClass, CustomTriggerClass, DoorClass, EdnaContainerClass, InteractObjectClass, MinigameInfoClass, MinigameScoreBoardClass, MyLandSettingsClass, MypadRoomDoorClass, NonClientBaseParams, NonSpawnPlacementClass, NpcOtherlandClass, OtherlandStructureClass, ParamClass, PatrolNodeClass, PlanetClass, PortalAttribute, PortalClass, PortalParams, PresetPointClass, QuestBeaconClass, ServerGatewayClass, ServerGatewayExitPhaseClass, ShipClass, SpawnNodeClass, SpawnerClass, StartingPointClass, StructureClass, TriggerClass, Uuid, WorldDisplayClass, UUID_NIL};
use bevy::utils::hashbrown::HashMap;
use bson::{doc, Regex};
use futures::TryStreamExt;
use log::{debug, error};
use mongodb::options::FindOptions;

use crate::{actors::get_display_name, db::{self, realm_database, DatabaseRecord, DisplayName, FlightTube, Instance, RawInstance, StructureContent, WorldDef, ZoneDef}, util::AnotherlandResult};

use super::{components::EntityType, PortalExitPoint, PortalHiveDestination, PortalNodelink, Zone};

static MAP_WHITELIST: &[&str; 45] = &[
    "WhiteCity_P",
    "Newbie_P",
    "NewbieInst_P",
    "MyPadRooms_P",
    "MrJ_P",
    "MonasteryInst_P",
    "MonasteryInstBattle_P",
    "MLRedValley_P",
    "MarsDock_P",
    "LMPlatform_P",
    "LMInteriors_P",
    "LMInteriors02_P",
    "LimboNew_P",
    "LanternDistrict_P",
    "GameEntry_P",
    "Dungeon02_P",
    "Dungeon01_P",
    "CollectiveBar_P",
    "ClassTest01_P",
    "ClassSelection_P",
    "Challenges10_P",
    "Challenges08_P",
    "Challenges07_P",
    "Challenges06_P",
    "Challenges05_P",
    "Challenges04_P",
    "Challenges01_P",
    "BugWorldRuins_P",
    "BugWorldResearchComplex_P",
    "BugWorldJungle_P",
    "Bazaar_P",
    "BattlegroundOasis01_P",
    "BarClub101_P",
    "BarBlackRoom_P",
    "BadSector_P",
    "ArenaBad_P",
    "AeroWood_P",
    "AeroWater_P",
    "AeroWaterFirewall_P",
    "AeroShipBattle_P",
    "AeroFire_P",
    "AeroFireStart_P",
    "8Squared_P",
    "8SquaredRedCastle_P",
    "8SquaredKOTH01_P",
];

impl Zone {
    pub(super) async fn load_content_instances(&mut self) -> AnotherlandResult<()> {
        let factory = self.factory.clone();

        for (instance, id) in factory.instances() {
            match &instance {
                Instance::Spawner { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<SpawnerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Spawner, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Npc { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<NpcOtherlandClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::NpcOtherland, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Structure { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<StructureClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Structure, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Portal { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PortalClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    let nodelink = params.nodelink().map(|v| v.to_owned());
                    let exitpoint = params.exit_point().map(|v| v.to_owned());
                    let db = self.realm_db.clone();

                    let portal = self.spawn_non_player_avatar(id.to_owned(), EntityType::Portal, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                    
                    if let Some(nodelink) = nodelink {
                        if let Ok(instance_id) = Uuid::parse_str(&nodelink) &&
                            let Ok(Some(node)) = RawInstance::get(db, &instance_id).await 
                        {
                            self.app.world.get_entity_mut(portal).unwrap()
                                .insert(PortalNodelink::RemotePortal { 
                                    zone: node.zone_guid, 
                                    portal: node.guid,
                                });
                        } else {
                            error!("Failed to read node {}", nodelink)
                        }
                    }

                    if let Some(exitpoint) = exitpoint && let Ok(exitpoint) = Uuid::parse_str(&exitpoint)  {
                        self.app.world.get_entity_mut(portal).unwrap()
                            .insert(PortalExitPoint(exitpoint));
                    }
                },
                Instance::StartingPoint { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<StartingPointClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::StartingPoint, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Trigger { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<TriggerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Trigger, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::ChessPiece { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ChessPieceClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessPiece, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Ship { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ShipClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Ship, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Planet { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PlanetClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Planet, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::InteractObject { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<InteractObjectClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::InteractObject, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::PatrolNode { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PatrolNodeClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::PatrolNode, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::SpawnNode { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<SpawnNodeClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::SpawnNode, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::MinigameInfo { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MinigameInfoClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameInfo, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::ChessMetaGameLogic { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ChessMetaGameLogicClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessMetaGameLogic, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::EDNAContainer { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<EdnaContainerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::EDNAContainer, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::BilliardBall { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<BilliardBallClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::BilliardBall, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::OtherlandStructure { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<OtherlandStructureClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::OtherlandStructure, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::MinigameScoreBoard { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MinigameScoreBoardClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameScoreBoard, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::PresetPoint { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PresetPointClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::PresetPoint, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::Door { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<DoorClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Door, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::CTFGameFlag { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<CtfGameFlagClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::CTFGameFlag, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::ServerGateway { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ServerGatewayClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGateway, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::ServerGatewayExitPhase { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ServerGatewayExitPhaseClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGatewayExitPhase, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::NonSpawnPlacement { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<NonSpawnPlacementClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::NonSpawnPlacement, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::MyLandSettings { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MyLandSettingsClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MyLandSettings, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::WorldDisplay { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<WorldDisplayClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::WorldDisplay, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::MypadRoomDoor { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MypadRoomDoorClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MypadRoomDoor, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::QuestBeacon { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<QuestBeaconClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::QuestBeacon, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
                Instance::CustomTrigger { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<CustomTriggerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::CustomTrigger, name, phase_tag, instance.guid().to_owned(), content.guid, params);
                },
            }
        }

        Ok(())
    }

    pub(super) async fn load_flight_tubes() -> AnotherlandResult<HashMap<Uuid, Arc<FlightTube>>> {
        let tubes = db::FlightTube::list(realm_database().await).await?;
        let mut tube_map = HashMap::new();

        for tube in tubes {
            tube_map.insert(tube.id, Arc::new(tube));
        }

        Ok(tube_map)
    }

    pub(super) async fn load_portal_hive_destinations() -> AnotherlandResult<HashMap<String, PortalHiveDestination>> {
        let db = realm_database().await;
        let structures = StructureContent::collection(db.clone());
        let instances = RawInstance::collection(db.clone());

        let mut destinations = HashMap::new();

        // lookup all portal objects
        let re = Regex {
            pattern: "PortalHive".to_string(),
            options: "i".to_string(),
        };
        let mut result = structures.find(doc! {
            "data.portal.tags.v": {
                "$regex": re
            }
        }, FindOptions::builder()
            .sort(doc! {
                "editor_name": 1
            })
            .build()
        ).await?;

        while let Some(portal) = result.try_next().await? {
            // lookup all instances of the portal
            let mut result = instances.find(doc! {
                "content_guid": portal.guid
            }, None).await?;

            while let Some(instance) = result.try_next().await? {
                if let Some(instance_data) = instance.data {
                    let mut portal_data = portal.data.clone()
                        .and_then(|d| d.take::<PortalClass>().ok())
                        .unwrap();

                    portal_data.apply(instance_data.take::<PortalAttribute>()?);

                    /*let display_name = portal_data.as_set().get(&PortalAttribute::DisplayName)
                        .map(|v| *<&atlas::Param as TryInto<&Uuid>>::try_into(v).unwrap())
                        .unwrap_or(UUID_NIL);*/
                    if let Some(exit_point) = portal_data.exit_point() &&
                        !exit_point.is_empty() &&
                        let Some(zone) = ZoneDef::get(db.clone(), &instance.zone_guid).await? &&
                        let Some(world) = WorldDef::get_by_guid(db.clone(), &zone.worlddef_guid).await? 
                    {
                        if MAP_WHITELIST.contains(&world.name.as_str()) {
                            debug!("Found hive portal: {}-{} ({})", instance.zone_guid, instance.editor_name, *portal_data.display_name());

                            destinations.insert(instance.editor_name.clone(), PortalHiveDestination {
                                name: instance.editor_name,
                                world_name: world.name[..world.name.len()-2].to_owned(),
                                display_name: *portal_data.display_name(),
                                zone: instance.zone_guid,
                                link: PortalNodelink::RemotePortal { zone: instance.zone_guid, portal: instance.guid }
                            });
                        } else {
                            debug!("Ignoring portal from {}: {}-{} ({})", world.name.as_str(), instance.zone_guid, instance.editor_name, *portal_data.display_name());
                        }
                    };
                }
            }
        }

        Ok(destinations)
    }

    pub(super) async fn load_display_names() -> AnotherlandResult<HashMap<Uuid, String>> {
        let mut display_names = HashMap::new();

        let names = DisplayName::list(realm_database().await).await?.into_iter();
        for name in names {
            display_names.insert(name.id, name.name);
        }

        Ok(display_names)
    }
}
