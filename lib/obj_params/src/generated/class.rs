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

// #################################################
// # This file is generated. Do not edit manually. #
// #################################################

use serde::Serialize;
use serde::Deserialize;
use std::str::FromStr;
use crate::Attribute;
use crate::AttributeInfo;
use crate::ParamSet;
use crate::ParamError;
use crate::Value;
use crate::GenericParamSet;
use crate::generated::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Class {
    LootSystem,
    Clan,
    CustomTrigger,
    QuestBeacon,
    MinigameMine,
    VehicleFlying,
    Player,
    MinigameItem,
    ItemMyLandTheme,
    MylandScoreboard,
    AiTemplate,
    CombatSystemGroup,
    CompinstDaemon,
    Spawner,
    MinigameInfo,
    WorldDisplay,
    OtherlandArea,
    Instance,
    Party,
    ClassItem,
    BuffBase,
    CompinstMulticlusterAvatarLookup,
    ClanRank,
    CooldownGroupExternal,
    Portal,
    ChessPiece,
    ItemPreset,
    EdnaContainer,
    ServerGateway,
    OtherlandStructure,
    CompinstClusternode,
    EdnaModule,
    MyLandSettings,
    MypadRoomDoor,
    OaBuff2,
    CompinstFrontendsrv,
    ItemBase,
    PortalItem,
    ServerGatewayExitPhase,
    ItemEdna,
    VehicleBase,
    BundleItem,
    ShopFilterSchema,
    Trigger,
    Trade,
    CompinstLoginsrv,
    EdnaFunction,
    CommonConfig,
    ClanMember,
    JsonSchema,
    SomaforgeItem,
    Planet,
    AbilityList,
    NonSpawnPlacementRadius,
    SpawnerBase,
    NonSpawnPlacement,
    CompinstMasterRedirectSrv,
    SteamDlc,
    CtfGameFlag,
    NpcOtherland,
    GameSession,
    BuffGroup,
    SkillGroup,
    StartingPoint,
    MoverBase,
    PresetPoint,
    PhysicsActor,
    CooldownGroup,
    BilliardBall,
    ChessMetaGameLogic,
    StandaloneLootPartition,
    Metagame,
    StructureBase,
    Faction,
    CompinstCommunicationsrv,
    CompinstClustersrv,
    Structure,
    EdnaAbility,
    OaCommonConfig,
    SteamItem,
    SpawnNode,
    NonClientBase,
    InteractObject,
    NpcBase,
    EdnaReceptor,
    CompinstClusterapp,
    OaZoneConfig,
    Version,
    Ship,
    PatrolNode,
    LifeDirector,
    Compinst,
    CompinstCommunitysrv,
    Config,
    LocalTacticNode,
    NpcShopConfig,
    LootScatterContainer,
    MinigameScoreBoard,
    Door,
    Mail,
    EdnaBase,
}
impl Class {
    pub fn get_attribute(&self, attr: &str) -> Option<&'static dyn AttributeInfo> {
        match self {
            Self::LootSystem => LOOT_SYSTEM_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::Clan => CLAN_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CustomTrigger => {
                CUSTOM_TRIGGER_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::QuestBeacon => {
                QUEST_BEACON_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::MinigameMine => {
                MINIGAME_MINE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::VehicleFlying => {
                VEHICLE_FLYING_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Player => PLAYER_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::MinigameItem => {
                MINIGAME_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ItemMyLandTheme => {
                ITEM_MY_LAND_THEME_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::MylandScoreboard => {
                MYLAND_SCOREBOARD_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::AiTemplate => AI_TEMPLATE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CombatSystemGroup => {
                COMBAT_SYSTEM_GROUP_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CompinstDaemon => {
                COMPINST_DAEMON_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Spawner => SPAWNER_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::MinigameInfo => {
                MINIGAME_INFO_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::WorldDisplay => {
                WORLD_DISPLAY_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::OtherlandArea => {
                OTHERLAND_AREA_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Instance => INSTANCE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::Party => PARTY_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::ClassItem => CLASS_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::BuffBase => BUFF_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CompinstMulticlusterAvatarLookup => {
                COMPINST_MULTICLUSTER_AVATAR_LOOKUP_ATTRIBUTES
                    .get(attr)
                    .map(|a| a.static_info())
            }
            Self::ClanRank => CLAN_RANK_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CooldownGroupExternal => {
                COOLDOWN_GROUP_EXTERNAL_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Portal => PORTAL_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::ChessPiece => CHESS_PIECE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::ItemPreset => ITEM_PRESET_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::EdnaContainer => {
                EDNA_CONTAINER_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ServerGateway => {
                SERVER_GATEWAY_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::OtherlandStructure => {
                OTHERLAND_STRUCTURE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CompinstClusternode => {
                COMPINST_CLUSTERNODE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::EdnaModule => EDNA_MODULE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::MyLandSettings => {
                MY_LAND_SETTINGS_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::MypadRoomDoor => {
                MYPAD_ROOM_DOOR_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::OaBuff2 => OA_BUFF_2_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CompinstFrontendsrv => {
                COMPINST_FRONTENDSRV_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ItemBase => ITEM_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::PortalItem => PORTAL_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::ServerGatewayExitPhase => {
                SERVER_GATEWAY_EXIT_PHASE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ItemEdna => ITEM_EDNA_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::VehicleBase => {
                VEHICLE_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::BundleItem => BUNDLE_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::ShopFilterSchema => {
                SHOP_FILTER_SCHEMA_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Trigger => TRIGGER_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::Trade => TRADE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CompinstLoginsrv => {
                COMPINST_LOGINSRV_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::EdnaFunction => {
                EDNA_FUNCTION_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CommonConfig => {
                COMMON_CONFIG_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ClanMember => CLAN_MEMBER_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::JsonSchema => JSON_SCHEMA_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::SomaforgeItem => {
                SOMAFORGE_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Planet => PLANET_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::AbilityList => {
                ABILITY_LIST_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::NonSpawnPlacementRadius => {
                NON_SPAWN_PLACEMENT_RADIUS_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::SpawnerBase => {
                SPAWNER_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::NonSpawnPlacement => {
                NON_SPAWN_PLACEMENT_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CompinstMasterRedirectSrv => {
                COMPINST_MASTER_REDIRECT_SRV_ATTRIBUTES
                    .get(attr)
                    .map(|a| a.static_info())
            }
            Self::SteamDlc => STEAM_DLC_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CtfGameFlag => {
                CTF_GAME_FLAG_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::NpcOtherland => {
                NPC_OTHERLAND_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::GameSession => {
                GAME_SESSION_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::BuffGroup => BUFF_GROUP_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::SkillGroup => SKILL_GROUP_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::StartingPoint => {
                STARTING_POINT_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::MoverBase => MOVER_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::PresetPoint => {
                PRESET_POINT_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::PhysicsActor => {
                PHYSICS_ACTOR_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CooldownGroup => {
                COOLDOWN_GROUP_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::BilliardBall => {
                BILLIARD_BALL_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::ChessMetaGameLogic => {
                CHESS_META_GAME_LOGIC_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::StandaloneLootPartition => {
                STANDALONE_LOOT_PARTITION_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Metagame => METAGAME_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::StructureBase => {
                STRUCTURE_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Faction => FACTION_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CompinstCommunicationsrv => {
                COMPINST_COMMUNICATIONSRV_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CompinstClustersrv => {
                COMPINST_CLUSTERSRV_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Structure => STRUCTURE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::EdnaAbility => {
                EDNA_ABILITY_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::OaCommonConfig => {
                OA_COMMON_CONFIG_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::SteamItem => STEAM_ITEM_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::SpawnNode => SPAWN_NODE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::NonClientBase => {
                NON_CLIENT_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::InteractObject => {
                INTERACT_OBJECT_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::NpcBase => NPC_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::EdnaReceptor => {
                EDNA_RECEPTOR_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::CompinstClusterapp => {
                COMPINST_CLUSTERAPP_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::OaZoneConfig => {
                OA_ZONE_CONFIG_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Version => VERSION_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::Ship => SHIP_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::PatrolNode => PATROL_NODE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::LifeDirector => {
                LIFE_DIRECTOR_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Compinst => COMPINST_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::CompinstCommunitysrv => {
                COMPINST_COMMUNITYSRV_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Config => CONFIG_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::LocalTacticNode => {
                LOCAL_TACTIC_NODE_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::NpcShopConfig => {
                NPC_SHOP_CONFIG_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::LootScatterContainer => {
                LOOT_SCATTER_CONTAINER_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::MinigameScoreBoard => {
                MINIGAME_SCORE_BOARD_ATTRIBUTES.get(attr).map(|a| a.static_info())
            }
            Self::Door => DOOR_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::Mail => MAIL_ATTRIBUTES.get(attr).map(|a| a.static_info()),
            Self::EdnaBase => EDNA_BASE_ATTRIBUTES.get(attr).map(|a| a.static_info()),
        }
    }
    pub fn get_attribute_from_id(
        &self,
        attr: u16,
    ) -> Option<&'static dyn AttributeInfo> {
        match self {
            Self::LootSystem => {
                LOOT_SYSTEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Clan => CLAN_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CustomTrigger => {
                CUSTOM_TRIGGER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::QuestBeacon => {
                QUEST_BEACON_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MinigameMine => {
                MINIGAME_MINE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::VehicleFlying => {
                VEHICLE_FLYING_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Player => PLAYER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::MinigameItem => {
                MINIGAME_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ItemMyLandTheme => {
                ITEM_MY_LAND_THEME_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MylandScoreboard => {
                MYLAND_SCOREBOARD_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::AiTemplate => {
                AI_TEMPLATE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CombatSystemGroup => {
                COMBAT_SYSTEM_GROUP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CompinstDaemon => {
                COMPINST_DAEMON_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Spawner => SPAWNER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::MinigameInfo => {
                MINIGAME_INFO_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::WorldDisplay => {
                WORLD_DISPLAY_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::OtherlandArea => {
                OTHERLAND_AREA_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Instance => INSTANCE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::Party => PARTY_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::ClassItem => {
                CLASS_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::BuffBase => BUFF_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CompinstMulticlusterAvatarLookup => {
                COMPINST_MULTICLUSTER_AVATAR_LOOKUP_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::ClanRank => CLAN_RANK_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CooldownGroupExternal => {
                COOLDOWN_GROUP_EXTERNAL_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Portal => PORTAL_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::ChessPiece => {
                CHESS_PIECE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ItemPreset => {
                ITEM_PRESET_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::EdnaContainer => {
                EDNA_CONTAINER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ServerGateway => {
                SERVER_GATEWAY_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::OtherlandStructure => {
                OTHERLAND_STRUCTURE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CompinstClusternode => {
                COMPINST_CLUSTERNODE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::EdnaModule => {
                EDNA_MODULE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MyLandSettings => {
                MY_LAND_SETTINGS_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MypadRoomDoor => {
                MYPAD_ROOM_DOOR_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::OaBuff2 => OA_BUFF_2_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CompinstFrontendsrv => {
                COMPINST_FRONTENDSRV_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ItemBase => ITEM_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::PortalItem => {
                PORTAL_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ServerGatewayExitPhase => {
                SERVER_GATEWAY_EXIT_PHASE_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::ItemEdna => ITEM_EDNA_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::VehicleBase => {
                VEHICLE_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::BundleItem => {
                BUNDLE_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ShopFilterSchema => {
                SHOP_FILTER_SCHEMA_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Trigger => TRIGGER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::Trade => TRADE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CompinstLoginsrv => {
                COMPINST_LOGINSRV_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::EdnaFunction => {
                EDNA_FUNCTION_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CommonConfig => {
                COMMON_CONFIG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ClanMember => {
                CLAN_MEMBER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::JsonSchema => {
                JSON_SCHEMA_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::SomaforgeItem => {
                SOMAFORGE_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Planet => PLANET_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::AbilityList => {
                ABILITY_LIST_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NonSpawnPlacementRadius => {
                NON_SPAWN_PLACEMENT_RADIUS_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::SpawnerBase => {
                SPAWNER_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NonSpawnPlacement => {
                NON_SPAWN_PLACEMENT_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CompinstMasterRedirectSrv => {
                COMPINST_MASTER_REDIRECT_SRV_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::SteamDlc => STEAM_DLC_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CtfGameFlag => {
                CTF_GAME_FLAG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NpcOtherland => {
                NPC_OTHERLAND_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::GameSession => {
                GAME_SESSION_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::BuffGroup => {
                BUFF_GROUP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::SkillGroup => {
                SKILL_GROUP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::StartingPoint => {
                STARTING_POINT_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MoverBase => {
                MOVER_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::PresetPoint => {
                PRESET_POINT_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::PhysicsActor => {
                PHYSICS_ACTOR_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CooldownGroup => {
                COOLDOWN_GROUP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::BilliardBall => {
                BILLIARD_BALL_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::ChessMetaGameLogic => {
                CHESS_META_GAME_LOGIC_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::StandaloneLootPartition => {
                STANDALONE_LOOT_PARTITION_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::Metagame => METAGAME_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::StructureBase => {
                STRUCTURE_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Faction => FACTION_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CompinstCommunicationsrv => {
                COMPINST_COMMUNICATIONSRV_ATTRIBUTES_ID
                    .get(&attr)
                    .map(|a| a.static_info())
            }
            Self::CompinstClustersrv => {
                COMPINST_CLUSTERSRV_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Structure => {
                STRUCTURE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::EdnaAbility => {
                EDNA_ABILITY_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::OaCommonConfig => {
                OA_COMMON_CONFIG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::SteamItem => {
                STEAM_ITEM_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::SpawnNode => {
                SPAWN_NODE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NonClientBase => {
                NON_CLIENT_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::InteractObject => {
                INTERACT_OBJECT_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NpcBase => NPC_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::EdnaReceptor => {
                EDNA_RECEPTOR_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::CompinstClusterapp => {
                COMPINST_CLUSTERAPP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::OaZoneConfig => {
                OA_ZONE_CONFIG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Version => VERSION_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::Ship => SHIP_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::PatrolNode => {
                PATROL_NODE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::LifeDirector => {
                LIFE_DIRECTOR_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Compinst => COMPINST_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::CompinstCommunitysrv => {
                COMPINST_COMMUNITYSRV_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Config => CONFIG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::LocalTacticNode => {
                LOCAL_TACTIC_NODE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::NpcShopConfig => {
                NPC_SHOP_CONFIG_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::LootScatterContainer => {
                LOOT_SCATTER_CONTAINER_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::MinigameScoreBoard => {
                MINIGAME_SCORE_BOARD_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info())
            }
            Self::Door => DOOR_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::Mail => MAIL_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
            Self::EdnaBase => EDNA_BASE_ATTRIBUTES_ID.get(&attr).map(|a| a.static_info()),
        }
    }
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            143u16 => Some(Self::LootSystem),
            149u16 => Some(Self::Clan),
            192u16 => Some(Self::CustomTrigger),
            178u16 => Some(Self::QuestBeacon),
            126u16 => Some(Self::MinigameMine),
            99u16 => Some(Self::VehicleFlying),
            77u16 => Some(Self::Player),
            116u16 => Some(Self::MinigameItem),
            137u16 => Some(Self::ItemMyLandTheme),
            131u16 => Some(Self::MylandScoreboard),
            147u16 => Some(Self::AiTemplate),
            179u16 => Some(Self::CombatSystemGroup),
            13u16 => Some(Self::CompinstDaemon),
            44u16 => Some(Self::Spawner),
            104u16 => Some(Self::MinigameInfo),
            146u16 => Some(Self::WorldDisplay),
            1u16 => Some(Self::OtherlandArea),
            185u16 => Some(Self::Instance),
            76u16 => Some(Self::Party),
            144u16 => Some(Self::ClassItem),
            7u16 => Some(Self::BuffBase),
            11u16 => Some(Self::CompinstMulticlusterAvatarLookup),
            175u16 => Some(Self::ClanRank),
            182u16 => Some(Self::CooldownGroupExternal),
            56u16 => Some(Self::Portal),
            62u16 => Some(Self::ChessPiece),
            139u16 => Some(Self::ItemPreset),
            109u16 => Some(Self::EdnaContainer),
            129u16 => Some(Self::ServerGateway),
            121u16 => Some(Self::OtherlandStructure),
            18u16 => Some(Self::CompinstClusternode),
            37u16 => Some(Self::EdnaModule),
            135u16 => Some(Self::MyLandSettings),
            154u16 => Some(Self::MypadRoomDoor),
            9u16 => Some(Self::OaBuff2),
            12u16 => Some(Self::CompinstFrontendsrv),
            26u16 => Some(Self::ItemBase),
            140u16 => Some(Self::PortalItem),
            130u16 => Some(Self::ServerGatewayExitPhase),
            34u16 => Some(Self::ItemEdna),
            98u16 => Some(Self::VehicleBase),
            176u16 => Some(Self::BundleItem),
            177u16 => Some(Self::ShopFilterSchema),
            61u16 => Some(Self::Trigger),
            95u16 => Some(Self::Trade),
            16u16 => Some(Self::CompinstLoginsrv),
            36u16 => Some(Self::EdnaFunction),
            191u16 => Some(Self::CommonConfig),
            174u16 => Some(Self::ClanMember),
            119u16 => Some(Self::JsonSchema),
            150u16 => Some(Self::SomaforgeItem),
            67u16 => Some(Self::Planet),
            152u16 => Some(Self::AbilityList),
            153u16 => Some(Self::NonSpawnPlacementRadius),
            42u16 => Some(Self::SpawnerBase),
            132u16 => Some(Self::NonSpawnPlacement),
            133u16 => Some(Self::CompinstMasterRedirectSrv),
            188u16 => Some(Self::SteamDlc),
            128u16 => Some(Self::CtfGameFlag),
            47u16 => Some(Self::NpcOtherland),
            22u16 => Some(Self::GameSession),
            180u16 => Some(Self::BuffGroup),
            181u16 => Some(Self::SkillGroup),
            57u16 => Some(Self::StartingPoint),
            106u16 => Some(Self::MoverBase),
            124u16 => Some(Self::PresetPoint),
            108u16 => Some(Self::PhysicsActor),
            113u16 => Some(Self::CooldownGroup),
            114u16 => Some(Self::BilliardBall),
            105u16 => Some(Self::ChessMetaGameLogic),
            120u16 => Some(Self::StandaloneLootPartition),
            142u16 => Some(Self::Metagame),
            54u16 => Some(Self::StructureBase),
            151u16 => Some(Self::Faction),
            15u16 => Some(Self::CompinstCommunicationsrv),
            19u16 => Some(Self::CompinstClustersrv),
            55u16 => Some(Self::Structure),
            21u16 => Some(Self::EdnaAbility),
            155u16 => Some(Self::OaCommonConfig),
            190u16 => Some(Self::SteamItem),
            71u16 => Some(Self::SpawnNode),
            41u16 => Some(Self::NonClientBase),
            68u16 => Some(Self::InteractObject),
            46u16 => Some(Self::NpcBase),
            69u16 => Some(Self::EdnaReceptor),
            17u16 => Some(Self::CompinstClusterapp),
            134u16 => Some(Self::OaZoneConfig),
            97u16 => Some(Self::Version),
            66u16 => Some(Self::Ship),
            70u16 => Some(Self::PatrolNode),
            148u16 => Some(Self::LifeDirector),
            10u16 => Some(Self::Compinst),
            14u16 => Some(Self::CompinstCommunitysrv),
            183u16 => Some(Self::Config),
            136u16 => Some(Self::LocalTacticNode),
            184u16 => Some(Self::NpcShopConfig),
            145u16 => Some(Self::LootScatterContainer),
            122u16 => Some(Self::MinigameScoreBoard),
            127u16 => Some(Self::Door),
            40u16 => Some(Self::Mail),
            35u16 => Some(Self::EdnaBase),
            _ => None,
        }
    }
    pub fn id(&self) -> u16 {
        match self {
            Self::LootSystem => 143u16,
            Self::Clan => 149u16,
            Self::CustomTrigger => 192u16,
            Self::QuestBeacon => 178u16,
            Self::MinigameMine => 126u16,
            Self::VehicleFlying => 99u16,
            Self::Player => 77u16,
            Self::MinigameItem => 116u16,
            Self::ItemMyLandTheme => 137u16,
            Self::MylandScoreboard => 131u16,
            Self::AiTemplate => 147u16,
            Self::CombatSystemGroup => 179u16,
            Self::CompinstDaemon => 13u16,
            Self::Spawner => 44u16,
            Self::MinigameInfo => 104u16,
            Self::WorldDisplay => 146u16,
            Self::OtherlandArea => 1u16,
            Self::Instance => 185u16,
            Self::Party => 76u16,
            Self::ClassItem => 144u16,
            Self::BuffBase => 7u16,
            Self::CompinstMulticlusterAvatarLookup => 11u16,
            Self::ClanRank => 175u16,
            Self::CooldownGroupExternal => 182u16,
            Self::Portal => 56u16,
            Self::ChessPiece => 62u16,
            Self::ItemPreset => 139u16,
            Self::EdnaContainer => 109u16,
            Self::ServerGateway => 129u16,
            Self::OtherlandStructure => 121u16,
            Self::CompinstClusternode => 18u16,
            Self::EdnaModule => 37u16,
            Self::MyLandSettings => 135u16,
            Self::MypadRoomDoor => 154u16,
            Self::OaBuff2 => 9u16,
            Self::CompinstFrontendsrv => 12u16,
            Self::ItemBase => 26u16,
            Self::PortalItem => 140u16,
            Self::ServerGatewayExitPhase => 130u16,
            Self::ItemEdna => 34u16,
            Self::VehicleBase => 98u16,
            Self::BundleItem => 176u16,
            Self::ShopFilterSchema => 177u16,
            Self::Trigger => 61u16,
            Self::Trade => 95u16,
            Self::CompinstLoginsrv => 16u16,
            Self::EdnaFunction => 36u16,
            Self::CommonConfig => 191u16,
            Self::ClanMember => 174u16,
            Self::JsonSchema => 119u16,
            Self::SomaforgeItem => 150u16,
            Self::Planet => 67u16,
            Self::AbilityList => 152u16,
            Self::NonSpawnPlacementRadius => 153u16,
            Self::SpawnerBase => 42u16,
            Self::NonSpawnPlacement => 132u16,
            Self::CompinstMasterRedirectSrv => 133u16,
            Self::SteamDlc => 188u16,
            Self::CtfGameFlag => 128u16,
            Self::NpcOtherland => 47u16,
            Self::GameSession => 22u16,
            Self::BuffGroup => 180u16,
            Self::SkillGroup => 181u16,
            Self::StartingPoint => 57u16,
            Self::MoverBase => 106u16,
            Self::PresetPoint => 124u16,
            Self::PhysicsActor => 108u16,
            Self::CooldownGroup => 113u16,
            Self::BilliardBall => 114u16,
            Self::ChessMetaGameLogic => 105u16,
            Self::StandaloneLootPartition => 120u16,
            Self::Metagame => 142u16,
            Self::StructureBase => 54u16,
            Self::Faction => 151u16,
            Self::CompinstCommunicationsrv => 15u16,
            Self::CompinstClustersrv => 19u16,
            Self::Structure => 55u16,
            Self::EdnaAbility => 21u16,
            Self::OaCommonConfig => 155u16,
            Self::SteamItem => 190u16,
            Self::SpawnNode => 71u16,
            Self::NonClientBase => 41u16,
            Self::InteractObject => 68u16,
            Self::NpcBase => 46u16,
            Self::EdnaReceptor => 69u16,
            Self::CompinstClusterapp => 17u16,
            Self::OaZoneConfig => 134u16,
            Self::Version => 97u16,
            Self::Ship => 66u16,
            Self::PatrolNode => 70u16,
            Self::LifeDirector => 148u16,
            Self::Compinst => 10u16,
            Self::CompinstCommunitysrv => 14u16,
            Self::Config => 183u16,
            Self::LocalTacticNode => 136u16,
            Self::NpcShopConfig => 184u16,
            Self::LootScatterContainer => 145u16,
            Self::MinigameScoreBoard => 122u16,
            Self::Door => 127u16,
            Self::Mail => 40u16,
            Self::EdnaBase => 35u16,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::LootSystem => "lootSystem",
            Self::Clan => "clan",
            Self::CustomTrigger => "CustomTrigger",
            Self::QuestBeacon => "QuestBeacon",
            Self::MinigameMine => "MinigameMine",
            Self::VehicleFlying => "vehicleFlying",
            Self::Player => "player",
            Self::MinigameItem => "minigameItem",
            Self::ItemMyLandTheme => "itemMyLandTheme",
            Self::MylandScoreboard => "mylandScoreboard",
            Self::AiTemplate => "AITemplate",
            Self::CombatSystemGroup => "combatSystemGroup",
            Self::CompinstDaemon => "compinst_daemon",
            Self::Spawner => "spawner",
            Self::MinigameInfo => "minigameInfo",
            Self::WorldDisplay => "WorldDisplay",
            Self::OtherlandArea => "OtherlandArea",
            Self::Instance => "instance",
            Self::Party => "party",
            Self::ClassItem => "classItem",
            Self::BuffBase => "buffBase",
            Self::CompinstMulticlusterAvatarLookup => "compinst_multiclusterAvatarLookup",
            Self::ClanRank => "clanRank",
            Self::CooldownGroupExternal => "CooldownGroupExternal",
            Self::Portal => "portal",
            Self::ChessPiece => "chessPiece",
            Self::ItemPreset => "ItemPreset",
            Self::EdnaContainer => "EDNAContainer",
            Self::ServerGateway => "ServerGateway",
            Self::OtherlandStructure => "OtherlandStructure",
            Self::CompinstClusternode => "compinst_clusternode",
            Self::EdnaModule => "ednaModule",
            Self::MyLandSettings => "MyLandSettings",
            Self::MypadRoomDoor => "mypadRoomDoor",
            Self::OaBuff2 => "oaBuff_2",
            Self::CompinstFrontendsrv => "compinst_frontendsrv",
            Self::ItemBase => "itemBase",
            Self::PortalItem => "PortalItem",
            Self::ServerGatewayExitPhase => "ServerGatewayExitPhase",
            Self::ItemEdna => "itemEdna",
            Self::VehicleBase => "vehicleBase",
            Self::BundleItem => "bundleItem",
            Self::ShopFilterSchema => "shopFilterSchema",
            Self::Trigger => "Trigger",
            Self::Trade => "trade",
            Self::CompinstLoginsrv => "compinst_loginsrv",
            Self::EdnaFunction => "ednaFunction",
            Self::CommonConfig => "CommonConfig",
            Self::ClanMember => "clanMember",
            Self::JsonSchema => "jsonSchema",
            Self::SomaforgeItem => "SomaforgeItem",
            Self::Planet => "Planet",
            Self::AbilityList => "abilityList",
            Self::NonSpawnPlacementRadius => "NonSpawnPlacementRadius",
            Self::SpawnerBase => "spawnerBase",
            Self::NonSpawnPlacement => "NonSpawnPlacement",
            Self::CompinstMasterRedirectSrv => "compinst_masterRedirectSrv",
            Self::SteamDlc => "SteamDlc",
            Self::CtfGameFlag => "CTFGameFlag",
            Self::NpcOtherland => "npcOtherland",
            Self::GameSession => "gameSession",
            Self::BuffGroup => "BuffGroup",
            Self::SkillGroup => "SkillGroup",
            Self::StartingPoint => "startingPoint",
            Self::MoverBase => "moverBase",
            Self::PresetPoint => "presetPoint",
            Self::PhysicsActor => "physicsActor",
            Self::CooldownGroup => "cooldownGroup",
            Self::BilliardBall => "billiardBall",
            Self::ChessMetaGameLogic => "chessMetaGameLogic",
            Self::StandaloneLootPartition => "standaloneLootPartition",
            Self::Metagame => "metagame",
            Self::StructureBase => "structureBase",
            Self::Faction => "faction",
            Self::CompinstCommunicationsrv => "compinst_communicationsrv",
            Self::CompinstClustersrv => "compinst_clustersrv",
            Self::Structure => "structure",
            Self::EdnaAbility => "ednaAbility",
            Self::OaCommonConfig => "oaCommonConfig",
            Self::SteamItem => "SteamItem",
            Self::SpawnNode => "spawnNode",
            Self::NonClientBase => "nonClientBase",
            Self::InteractObject => "InteractObject",
            Self::NpcBase => "npcBase",
            Self::EdnaReceptor => "EDNAReceptor",
            Self::CompinstClusterapp => "compinst_clusterapp",
            Self::OaZoneConfig => "oaZoneConfig",
            Self::Version => "version",
            Self::Ship => "Ship",
            Self::PatrolNode => "patrolNode",
            Self::LifeDirector => "LifeDirector",
            Self::Compinst => "compinst",
            Self::CompinstCommunitysrv => "compinst_communitysrv",
            Self::Config => "config",
            Self::LocalTacticNode => "LocalTacticNode",
            Self::NpcShopConfig => "npcShopConfig",
            Self::LootScatterContainer => "LootScatterContainer",
            Self::MinigameScoreBoard => "minigameScoreBoard",
            Self::Door => "door",
            Self::Mail => "mail",
            Self::EdnaBase => "ednaBase",
        }
    }
    pub fn parent(&self) -> Option<Class> {
        match self {
            Self::LootSystem => Some(Class::Metagame),
            Self::Clan => None,
            Self::CustomTrigger => Some(Class::Structure),
            Self::QuestBeacon => Some(Class::NonSpawnPlacement),
            Self::MinigameMine => Some(Class::Structure),
            Self::VehicleFlying => Some(Class::VehicleBase),
            Self::Player => None,
            Self::MinigameItem => Some(Class::ItemBase),
            Self::ItemMyLandTheme => Some(Class::ItemBase),
            Self::MylandScoreboard => Some(Class::NonClientBase),
            Self::AiTemplate => None,
            Self::CombatSystemGroup => Some(Class::Metagame),
            Self::CompinstDaemon => Some(Class::Compinst),
            Self::Spawner => Some(Class::SpawnerBase),
            Self::MinigameInfo => Some(Class::NonClientBase),
            Self::WorldDisplay => Some(Class::Structure),
            Self::OtherlandArea => None,
            Self::Instance => Some(Class::Config),
            Self::Party => None,
            Self::ClassItem => Some(Class::ItemBase),
            Self::BuffBase => None,
            Self::CompinstMulticlusterAvatarLookup => Some(Class::Compinst),
            Self::ClanRank => None,
            Self::CooldownGroupExternal => Some(Class::CombatSystemGroup),
            Self::Portal => Some(Class::Structure),
            Self::ChessPiece => Some(Class::Structure),
            Self::ItemPreset => None,
            Self::EdnaContainer => Some(Class::InteractObject),
            Self::ServerGateway => Some(Class::Structure),
            Self::OtherlandStructure => Some(Class::Structure),
            Self::CompinstClusternode => Some(Class::CompinstClusterapp),
            Self::EdnaModule => Some(Class::ItemEdna),
            Self::MyLandSettings => Some(Class::Structure),
            Self::MypadRoomDoor => Some(Class::Structure),
            Self::OaBuff2 => Some(Class::BuffBase),
            Self::CompinstFrontendsrv => Some(Class::Compinst),
            Self::ItemBase => None,
            Self::PortalItem => Some(Class::ItemBase),
            Self::ServerGatewayExitPhase => Some(Class::Structure),
            Self::ItemEdna => Some(Class::ItemBase),
            Self::VehicleBase => Some(Class::NonClientBase),
            Self::BundleItem => Some(Class::ItemEdna),
            Self::ShopFilterSchema => Some(Class::OaCommonConfig),
            Self::Trigger => Some(Class::Structure),
            Self::Trade => None,
            Self::CompinstLoginsrv => Some(Class::Compinst),
            Self::EdnaFunction => Some(Class::ItemEdna),
            Self::CommonConfig => Some(Class::OaCommonConfig),
            Self::ClanMember => None,
            Self::JsonSchema => None,
            Self::SomaforgeItem => Some(Class::ItemEdna),
            Self::Planet => Some(Class::Structure),
            Self::AbilityList => None,
            Self::NonSpawnPlacementRadius => Some(Class::NonSpawnPlacement),
            Self::SpawnerBase => Some(Class::NonClientBase),
            Self::NonSpawnPlacement => None,
            Self::CompinstMasterRedirectSrv => Some(Class::Compinst),
            Self::SteamDlc => None,
            Self::CtfGameFlag => Some(Class::Structure),
            Self::NpcOtherland => Some(Class::NpcBase),
            Self::GameSession => None,
            Self::BuffGroup => Some(Class::CombatSystemGroup),
            Self::SkillGroup => Some(Class::CombatSystemGroup),
            Self::StartingPoint => Some(Class::Structure),
            Self::MoverBase => Some(Class::NonClientBase),
            Self::PresetPoint => Some(Class::Structure),
            Self::PhysicsActor => Some(Class::MoverBase),
            Self::CooldownGroup => None,
            Self::BilliardBall => Some(Class::PhysicsActor),
            Self::ChessMetaGameLogic => Some(Class::Structure),
            Self::StandaloneLootPartition => None,
            Self::Metagame => None,
            Self::StructureBase => Some(Class::NonClientBase),
            Self::Faction => None,
            Self::CompinstCommunicationsrv => Some(Class::Compinst),
            Self::CompinstClustersrv => Some(Class::CompinstClusterapp),
            Self::Structure => Some(Class::StructureBase),
            Self::EdnaAbility => None,
            Self::OaCommonConfig => None,
            Self::SteamItem => None,
            Self::SpawnNode => Some(Class::Structure),
            Self::NonClientBase => None,
            Self::InteractObject => Some(Class::Structure),
            Self::NpcBase => Some(Class::NonClientBase),
            Self::EdnaReceptor => Some(Class::InteractObject),
            Self::CompinstClusterapp => Some(Class::Compinst),
            Self::OaZoneConfig => None,
            Self::Version => None,
            Self::Ship => Some(Class::Structure),
            Self::PatrolNode => Some(Class::Structure),
            Self::LifeDirector => None,
            Self::Compinst => None,
            Self::CompinstCommunitysrv => Some(Class::Compinst),
            Self::Config => None,
            Self::LocalTacticNode => Some(Class::Structure),
            Self::NpcShopConfig => Some(Class::Config),
            Self::LootScatterContainer => Some(Class::Structure),
            Self::MinigameScoreBoard => Some(Class::NonClientBase),
            Self::Door => Some(Class::Structure),
            Self::Mail => None,
            Self::EdnaBase => Some(Class::ItemEdna),
        }
    }
    pub(crate) fn create_param_set(
        &self,
        attributes: Vec<(&'static dyn AttributeInfo, Value)>,
    ) -> Box<dyn GenericParamSet> {
        match self {
            Self::LootSystem => {
                Box::new(ParamSet::<LootSystem>::new_from_attributes(attributes))
            }
            Self::Clan => Box::new(ParamSet::<Clan>::new_from_attributes(attributes)),
            Self::CustomTrigger => {
                Box::new(ParamSet::<CustomTrigger>::new_from_attributes(attributes))
            }
            Self::QuestBeacon => {
                Box::new(ParamSet::<QuestBeacon>::new_from_attributes(attributes))
            }
            Self::MinigameMine => {
                Box::new(ParamSet::<MinigameMine>::new_from_attributes(attributes))
            }
            Self::VehicleFlying => {
                Box::new(ParamSet::<VehicleFlying>::new_from_attributes(attributes))
            }
            Self::Player => Box::new(ParamSet::<Player>::new_from_attributes(attributes)),
            Self::MinigameItem => {
                Box::new(ParamSet::<MinigameItem>::new_from_attributes(attributes))
            }
            Self::ItemMyLandTheme => {
                Box::new(ParamSet::<ItemMyLandTheme>::new_from_attributes(attributes))
            }
            Self::MylandScoreboard => {
                Box::new(ParamSet::<MylandScoreboard>::new_from_attributes(attributes))
            }
            Self::AiTemplate => {
                Box::new(ParamSet::<AiTemplate>::new_from_attributes(attributes))
            }
            Self::CombatSystemGroup => {
                Box::new(ParamSet::<CombatSystemGroup>::new_from_attributes(attributes))
            }
            Self::CompinstDaemon => {
                Box::new(ParamSet::<CompinstDaemon>::new_from_attributes(attributes))
            }
            Self::Spawner => {
                Box::new(ParamSet::<Spawner>::new_from_attributes(attributes))
            }
            Self::MinigameInfo => {
                Box::new(ParamSet::<MinigameInfo>::new_from_attributes(attributes))
            }
            Self::WorldDisplay => {
                Box::new(ParamSet::<WorldDisplay>::new_from_attributes(attributes))
            }
            Self::OtherlandArea => {
                Box::new(ParamSet::<OtherlandArea>::new_from_attributes(attributes))
            }
            Self::Instance => {
                Box::new(ParamSet::<Instance>::new_from_attributes(attributes))
            }
            Self::Party => Box::new(ParamSet::<Party>::new_from_attributes(attributes)),
            Self::ClassItem => {
                Box::new(ParamSet::<ClassItem>::new_from_attributes(attributes))
            }
            Self::BuffBase => {
                Box::new(ParamSet::<BuffBase>::new_from_attributes(attributes))
            }
            Self::CompinstMulticlusterAvatarLookup => {
                Box::new(
                    ParamSet::<
                        CompinstMulticlusterAvatarLookup,
                    >::new_from_attributes(attributes),
                )
            }
            Self::ClanRank => {
                Box::new(ParamSet::<ClanRank>::new_from_attributes(attributes))
            }
            Self::CooldownGroupExternal => {
                Box::new(
                    ParamSet::<CooldownGroupExternal>::new_from_attributes(attributes),
                )
            }
            Self::Portal => Box::new(ParamSet::<Portal>::new_from_attributes(attributes)),
            Self::ChessPiece => {
                Box::new(ParamSet::<ChessPiece>::new_from_attributes(attributes))
            }
            Self::ItemPreset => {
                Box::new(ParamSet::<ItemPreset>::new_from_attributes(attributes))
            }
            Self::EdnaContainer => {
                Box::new(ParamSet::<EdnaContainer>::new_from_attributes(attributes))
            }
            Self::ServerGateway => {
                Box::new(ParamSet::<ServerGateway>::new_from_attributes(attributes))
            }
            Self::OtherlandStructure => {
                Box::new(ParamSet::<OtherlandStructure>::new_from_attributes(attributes))
            }
            Self::CompinstClusternode => {
                Box::new(
                    ParamSet::<CompinstClusternode>::new_from_attributes(attributes),
                )
            }
            Self::EdnaModule => {
                Box::new(ParamSet::<EdnaModule>::new_from_attributes(attributes))
            }
            Self::MyLandSettings => {
                Box::new(ParamSet::<MyLandSettings>::new_from_attributes(attributes))
            }
            Self::MypadRoomDoor => {
                Box::new(ParamSet::<MypadRoomDoor>::new_from_attributes(attributes))
            }
            Self::OaBuff2 => {
                Box::new(ParamSet::<OaBuff2>::new_from_attributes(attributes))
            }
            Self::CompinstFrontendsrv => {
                Box::new(
                    ParamSet::<CompinstFrontendsrv>::new_from_attributes(attributes),
                )
            }
            Self::ItemBase => {
                Box::new(ParamSet::<ItemBase>::new_from_attributes(attributes))
            }
            Self::PortalItem => {
                Box::new(ParamSet::<PortalItem>::new_from_attributes(attributes))
            }
            Self::ServerGatewayExitPhase => {
                Box::new(
                    ParamSet::<ServerGatewayExitPhase>::new_from_attributes(attributes),
                )
            }
            Self::ItemEdna => {
                Box::new(ParamSet::<ItemEdna>::new_from_attributes(attributes))
            }
            Self::VehicleBase => {
                Box::new(ParamSet::<VehicleBase>::new_from_attributes(attributes))
            }
            Self::BundleItem => {
                Box::new(ParamSet::<BundleItem>::new_from_attributes(attributes))
            }
            Self::ShopFilterSchema => {
                Box::new(ParamSet::<ShopFilterSchema>::new_from_attributes(attributes))
            }
            Self::Trigger => {
                Box::new(ParamSet::<Trigger>::new_from_attributes(attributes))
            }
            Self::Trade => Box::new(ParamSet::<Trade>::new_from_attributes(attributes)),
            Self::CompinstLoginsrv => {
                Box::new(ParamSet::<CompinstLoginsrv>::new_from_attributes(attributes))
            }
            Self::EdnaFunction => {
                Box::new(ParamSet::<EdnaFunction>::new_from_attributes(attributes))
            }
            Self::CommonConfig => {
                Box::new(ParamSet::<CommonConfig>::new_from_attributes(attributes))
            }
            Self::ClanMember => {
                Box::new(ParamSet::<ClanMember>::new_from_attributes(attributes))
            }
            Self::JsonSchema => {
                Box::new(ParamSet::<JsonSchema>::new_from_attributes(attributes))
            }
            Self::SomaforgeItem => {
                Box::new(ParamSet::<SomaforgeItem>::new_from_attributes(attributes))
            }
            Self::Planet => Box::new(ParamSet::<Planet>::new_from_attributes(attributes)),
            Self::AbilityList => {
                Box::new(ParamSet::<AbilityList>::new_from_attributes(attributes))
            }
            Self::NonSpawnPlacementRadius => {
                Box::new(
                    ParamSet::<NonSpawnPlacementRadius>::new_from_attributes(attributes),
                )
            }
            Self::SpawnerBase => {
                Box::new(ParamSet::<SpawnerBase>::new_from_attributes(attributes))
            }
            Self::NonSpawnPlacement => {
                Box::new(ParamSet::<NonSpawnPlacement>::new_from_attributes(attributes))
            }
            Self::CompinstMasterRedirectSrv => {
                Box::new(
                    ParamSet::<
                        CompinstMasterRedirectSrv,
                    >::new_from_attributes(attributes),
                )
            }
            Self::SteamDlc => {
                Box::new(ParamSet::<SteamDlc>::new_from_attributes(attributes))
            }
            Self::CtfGameFlag => {
                Box::new(ParamSet::<CtfGameFlag>::new_from_attributes(attributes))
            }
            Self::NpcOtherland => {
                Box::new(ParamSet::<NpcOtherland>::new_from_attributes(attributes))
            }
            Self::GameSession => {
                Box::new(ParamSet::<GameSession>::new_from_attributes(attributes))
            }
            Self::BuffGroup => {
                Box::new(ParamSet::<BuffGroup>::new_from_attributes(attributes))
            }
            Self::SkillGroup => {
                Box::new(ParamSet::<SkillGroup>::new_from_attributes(attributes))
            }
            Self::StartingPoint => {
                Box::new(ParamSet::<StartingPoint>::new_from_attributes(attributes))
            }
            Self::MoverBase => {
                Box::new(ParamSet::<MoverBase>::new_from_attributes(attributes))
            }
            Self::PresetPoint => {
                Box::new(ParamSet::<PresetPoint>::new_from_attributes(attributes))
            }
            Self::PhysicsActor => {
                Box::new(ParamSet::<PhysicsActor>::new_from_attributes(attributes))
            }
            Self::CooldownGroup => {
                Box::new(ParamSet::<CooldownGroup>::new_from_attributes(attributes))
            }
            Self::BilliardBall => {
                Box::new(ParamSet::<BilliardBall>::new_from_attributes(attributes))
            }
            Self::ChessMetaGameLogic => {
                Box::new(ParamSet::<ChessMetaGameLogic>::new_from_attributes(attributes))
            }
            Self::StandaloneLootPartition => {
                Box::new(
                    ParamSet::<StandaloneLootPartition>::new_from_attributes(attributes),
                )
            }
            Self::Metagame => {
                Box::new(ParamSet::<Metagame>::new_from_attributes(attributes))
            }
            Self::StructureBase => {
                Box::new(ParamSet::<StructureBase>::new_from_attributes(attributes))
            }
            Self::Faction => {
                Box::new(ParamSet::<Faction>::new_from_attributes(attributes))
            }
            Self::CompinstCommunicationsrv => {
                Box::new(
                    ParamSet::<CompinstCommunicationsrv>::new_from_attributes(attributes),
                )
            }
            Self::CompinstClustersrv => {
                Box::new(ParamSet::<CompinstClustersrv>::new_from_attributes(attributes))
            }
            Self::Structure => {
                Box::new(ParamSet::<Structure>::new_from_attributes(attributes))
            }
            Self::EdnaAbility => {
                Box::new(ParamSet::<EdnaAbility>::new_from_attributes(attributes))
            }
            Self::OaCommonConfig => {
                Box::new(ParamSet::<OaCommonConfig>::new_from_attributes(attributes))
            }
            Self::SteamItem => {
                Box::new(ParamSet::<SteamItem>::new_from_attributes(attributes))
            }
            Self::SpawnNode => {
                Box::new(ParamSet::<SpawnNode>::new_from_attributes(attributes))
            }
            Self::NonClientBase => {
                Box::new(ParamSet::<NonClientBase>::new_from_attributes(attributes))
            }
            Self::InteractObject => {
                Box::new(ParamSet::<InteractObject>::new_from_attributes(attributes))
            }
            Self::NpcBase => {
                Box::new(ParamSet::<NpcBase>::new_from_attributes(attributes))
            }
            Self::EdnaReceptor => {
                Box::new(ParamSet::<EdnaReceptor>::new_from_attributes(attributes))
            }
            Self::CompinstClusterapp => {
                Box::new(ParamSet::<CompinstClusterapp>::new_from_attributes(attributes))
            }
            Self::OaZoneConfig => {
                Box::new(ParamSet::<OaZoneConfig>::new_from_attributes(attributes))
            }
            Self::Version => {
                Box::new(ParamSet::<Version>::new_from_attributes(attributes))
            }
            Self::Ship => Box::new(ParamSet::<Ship>::new_from_attributes(attributes)),
            Self::PatrolNode => {
                Box::new(ParamSet::<PatrolNode>::new_from_attributes(attributes))
            }
            Self::LifeDirector => {
                Box::new(ParamSet::<LifeDirector>::new_from_attributes(attributes))
            }
            Self::Compinst => {
                Box::new(ParamSet::<Compinst>::new_from_attributes(attributes))
            }
            Self::CompinstCommunitysrv => {
                Box::new(
                    ParamSet::<CompinstCommunitysrv>::new_from_attributes(attributes),
                )
            }
            Self::Config => Box::new(ParamSet::<Config>::new_from_attributes(attributes)),
            Self::LocalTacticNode => {
                Box::new(ParamSet::<LocalTacticNode>::new_from_attributes(attributes))
            }
            Self::NpcShopConfig => {
                Box::new(ParamSet::<NpcShopConfig>::new_from_attributes(attributes))
            }
            Self::LootScatterContainer => {
                Box::new(
                    ParamSet::<LootScatterContainer>::new_from_attributes(attributes),
                )
            }
            Self::MinigameScoreBoard => {
                Box::new(ParamSet::<MinigameScoreBoard>::new_from_attributes(attributes))
            }
            Self::Door => Box::new(ParamSet::<Door>::new_from_attributes(attributes)),
            Self::Mail => Box::new(ParamSet::<Mail>::new_from_attributes(attributes)),
            Self::EdnaBase => {
                Box::new(ParamSet::<EdnaBase>::new_from_attributes(attributes))
            }
        }
    }
}
impl FromStr for Class {
    type Err = ParamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lootSystem" => Ok(Self::LootSystem),
            "clan" => Ok(Self::Clan),
            "CustomTrigger" => Ok(Self::CustomTrigger),
            "QuestBeacon" => Ok(Self::QuestBeacon),
            "MinigameMine" => Ok(Self::MinigameMine),
            "vehicleFlying" => Ok(Self::VehicleFlying),
            "player" => Ok(Self::Player),
            "minigameItem" => Ok(Self::MinigameItem),
            "itemMyLandTheme" => Ok(Self::ItemMyLandTheme),
            "mylandScoreboard" => Ok(Self::MylandScoreboard),
            "AITemplate" => Ok(Self::AiTemplate),
            "combatSystemGroup" => Ok(Self::CombatSystemGroup),
            "compinst_daemon" => Ok(Self::CompinstDaemon),
            "spawner" => Ok(Self::Spawner),
            "minigameInfo" => Ok(Self::MinigameInfo),
            "WorldDisplay" => Ok(Self::WorldDisplay),
            "OtherlandArea" => Ok(Self::OtherlandArea),
            "instance" => Ok(Self::Instance),
            "party" => Ok(Self::Party),
            "classItem" => Ok(Self::ClassItem),
            "buffBase" => Ok(Self::BuffBase),
            "compinst_multiclusterAvatarLookup" => {
                Ok(Self::CompinstMulticlusterAvatarLookup)
            }
            "clanRank" => Ok(Self::ClanRank),
            "CooldownGroupExternal" => Ok(Self::CooldownGroupExternal),
            "portal" => Ok(Self::Portal),
            "chessPiece" => Ok(Self::ChessPiece),
            "ItemPreset" => Ok(Self::ItemPreset),
            "EDNAContainer" => Ok(Self::EdnaContainer),
            "ServerGateway" => Ok(Self::ServerGateway),
            "OtherlandStructure" => Ok(Self::OtherlandStructure),
            "compinst_clusternode" => Ok(Self::CompinstClusternode),
            "ednaModule" => Ok(Self::EdnaModule),
            "MyLandSettings" => Ok(Self::MyLandSettings),
            "mypadRoomDoor" => Ok(Self::MypadRoomDoor),
            "oaBuff_2" => Ok(Self::OaBuff2),
            "compinst_frontendsrv" => Ok(Self::CompinstFrontendsrv),
            "itemBase" => Ok(Self::ItemBase),
            "PortalItem" => Ok(Self::PortalItem),
            "ServerGatewayExitPhase" => Ok(Self::ServerGatewayExitPhase),
            "itemEdna" => Ok(Self::ItemEdna),
            "vehicleBase" => Ok(Self::VehicleBase),
            "bundleItem" => Ok(Self::BundleItem),
            "shopFilterSchema" => Ok(Self::ShopFilterSchema),
            "Trigger" => Ok(Self::Trigger),
            "trade" => Ok(Self::Trade),
            "compinst_loginsrv" => Ok(Self::CompinstLoginsrv),
            "ednaFunction" => Ok(Self::EdnaFunction),
            "CommonConfig" => Ok(Self::CommonConfig),
            "clanMember" => Ok(Self::ClanMember),
            "jsonSchema" => Ok(Self::JsonSchema),
            "SomaforgeItem" => Ok(Self::SomaforgeItem),
            "Planet" => Ok(Self::Planet),
            "abilityList" => Ok(Self::AbilityList),
            "NonSpawnPlacementRadius" => Ok(Self::NonSpawnPlacementRadius),
            "spawnerBase" => Ok(Self::SpawnerBase),
            "NonSpawnPlacement" => Ok(Self::NonSpawnPlacement),
            "compinst_masterRedirectSrv" => Ok(Self::CompinstMasterRedirectSrv),
            "SteamDlc" => Ok(Self::SteamDlc),
            "CTFGameFlag" => Ok(Self::CtfGameFlag),
            "npcOtherland" => Ok(Self::NpcOtherland),
            "gameSession" => Ok(Self::GameSession),
            "BuffGroup" => Ok(Self::BuffGroup),
            "SkillGroup" => Ok(Self::SkillGroup),
            "startingPoint" => Ok(Self::StartingPoint),
            "moverBase" => Ok(Self::MoverBase),
            "presetPoint" => Ok(Self::PresetPoint),
            "physicsActor" => Ok(Self::PhysicsActor),
            "cooldownGroup" => Ok(Self::CooldownGroup),
            "billiardBall" => Ok(Self::BilliardBall),
            "chessMetaGameLogic" => Ok(Self::ChessMetaGameLogic),
            "standaloneLootPartition" => Ok(Self::StandaloneLootPartition),
            "metagame" => Ok(Self::Metagame),
            "structureBase" => Ok(Self::StructureBase),
            "faction" => Ok(Self::Faction),
            "compinst_communicationsrv" => Ok(Self::CompinstCommunicationsrv),
            "compinst_clustersrv" => Ok(Self::CompinstClustersrv),
            "structure" => Ok(Self::Structure),
            "ednaAbility" => Ok(Self::EdnaAbility),
            "oaCommonConfig" => Ok(Self::OaCommonConfig),
            "SteamItem" => Ok(Self::SteamItem),
            "spawnNode" => Ok(Self::SpawnNode),
            "nonClientBase" => Ok(Self::NonClientBase),
            "InteractObject" => Ok(Self::InteractObject),
            "npcBase" => Ok(Self::NpcBase),
            "EDNAReceptor" => Ok(Self::EdnaReceptor),
            "compinst_clusterapp" => Ok(Self::CompinstClusterapp),
            "oaZoneConfig" => Ok(Self::OaZoneConfig),
            "version" => Ok(Self::Version),
            "Ship" => Ok(Self::Ship),
            "patrolNode" => Ok(Self::PatrolNode),
            "LifeDirector" => Ok(Self::LifeDirector),
            "compinst" => Ok(Self::Compinst),
            "compinst_communitysrv" => Ok(Self::CompinstCommunitysrv),
            "config" => Ok(Self::Config),
            "LocalTacticNode" => Ok(Self::LocalTacticNode),
            "npcShopConfig" => Ok(Self::NpcShopConfig),
            "LootScatterContainer" => Ok(Self::LootScatterContainer),
            "minigameScoreBoard" => Ok(Self::MinigameScoreBoard),
            "door" => Ok(Self::Door),
            "mail" => Ok(Self::Mail),
            "ednaBase" => Ok(Self::EdnaBase),
            _ => Err(ParamError::UnknownClass),
        }
    }
}
