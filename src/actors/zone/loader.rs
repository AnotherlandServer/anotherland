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

use atlas::{BilliardBallClass, ChessMetaGameLogicClass, ChessPieceClass, CtfGameFlagClass, CustomTriggerClass, DoorClass, EdnaContainerClass, InteractObjectClass, MinigameInfoClass, MinigameScoreBoardClass, MyLandSettingsClass, MypadRoomDoorClass, NonSpawnPlacementClass, NpcOtherlandClass, OtherlandStructureClass, ParamClass, PatrolNodeClass, PlanetClass, PortalClass, PresetPointClass, QuestBeaconClass, ServerGatewayClass, ServerGatewayExitPhaseClass, ShipClass, SpawnNodeClass, SpawnerClass, StartingPointClass, StructureClass, TriggerClass, WorldDisplayClass};

use crate::{util::AnotherlandResult, db::Instance};

use super::{Zone, components::EntityType};

impl Zone {
    pub(super) async fn load_content(&mut self) -> AnotherlandResult<()> {
        let factory = self.factory.clone();

        for (instance, id) in factory.instances() {
            match &instance {
                Instance::Spawner { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<SpawnerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Spawner, name, phase_tag, params);
                },
                Instance::Npc { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<NpcOtherlandClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::NpcOtherland, name, phase_tag, params);
                },
                Instance::Structure { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<StructureClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Structure, name, phase_tag, params);
                },
                Instance::Portal { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PortalClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Portal, name, phase_tag, params);
                },
                Instance::StartingPoint { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<StartingPointClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::StartingPoint, name, phase_tag, params);
                },
                Instance::Trigger { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<TriggerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Trigger, name, phase_tag, params);
                },
                Instance::ChessPiece { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ChessPieceClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessPiece, name, phase_tag, params);
                },
                Instance::Ship { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ShipClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Ship, name, phase_tag, params);
                },
                Instance::Planet { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PlanetClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Planet, name, phase_tag, params);
                },
                Instance::InteractObject { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<InteractObjectClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::InteractObject, name, phase_tag, params);
                },
                Instance::PatrolNode { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PatrolNodeClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::PatrolNode, name, phase_tag, params);
                },
                Instance::SpawnNode { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<SpawnNodeClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::SpawnNode, name, phase_tag, params);
                },
                Instance::MinigameInfo { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MinigameInfoClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameInfo, name, phase_tag, params);
                },
                Instance::ChessMetaGameLogic { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ChessMetaGameLogicClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessMetaGameLogic, name, phase_tag, params);
                },
                Instance::EDNAContainer { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<EdnaContainerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::EDNAContainer, name, phase_tag, params);
                },
                Instance::BilliardBall { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<BilliardBallClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::BilliardBall, name, phase_tag, params);
                },
                Instance::OtherlandStructure { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<OtherlandStructureClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::OtherlandStructure, name, phase_tag, params);
                },
                Instance::MinigameScoreBoard { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MinigameScoreBoardClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameScoreBoard, name, phase_tag, params);
                },
                Instance::PresetPoint { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<PresetPointClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::PresetPoint, name, phase_tag, params);
                },
                Instance::Door { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<DoorClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::Door, name, phase_tag, params);
                },
                Instance::CTFGameFlag { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<CtfGameFlagClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::CTFGameFlag, name, phase_tag, params);
                },
                Instance::ServerGateway { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ServerGatewayClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGateway, name, phase_tag, params);
                },
                Instance::ServerGatewayExitPhase { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<ServerGatewayExitPhaseClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGatewayExitPhase, name, phase_tag, params);
                },
                Instance::NonSpawnPlacement { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<NonSpawnPlacementClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::NonSpawnPlacement, name, phase_tag, params);
                },
                Instance::MyLandSettings { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MyLandSettingsClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MyLandSettings, name, phase_tag, params);
                },
                Instance::WorldDisplay { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<WorldDisplayClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::WorldDisplay, name, phase_tag, params);
                },
                Instance::MypadRoomDoor { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<MypadRoomDoorClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::MypadRoomDoor, name, phase_tag, params);
                },
                Instance::QuestBeacon { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<QuestBeaconClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::QuestBeacon, name, phase_tag, params);
                },
                Instance::CustomTrigger { name, data, phase_tag, content, .. } => {
                    let mut params = content.to_owned().into_param::<CustomTriggerClass>().unwrap();
                    params.apply(data.to_owned());
                    
                    self.spawn_non_player_avatar(id.to_owned(), EntityType::CustomTrigger, name, phase_tag, params);
                },
            }
        }

        Ok(())
    }
}