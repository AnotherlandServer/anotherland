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

use async_trait::async_trait;
use bson::{Document, doc};
use log::{warn, debug};

use mongodb::{Database, Collection};
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use atlas::{BilliardBallAttribute, ChessMetaGameLogicAttribute, ChessPieceAttribute, CtfGameFlagAttribute, CustomTriggerAttribute, DoorAttribute, EdnaContainerAttribute, InteractObjectAttribute, MinigameInfoAttribute, MinigameScoreBoardAttribute, MyLandSettingsAttribute, MypadRoomDoorAttribute, NonSpawnPlacementAttribute, NpcOtherlandAttribute, OtherlandStructureAttribute, ParamSet, ParamSetBox, PatrolNodeAttribute, PlanetAttribute, PortalAttribute, PresetPointAttribute, QuestBeaconAttribute, ServerGatewayAttribute, ServerGatewayExitPhaseAttribute, ShipAttribute, SpawnNodeAttribute, SpawnerAttribute, StartingPointAttribute, StructureAttribute, TriggerAttribute, Uuid, WorldDisplayAttribute};
use crate::util::AnotherlandResult;

use super::{DatabaseRecord, SpawnerContent, NpcContent, StructureContent};

#[derive(Serialize, Deserialize)]
pub struct RawInstance {
    pub id: i64,
    pub guid: Uuid,
    pub zone_guid: Uuid,
    pub class: i64,
    pub content_guid: Uuid,
    pub editor_name: String,
    pub data: Option<ParamSetBox>,
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
    Spawner { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<SpawnerAttribute>, phase_tag: String, content: SpawnerContent }, // 44
    Npc { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<NpcOtherlandAttribute>, phase_tag: String, content: NpcContent }, // 47
    Structure { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<StructureAttribute>, phase_tag: String, content: StructureContent }, // 55
    Portal { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<PortalAttribute>, phase_tag: String, content: StructureContent }, // 56
    StartingPoint { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<StartingPointAttribute>, phase_tag: String, content: StructureContent }, // 57
    Trigger { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<TriggerAttribute>, phase_tag: String, content: StructureContent }, // 61
    ChessPiece { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<ChessPieceAttribute>, phase_tag: String, content: StructureContent }, // 62
    Ship { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<ShipAttribute>, phase_tag: String, content: StructureContent }, // 66
    Planet { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<PlanetAttribute>, phase_tag: String, content: StructureContent }, // 67
    InteractObject  { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<InteractObjectAttribute>, phase_tag: String, content: StructureContent }, // 68
    PatrolNode { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<PatrolNodeAttribute>, phase_tag: String, content: StructureContent }, // 70
    SpawnNode { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<SpawnNodeAttribute>, phase_tag: String, content: StructureContent }, // 77
    MinigameInfo { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<MinigameInfoAttribute>, phase_tag: String, content: StructureContent }, // 104
    ChessMetaGameLogic { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<ChessMetaGameLogicAttribute>, phase_tag: String, content: StructureContent }, // 105
    EDNAContainer { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<EdnaContainerAttribute>, phase_tag: String, content: StructureContent }, // 109
    BilliardBall { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<BilliardBallAttribute>, phase_tag: String, content: StructureContent }, // 114
    OtherlandStructure { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<OtherlandStructureAttribute>, phase_tag: String, content: StructureContent }, // 121
    MinigameScoreBoard { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<MinigameScoreBoardAttribute>, phase_tag: String, content: StructureContent }, // 122
    PresetPoint { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<PresetPointAttribute>, phase_tag: String, content: StructureContent }, // 124
    Door { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<DoorAttribute>, phase_tag: String, content: StructureContent }, // 127
    CTFGameFlag { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<CtfGameFlagAttribute>, phase_tag: String, content: StructureContent }, // 128
    ServerGateway { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<ServerGatewayAttribute>, phase_tag: String, content: StructureContent }, // 129
    ServerGatewayExitPhase { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<ServerGatewayExitPhaseAttribute>, phase_tag: String, content: StructureContent }, // 130
    NonSpawnPlacement { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<NonSpawnPlacementAttribute>, phase_tag: String, content: StructureContent }, // 132
    MyLandSettings { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<MyLandSettingsAttribute>, phase_tag: String, content: StructureContent }, // 135
    WorldDisplay { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<WorldDisplayAttribute>, phase_tag: String, content: StructureContent }, // 146
    MypadRoomDoor { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<MypadRoomDoorAttribute>, phase_tag: String, content: StructureContent }, // 154
    QuestBeacon { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<QuestBeaconAttribute>, phase_tag: String, content: StructureContent }, // 178
    CustomTrigger { guid: Uuid, content_guid: Uuid, name: String, data: ParamSet<CustomTriggerAttribute>, phase_tag: String, content: StructureContent }, // 192
}

impl Instance {
    pub async fn load_for_zone(db: Database, zone: &Uuid) -> AnotherlandResult<Vec<Instance>> {
        let mut rows = Vec::new();

        let mut result = RawInstance::collection(db.clone()).find(doc!{"zone_guid": {"$eq": zone}}, None).await?;
        while let Some(row) = result.try_next().await? {
            let content_guid = row.content_guid;

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

    async fn from_raw_instance(db: Database, value: RawInstance) -> AnotherlandResult<Option<Self>> {
        Ok(match value.class {
            44 => 
                if let Some(content) = SpawnerContent::get(db, &value.content_guid).await? {
                    Some(Instance::Spawner { 
                        guid: value.guid, 
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
                        content_guid: value.content_guid, 
                        name: value.editor_name,
                        data: value.data.map(|v| 
                            v.take()
                            .expect("unexpected param class")
                        ).unwrap_or_default(), 
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
