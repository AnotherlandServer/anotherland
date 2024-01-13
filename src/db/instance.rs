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

use std::io;
use std::ops::DerefMut;

use async_trait::async_trait;
use bson::{Document, doc};
use log::{warn, debug};
use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::{ParamClassContainer, NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, AnyClass, SpawnNodeParam, BoundParamClass, ParamEntity, ParamError, ParamClass, InteractObject, PatrolNodeParam, SpawnerParam, InteractObjectParam, EdnaContainerParam, ShipParam, MinigameInfoParam, MinigameScoreBoardParam, ChessPieceParam, NonSpawnPlacementParam, ServerGatewayExitPhaseParam, PresetPointParam, DoorParam, MyLandSettingsParam, ServerGatewayParam, QuestBeaconParam, OtherlandStructureParam, PlanetParam, ChessMetaGameLogicParam, MypadRoomDoorParam, BilliardBallParam, CustomTriggerParam, WorldDisplayParam, Uuid, CtfGameFlagParam};
use atlas::NonClientBase;
use crate::util::{AnotherlandResult, AnotherlandError};

use super::{DatabaseRecord, Content, SpawnerContent, NpcContent, StructureContent};

#[derive(Serialize, Deserialize)]
pub struct RawInstance {
    pub id: i64,
    pub guid: Uuid,
    pub zone_guid: Uuid,
    pub class: i64,
    pub content_guid: Uuid,
    pub editor_name: String,
    pub data: Option<ParamClassContainer>,
    pub phase_tag: String,
}

#[async_trait]
impl DatabaseRecord<'_> for RawInstance {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("instances")
    }

    fn query_one(key: &Self::Key) -> Document {
        doc!{ "guid": { "$eq": bson::to_bson(key).unwrap() } }
    }

    fn key(&self) -> &Self::Key {
        &self.guid
    }
}

pub enum Instance {
    Spawner { guid: Uuid, content_guid: Uuid, name: String, data: SpawnerParam, phase_tag: String, content: SpawnerContent }, // 44
    Npc { guid: Uuid, content_guid: Uuid, name: String, data: NpcOtherlandParam, phase_tag: String, content: NpcContent }, // 47
    Structure { guid: Uuid, content_guid: Uuid, name: String, data: StructureParam, phase_tag: String, content: StructureContent }, // 55
    Portal { guid: Uuid, content_guid: Uuid, name: String, data: PortalParam, phase_tag: String, content: StructureContent }, // 56
    StartingPoint { guid: Uuid, content_guid: Uuid, name: String, data: StartingPointParam, phase_tag: String, content: StructureContent }, // 57
    Trigger { guid: Uuid, content_guid: Uuid, name: String, data: TriggerParam, phase_tag: String, content: StructureContent }, // 61
    ChessPiece { guid: Uuid, content_guid: Uuid, name: String, data: ChessPieceParam, phase_tag: String, content: StructureContent }, // 62
    Ship { guid: Uuid, content_guid: Uuid, name: String, data: ShipParam, phase_tag: String, content: StructureContent }, // 66
    Planet { guid: Uuid, content_guid: Uuid, name: String, data: PlanetParam, phase_tag: String, content: StructureContent }, // 67
    InteractObject  { guid: Uuid, content_guid: Uuid, name: String, data: InteractObjectParam, phase_tag: String, content: StructureContent }, // 68
    PatrolNode { guid: Uuid, content_guid: Uuid, name: String, data: PatrolNodeParam, phase_tag: String, content: StructureContent }, // 70
    SpawnNode { guid: Uuid, content_guid: Uuid, name: String, data: SpawnNodeParam, phase_tag: String, content: StructureContent }, // 77
    MinigameInfo { guid: Uuid, content_guid: Uuid, name: String, data: MinigameInfoParam, phase_tag: String, content: StructureContent }, // 104
    ChessMetaGameLogic { guid: Uuid, content_guid: Uuid, name: String, data: ChessMetaGameLogicParam, phase_tag: String, content: StructureContent }, // 105
    EDNAContainer { guid: Uuid, content_guid: Uuid, name: String, data: EdnaContainerParam, phase_tag: String, content: StructureContent }, // 109
    BilliardBall { guid: Uuid, content_guid: Uuid, name: String, data: BilliardBallParam, phase_tag: String, content: StructureContent }, // 114
    OtherlandStructure { guid: Uuid, content_guid: Uuid, name: String, data: OtherlandStructureParam, phase_tag: String, content: StructureContent }, // 121
    MinigameScoreBoard { guid: Uuid, content_guid: Uuid, name: String, data: MinigameScoreBoardParam, phase_tag: String, content: StructureContent }, // 122
    PresetPoint { guid: Uuid, content_guid: Uuid, name: String, data: PresetPointParam, phase_tag: String, content: StructureContent }, // 124
    Door { guid: Uuid, content_guid: Uuid, name: String, data: DoorParam, phase_tag: String, content: StructureContent }, // 127
    CTFGameFlag { guid: Uuid, content_guid: Uuid, name: String, data: CtfGameFlagParam, phase_tag: String, content: StructureContent }, // 128
    ServerGateway { guid: Uuid, content_guid: Uuid, name: String, data: ServerGatewayParam, phase_tag: String, content: StructureContent }, // 129
    ServerGatewayExitPhase { guid: Uuid, content_guid: Uuid, name: String, data: ServerGatewayExitPhaseParam, phase_tag: String, content: StructureContent }, // 130
    NonSpawnPlacement { guid: Uuid, content_guid: Uuid, name: String, data: NonSpawnPlacementParam, phase_tag: String, content: StructureContent }, // 132
    MyLandSettings { guid: Uuid, content_guid: Uuid, name: String, data: MyLandSettingsParam, phase_tag: String, content: StructureContent }, // 135
    WorldDisplay { guid: Uuid, content_guid: Uuid, name: String, data: WorldDisplayParam, phase_tag: String, content: StructureContent }, // 146
    MypadRoomDoor { guid: Uuid, content_guid: Uuid, name: String, data: MypadRoomDoorParam, phase_tag: String, content: StructureContent }, // 154
    QuestBeacon { guid: Uuid, content_guid: Uuid, name: String, data: QuestBeaconParam, phase_tag: String, content: StructureContent }, // 178
    CustomTrigger { guid: Uuid, content_guid: Uuid, name: String, data: CustomTriggerParam, phase_tag: String, content: StructureContent }, // 192
}

impl Instance {
    pub async fn load_for_zone(db: Database, zone: &Uuid) -> AnotherlandResult<Vec<Instance>> {
        let mut rows = Vec::new();

        let mut result = RawInstance::collection(db.clone()).find(doc!{"zone_guid": {"$eq": zone}}, None).await?;
        while let Some(row) = result.try_next().await? {
            let content_guid = row.content_guid.clone();

            if let Some(instance) = Instance::from_raw_instance(db.clone(), row).await? {
                rows.push(instance);
            } else {
                warn!("Content {} not found", content_guid);
            }
        }

        Ok(rows)
    }

    pub fn guid(&self) -> &Uuid {
        match self {
            Self::Spawner { guid, .. } => guid,
            Self::Npc { guid, .. } => guid,
            Self::Structure { guid, .. } => guid,
            Self::Portal { guid, .. } => guid,
            Self::StartingPoint { guid, .. } => guid,
            Self::Trigger { guid, .. } => guid,
            Self::ChessPiece { guid, .. } => guid,
            Self::Ship { guid, .. } => guid,
            Self::Planet { guid, .. } => guid,
            Self::InteractObject { guid, .. } => guid,
            Self::PatrolNode { guid, .. } => guid,
            Self::SpawnNode { guid, .. } => guid,
            Self::MinigameInfo { guid, .. } => guid,
            Self::ChessMetaGameLogic { guid, .. } => guid,
            Self::EDNAContainer { guid, .. } => guid,
            Self::BilliardBall { guid, .. } => guid,
            Self::OtherlandStructure { guid, .. } => guid,
            Self::MinigameScoreBoard { guid, .. } => guid,
            Self::PresetPoint { guid, .. } => guid,
            Self::Door { guid, .. } => guid,
            Self::CTFGameFlag { guid, .. } => guid,
            Self::ServerGateway { guid, .. } => guid,
            Self::ServerGatewayExitPhase { guid, .. } => guid,
            Self::NonSpawnPlacement { guid, .. } => guid,
            Self::MyLandSettings { guid, .. } => guid,
            Self::WorldDisplay { guid, .. } => guid,
            Self::MypadRoomDoor { guid, .. } => guid,
            Self::QuestBeacon { guid, .. } => guid,
            Self::CustomTrigger { guid, .. } => guid,
        }
    }

    pub fn content_guid(&self) -> &Uuid {
        match self {
            Self::Spawner { content_guid, .. } => content_guid,
            Self::Npc { content_guid, .. } => content_guid,
            Self::Structure { content_guid, .. } => content_guid,
            Self::Portal { content_guid, .. } => content_guid,
            Self::StartingPoint { content_guid, .. } => content_guid,
            Self::Trigger { content_guid, .. } => content_guid,
            Self::ChessPiece { content_guid, .. } => content_guid,
            Self::Ship { content_guid, .. } => content_guid,
            Self::Planet { content_guid, .. } => content_guid,
            Self::InteractObject { content_guid, .. } =>content_guid,
            Self::PatrolNode { content_guid, .. } => content_guid,
            Self::SpawnNode { content_guid, .. } => content_guid,
            Self::MinigameInfo { content_guid, .. } => content_guid,
            Self::ChessMetaGameLogic { content_guid, .. } => content_guid,
            Self::EDNAContainer { content_guid, .. } => content_guid,
            Self::BilliardBall { content_guid, .. } => content_guid,
            Self::OtherlandStructure { content_guid, .. } => content_guid,
            Self::MinigameScoreBoard { content_guid, .. } => content_guid,
            Self::PresetPoint { content_guid, .. } => content_guid,
            Self::Door { content_guid, .. } => content_guid,
            Self::CTFGameFlag { content_guid, .. } => content_guid,
            Self::ServerGateway { content_guid, .. } => content_guid,
            Self::ServerGatewayExitPhase { content_guid, .. } => content_guid,
            Self::NonSpawnPlacement { content_guid, .. } => content_guid,
            Self::MyLandSettings { content_guid, .. } => content_guid,
            Self::WorldDisplay { content_guid, .. } => content_guid,
            Self::MypadRoomDoor { content_guid, .. } => content_guid,
            Self::QuestBeacon { content_guid, .. } => content_guid,
            Self::CustomTrigger { content_guid, .. } => content_guid,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Spawner { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Npc { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Structure { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Portal { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::StartingPoint { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Trigger { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::ChessPiece { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Ship { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Planet { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::InteractObject { data, .. } =>*data.enable_in_game().unwrap_or(&true),
            Self::PatrolNode { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::SpawnNode { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::MinigameInfo { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::ChessMetaGameLogic { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::EDNAContainer { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::BilliardBall { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::OtherlandStructure { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::MinigameScoreBoard { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::PresetPoint { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::Door { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::CTFGameFlag { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::ServerGateway { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::ServerGatewayExitPhase { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::NonSpawnPlacement { .. } => true,
            Self::MyLandSettings { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::WorldDisplay { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::MypadRoomDoor { data, .. } => *data.enable_in_game().unwrap_or(&true),
            Self::QuestBeacon { .. } => true,
            Self::CustomTrigger { data, .. } => *data.enable_in_game().unwrap_or(&true),
        }
    }

    pub async fn load_content<'a, T1, T2>(&'a self, db: Database) -> AnotherlandResult<T1> 
        where 
            T1: DatabaseRecord<'a, Key = bson::Uuid> + DerefMut<Target = Content>,
            T2: BoundParamClass + ParamEntity,
            T2: TryFrom<ParamClassContainer, Error = ParamError>
    {
        if let Some(mut content) = T1::get(db.clone(), &self.content_guid()).await? {
            if let Some(class) = content.data.as_mut() {
                class.as_anyclass_mut().apply(self.data_as_anyclass().clone());
            }

            Ok(content.into())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("{} {} not found in content db", std::any::type_name::<T1>(), self.content_guid())
            ).into())
        }
    }

    fn data_as_anyclass(&self) -> &AnyClass {
        match self {
            Self::Spawner { data, .. } => data.as_anyclass(),
            Self::Npc { data, .. } => data.as_anyclass(),
            Self::Structure { data, .. } => data.as_anyclass(),
            Self::Portal { data, .. } => data.as_anyclass(),
            Self::StartingPoint { data, .. } => data.as_anyclass(),
            Self::Trigger { data, .. } => data.as_anyclass(),
            Self::ChessPiece { data, .. } => data.as_anyclass(),
            Self::Ship { data, .. } => data.as_anyclass(),
            Self::Planet { data, .. } => data.as_anyclass(),
            Self::InteractObject { data, .. } => data.as_anyclass(),
            Self::PatrolNode { data, .. } => data.as_anyclass(),
            Self::SpawnNode { data, .. } => data.as_anyclass(),
            Self::MinigameInfo { data, .. } => data.as_anyclass(),
            Self::ChessMetaGameLogic { data, .. } => data.as_anyclass(),
            Self::EDNAContainer { data, .. } => data.as_anyclass(),
            Self::BilliardBall { data, .. } => data.as_anyclass(),
            Self::OtherlandStructure { data, .. } => data.as_anyclass(),
            Self::MinigameScoreBoard { data, .. } => data.as_anyclass(),
            Self::PresetPoint { data, .. } => data.as_anyclass(),
            Self::Door { data, .. } => data.as_anyclass(),
            Self::CTFGameFlag { data, .. } => data.as_anyclass(),
            Self::ServerGateway { data, .. } => data.as_anyclass(),
            Self::ServerGatewayExitPhase { data, .. } => data.as_anyclass(),
            Self::NonSpawnPlacement { data, .. } => data.as_anyclass(),
            Self::MyLandSettings { data, .. } => data.as_anyclass(),
            Self::WorldDisplay { data, .. } => data.as_anyclass(),
            Self::MypadRoomDoor { data, .. } => data.as_anyclass(),
            Self::QuestBeacon { data, .. } => data.as_anyclass(),
            Self::CustomTrigger { data, .. } => data.as_anyclass(),
        }
    }

    async fn from_raw_instance(db: Database, value: RawInstance) -> AnotherlandResult<Option<Self>> {
        Ok(match value.class {
            44 => 
                if let Some(content) = SpawnerContent::get(db, &value.content_guid).await? {
                    Some(Instance::Spawner { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(SpawnerParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            47 => 
                if let Some(content) = NpcContent::get(db, &value.content_guid).await? {
                    Some(Instance::Npc { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(NpcOtherlandParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            55 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Structure { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(StructureParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            56 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Portal { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(PortalParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            57 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::StartingPoint { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(StartingPointParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            61 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Trigger { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(TriggerParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            62 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::ChessPiece { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(ChessPieceParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            66 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Ship { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(ShipParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            67 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Planet { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(PlanetParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            68 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::InteractObject { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(InteractObjectParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            70 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::PatrolNode { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(PatrolNodeParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            71 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::SpawnNode { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(SpawnNodeParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            104 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::MinigameInfo { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(MinigameInfoParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            105 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::ChessMetaGameLogic { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(ChessMetaGameLogicParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            109 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::EDNAContainer { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(EdnaContainerParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            114 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::BilliardBall { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(BilliardBallParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            121 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::OtherlandStructure { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(OtherlandStructureParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            122 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::MinigameScoreBoard { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(MinigameScoreBoardParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            124 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::PresetPoint { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(PresetPointParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            127 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::Door { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(DoorParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            128 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::CTFGameFlag { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(CtfGameFlagParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            129 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::ServerGateway { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(ServerGatewayParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            130 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::ServerGatewayExitPhase { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(ServerGatewayExitPhaseParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            132 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::NonSpawnPlacement { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(NonSpawnPlacementParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            135 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::MyLandSettings { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(MyLandSettingsParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            136 => {
                debug!("Class 136 (LocalTacticNode) not implemented!");
                None
            },
            146 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::WorldDisplay { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(WorldDisplayParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            154 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::MypadRoomDoor { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(MypadRoomDoorParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            178 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::QuestBeacon { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(QuestBeaconParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            192 => 
                if let Some(content) = StructureContent::get(db, &value.content_guid).await? {
                    Some(Instance::CustomTrigger { 
                        guid: value.guid, 
                        content_guid: value.content_guid.clone(), 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.try_into()
                            .expect("unexpected param class")
                        ).unwrap_or(CustomTriggerParam::new()), 
                        phase_tag: value.phase_tag,
                        content,
                    })
                } else {
                    None
                },
            _ => todo!("loader for content class {}", value.class),
        })
    }
}
