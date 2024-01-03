// Copyright (C) 2023 AnotherlandServer
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

use atlas::{SpawnerParam, ParamClass, NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, ChessPieceParam, ShipParam, PlanetParam, InteractObjectParam, PatrolNodeParam, SpawnNodeParam, MinigameInfoParam, ChessMetaGameLogicParam, EdnaContainerParam, OtherlandStructureParam, MinigameScoreBoardParam, PresetPointParam, DoorParam, ServerGatewayParam, ServerGatewayExitPhaseParam, NonSpawnPlacementParam, MyLandSettingsParam, QuestBeaconParam, ParamEntity, MypadRoomDoorParam, BilliardBallParam, WorldDisplayParam, CustomTriggerParam};
use log::{warn, debug};

use crate::{util::AnotherlandResult, db::{Instance, SpawnerContent, NpcContent, StructureContent}};

use super::{Zone, components::EntityType};

impl Zone {
    pub(super) async fn load_content(&mut self) -> AnotherlandResult<()> {
        let instances = Instance::load_for_zone(self.realm_db.clone(), &self.zone_def.guid).await?;
        for instance in instances.into_iter() {
            if instance.is_enabled() {
                let id = match &instance {
                    Instance::Spawner { name, data, phase_tag, .. } => {
                        match instance.load_content::<SpawnerContent, SpawnerParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<SpawnerParam>().unwrap();
                                params.apply(data.to_owned());
                                
                                self.spawn_non_player_avatar(EntityType::Spawner, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Spawner {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Npc { name, data, phase_tag, .. } => {
                        match instance.load_content::<NpcContent, NpcOtherlandParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<NpcOtherlandParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::NpcOtherland, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Npc {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Structure { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, StructureParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<StructureParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Structure, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Structure {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Portal { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, PortalParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<PortalParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Portal, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Portal {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::StartingPoint { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, StartingPointParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<StartingPointParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::StartingPoint, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("StartingPoint {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Trigger { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, TriggerParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<TriggerParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Trigger, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Trigger {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::ChessPiece { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, ChessPieceParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<ChessPieceParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::ChessPiece, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("ChessPiece {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Ship { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, ShipParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<ShipParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Ship, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Ship {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Planet { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, PlanetParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<PlanetParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Planet, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Planet {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::InteractObject { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, InteractObjectParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<InteractObjectParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::InteractObject, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("InteractObject {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::PatrolNode { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, PatrolNodeParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<PatrolNodeParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::PatrolNode, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("PatrolNode {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::SpawnNode { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, SpawnNodeParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<SpawnNodeParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::SpawnNode, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("SpawnNode {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::MinigameInfo { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, MinigameInfoParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<MinigameInfoParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::MinigameInfo, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("MinigameInfo {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::ChessMetaGameLogic { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, ChessMetaGameLogicParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<ChessMetaGameLogicParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::ChessMetaGameLogic, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("ChessMetaGameLogic {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::EDNAContainer { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, EdnaContainerParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<EdnaContainerParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::EDNAContainer, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("EDNAContainer {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::BilliardBall { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, BilliardBallParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<BilliardBallParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::BilliardBall, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("BilliardBall {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::OtherlandStructure { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, OtherlandStructureParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<OtherlandStructureParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::OtherlandStructure, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("OtherlandStructure {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::MinigameScoreBoard { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, MinigameScoreBoardParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<MinigameScoreBoardParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::MinigameScoreBoard, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("MinigameScoreBoard {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::PresetPoint { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, PresetPointParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<PresetPointParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::PresetPoint, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("PresetPoint {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::Door { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, DoorParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<DoorParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::Door, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("Door {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::ServerGateway { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, ServerGatewayParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<ServerGatewayParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::ServerGateway, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("ServerGateway {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::ServerGatewayExitPhase { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, ServerGatewayExitPhaseParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<ServerGatewayExitPhaseParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::ServerGatewayExitPhase, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("ServerGatewayExitPhase {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::NonSpawnPlacement { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, NonSpawnPlacementParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<NonSpawnPlacementParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::NonSpawnPlacement, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("NonSpawnPlacement {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::MyLandSettings { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, MyLandSettingsParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<MyLandSettingsParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::MyLandSettings, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("MyLandSettings {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::WorldDisplay { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, WorldDisplayParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<WorldDisplayParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::WorldDisplay, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("WorldDisplay {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::MypadRoomDoor { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, MypadRoomDoorParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<MypadRoomDoorParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::MypadRoomDoor, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("MypadRoomDoor {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::QuestBeacon { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, QuestBeaconParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<QuestBeaconParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::QuestBeacon, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("QuestBeacon {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                    Instance::CustomTrigger { name, data, phase_tag, .. } => {
                        match instance.load_content::<StructureContent, CustomTriggerParam>(self.realm_db.clone()).await {
                            Ok(content) => {
                                let mut params = content.into_param::<CustomTriggerParam>().unwrap();
                                params.apply(data.to_owned());

                                self.spawn_non_player_avatar(EntityType::CustomTrigger, name, phase_tag, params.to_entity())
                            },
                            Err(_) => {
                                warn!("CustomTrigger {} not found", instance.content_guid());
                                continue;
                            },
                        }
                    },
                };

                self.instance_template.insert(instance.guid().clone(), (instance, Some(id)));
            } else {
                self.instance_template.insert(instance.guid().clone(), (instance, None));
            }
        }

        Ok(())
    }
}