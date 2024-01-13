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

use atlas::{SpawnerParam, ParamClass, NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, ChessPieceParam, ShipParam, PlanetParam, InteractObjectParam, PatrolNodeParam, SpawnNodeParam, MinigameInfoParam, ChessMetaGameLogicParam, EdnaContainerParam, OtherlandStructureParam, MinigameScoreBoardParam, PresetPointParam, DoorParam, ServerGatewayParam, ServerGatewayExitPhaseParam, NonSpawnPlacementParam, MyLandSettingsParam, QuestBeaconParam, ParamEntity, MypadRoomDoorParam, BilliardBallParam, WorldDisplayParam, CustomTriggerParam, CtfGameFlagParam};
use log::{warn, debug};

use crate::{util::AnotherlandResult, db::{Instance, SpawnerContent, NpcContent, StructureContent}};

use super::{Zone, components::EntityType};

impl Zone {
    pub(super) async fn load_content(&mut self) -> AnotherlandResult<()> {
        let factory = self.factory.clone();

        for (instance, id) in factory.instances() {
            if instance.is_enabled() {
                let id = match &instance {
                    Instance::Spawner { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<SpawnerParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Spawner, name, phase_tag, params.to_entity())
                    },
                    Instance::Npc { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<NpcOtherlandParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::NpcOtherland, name, phase_tag, params.to_entity())
                    },
                    Instance::Structure { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<StructureParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Structure, name, phase_tag, params.to_entity())
                    },
                    Instance::Portal { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<PortalParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Portal, name, phase_tag, params.to_entity())
                    },
                    Instance::StartingPoint { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<StartingPointParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::StartingPoint, name, phase_tag, params.to_entity())
                    },
                    Instance::Trigger { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<TriggerParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Trigger, name, phase_tag, params.to_entity())
                    },
                    Instance::ChessPiece { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<ChessPieceParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessPiece, name, phase_tag, params.to_entity())
                    },
                    Instance::Ship { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<ShipParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Ship, name, phase_tag, params.to_entity())
                    },
                    Instance::Planet { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<PlanetParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Planet, name, phase_tag, params.to_entity())
                    },
                    Instance::InteractObject { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<InteractObjectParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::InteractObject, name, phase_tag, params.to_entity())
                    },
                    Instance::PatrolNode { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<PatrolNodeParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::PatrolNode, name, phase_tag, params.to_entity())
                    },
                    Instance::SpawnNode { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<SpawnNodeParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::SpawnNode, name, phase_tag, params.to_entity())
                    },
                    Instance::MinigameInfo { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<MinigameInfoParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameInfo, name, phase_tag, params.to_entity())
                    },
                    Instance::ChessMetaGameLogic { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<ChessMetaGameLogicParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::ChessMetaGameLogic, name, phase_tag, params.to_entity())
                    },
                    Instance::EDNAContainer { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<EdnaContainerParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::EDNAContainer, name, phase_tag, params.to_entity())
                    },
                    Instance::BilliardBall { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<BilliardBallParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::BilliardBall, name, phase_tag, params.to_entity())
                    },
                    Instance::OtherlandStructure { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<OtherlandStructureParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::OtherlandStructure, name, phase_tag, params.to_entity())
                    },
                    Instance::MinigameScoreBoard { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<MinigameScoreBoardParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::MinigameScoreBoard, name, phase_tag, params.to_entity())
                    },
                    Instance::PresetPoint { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<PresetPointParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::PresetPoint, name, phase_tag, params.to_entity())
                    },
                    Instance::Door { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<DoorParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::Door, name, phase_tag, params.to_entity())
                    },
                    Instance::CTFGameFlag { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<CtfGameFlagParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::CTFGameFlag, name, phase_tag, params.to_entity())
                    },
                    Instance::ServerGateway { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<ServerGatewayParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGateway, name, phase_tag, params.to_entity())
                    },
                    Instance::ServerGatewayExitPhase { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<ServerGatewayExitPhaseParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::ServerGatewayExitPhase, name, phase_tag, params.to_entity())
                    },
                    Instance::NonSpawnPlacement { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<NonSpawnPlacementParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::NonSpawnPlacement, name, phase_tag, params.to_entity())
                    },
                    Instance::MyLandSettings { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<MyLandSettingsParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::MyLandSettings, name, phase_tag, params.to_entity())
                    },
                    Instance::WorldDisplay { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<WorldDisplayParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::WorldDisplay, name, phase_tag, params.to_entity())
                    },
                    Instance::MypadRoomDoor { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<MypadRoomDoorParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::MypadRoomDoor, name, phase_tag, params.to_entity())
                    },
                    Instance::QuestBeacon { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<QuestBeaconParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::QuestBeacon, name, phase_tag, params.to_entity())
                    },
                    Instance::CustomTrigger { name, data, phase_tag, content, .. } => {
                        let mut params = content.to_owned().into_param::<CustomTriggerParam>().unwrap();
                        params.apply(data.to_owned());
                        
                        self.spawn_non_player_avatar(id.to_owned(), EntityType::CustomTrigger, name, phase_tag, params.to_entity())
                    },
                };
            }
        }

        Ok(())
    }
}