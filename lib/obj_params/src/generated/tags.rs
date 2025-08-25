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

use bevy::prelude::*;
use crate::Class;
use crate::GameObjectData;
#[derive(Component)]
pub struct CtfGameFlagTag;
#[derive(Component)]
pub struct SkillGroupTag;
#[derive(Component)]
pub struct OaCommonConfigTag;
#[derive(Component)]
pub struct PartyTag;
#[derive(Component)]
pub struct CompinstFrontendsrvTag;
#[derive(Component)]
pub struct ItemEdnaTag;
#[derive(Component)]
pub struct ClassItemTag;
#[derive(Component)]
pub struct MailTag;
#[derive(Component)]
pub struct PlanetTag;
#[derive(Component)]
pub struct MinigameMineTag;
#[derive(Component)]
pub struct CompinstCommunicationsrvTag;
#[derive(Component)]
pub struct InteractObjectTag;
#[derive(Component)]
pub struct MinigameItemTag;
#[derive(Component)]
pub struct ChessPieceTag;
#[derive(Component)]
pub struct CompinstClustersrvTag;
#[derive(Component)]
pub struct NpcShopConfigTag;
#[derive(Component)]
pub struct LootScatterContainerTag;
#[derive(Component)]
pub struct FactionTag;
#[derive(Component)]
pub struct ItemBaseTag;
#[derive(Component)]
pub struct BundleItemTag;
#[derive(Component)]
pub struct BilliardBallTag;
#[derive(Component)]
pub struct CommonConfigTag;
#[derive(Component)]
pub struct PlayerTag;
#[derive(Component)]
pub struct NonSpawnPlacementTag;
#[derive(Component)]
pub struct ChessMetaGameLogicTag;
#[derive(Component)]
pub struct GameSessionTag;
#[derive(Component)]
pub struct PatrolNodeTag;
#[derive(Component)]
pub struct TradeTag;
#[derive(Component)]
pub struct SomaforgeItemTag;
#[derive(Component)]
pub struct EdnaBaseTag;
#[derive(Component)]
pub struct PortalTag;
#[derive(Component)]
pub struct ServerGatewayTag;
#[derive(Component)]
pub struct CompinstLoginsrvTag;
#[derive(Component)]
pub struct ItemMyLandThemeTag;
#[derive(Component)]
pub struct StructureTag;
#[derive(Component)]
pub struct BuffGroupTag;
#[derive(Component)]
pub struct CustomTriggerTag;
#[derive(Component)]
pub struct VehicleFlyingTag;
#[derive(Component)]
pub struct LocalTacticNodeTag;
#[derive(Component)]
pub struct StructureBaseTag;
#[derive(Component)]
pub struct CompinstMulticlusterAvatarLookupTag;
#[derive(Component)]
pub struct EdnaModuleTag;
#[derive(Component)]
pub struct CombatSystemGroupTag;
#[derive(Component)]
pub struct MinigameScoreBoardTag;
#[derive(Component)]
pub struct VehicleBaseTag;
#[derive(Component)]
pub struct CompinstClusternodeTag;
#[derive(Component)]
pub struct MypadRoomDoorTag;
#[derive(Component)]
pub struct MoverBaseTag;
#[derive(Component)]
pub struct CompinstCommunitysrvTag;
#[derive(Component)]
pub struct MyLandSettingsTag;
#[derive(Component)]
pub struct ServerGatewayExitPhaseTag;
#[derive(Component)]
pub struct EdnaReceptorTag;
#[derive(Component)]
pub struct OtherlandStructureTag;
#[derive(Component)]
pub struct EdnaAbilityTag;
#[derive(Component)]
pub struct EdnaFunctionTag;
#[derive(Component)]
pub struct MetagameTag;
#[derive(Component)]
pub struct StartingPointTag;
#[derive(Component)]
pub struct PhysicsActorTag;
#[derive(Component)]
pub struct ClanMemberTag;
#[derive(Component)]
pub struct PortalItemTag;
#[derive(Component)]
pub struct QuestBeaconTag;
#[derive(Component)]
pub struct LootSystemTag;
#[derive(Component)]
pub struct SpawnerTag;
#[derive(Component)]
pub struct CompinstClusterappTag;
#[derive(Component)]
pub struct NonClientBaseTag;
#[derive(Component)]
pub struct BuffBaseTag;
#[derive(Component)]
pub struct CompinstMasterRedirectSrvTag;
#[derive(Component)]
pub struct WorldDisplayTag;
#[derive(Component)]
pub struct ShopFilterSchemaTag;
#[derive(Component)]
pub struct ItemPresetTag;
#[derive(Component)]
pub struct CompinstDaemonTag;
#[derive(Component)]
pub struct CooldownGroupTag;
#[derive(Component)]
pub struct SpawnerBaseTag;
#[derive(Component)]
pub struct ShipTag;
#[derive(Component)]
pub struct AiTemplateTag;
#[derive(Component)]
pub struct TriggerTag;
#[derive(Component)]
pub struct NpcBaseTag;
#[derive(Component)]
pub struct OaZoneConfigTag;
#[derive(Component)]
pub struct LifeDirectorTag;
#[derive(Component)]
pub struct InstanceTag;
#[derive(Component)]
pub struct JsonSchemaTag;
#[derive(Component)]
pub struct OaBuff2Tag;
#[derive(Component)]
pub struct PresetPointTag;
#[derive(Component)]
pub struct VersionTag;
#[derive(Component)]
pub struct CompinstTag;
#[derive(Component)]
pub struct SteamDlcTag;
#[derive(Component)]
pub struct ClanRankTag;
#[derive(Component)]
pub struct SteamItemTag;
#[derive(Component)]
pub struct CooldownGroupExternalTag;
#[derive(Component)]
pub struct MinigameInfoTag;
#[derive(Component)]
pub struct StandaloneLootPartitionTag;
#[derive(Component)]
pub struct OtherlandAreaTag;
#[derive(Component)]
pub struct ConfigTag;
#[derive(Component)]
pub struct AbilityListTag;
#[derive(Component)]
pub struct ClanTag;
#[derive(Component)]
pub struct NpcOtherlandTag;
#[derive(Component)]
pub struct SpawnNodeTag;
#[derive(Component)]
pub struct MylandScoreboardTag;
#[derive(Component)]
pub struct NonSpawnPlacementRadiusTag;
#[derive(Component)]
pub struct EdnaContainerTag;
#[derive(Component)]
pub struct DoorTag;
pub fn tag_gameobject_entity(data: &GameObjectData, commands: &mut EntityCommands<'_>) {
    match data.class() {
        Class::CtfGameFlag => {
            commands
                .insert((
                    CtfGameFlagTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::SkillGroup => {
            commands.insert((SkillGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::OaCommonConfig => {
            commands.insert(OaCommonConfigTag);
        }
        Class::Party => {
            commands.insert(PartyTag);
        }
        Class::CompinstFrontendsrv => {
            commands.insert((CompinstFrontendsrvTag, CompinstTag));
        }
        Class::ItemEdna => {
            commands.insert((ItemEdnaTag, ItemBaseTag));
        }
        Class::ClassItem => {
            commands.insert((ClassItemTag, ItemBaseTag));
        }
        Class::Mail => {
            commands.insert(MailTag);
        }
        Class::Planet => {
            commands
                .insert((PlanetTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::MinigameMine => {
            commands
                .insert((
                    MinigameMineTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstCommunicationsrv => {
            commands.insert((CompinstCommunicationsrvTag, CompinstTag));
        }
        Class::InteractObject => {
            commands
                .insert((
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::MinigameItem => {
            commands.insert((MinigameItemTag, ItemBaseTag));
        }
        Class::ChessPiece => {
            commands
                .insert((
                    ChessPieceTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstClustersrv => {
            commands.insert((CompinstClustersrvTag, CompinstClusterappTag, CompinstTag));
        }
        Class::NpcShopConfig => {
            commands.insert((NpcShopConfigTag, ConfigTag));
        }
        Class::LootScatterContainer => {
            commands
                .insert((
                    LootScatterContainerTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Faction => {
            commands.insert(FactionTag);
        }
        Class::ItemBase => {
            commands.insert(ItemBaseTag);
        }
        Class::BundleItem => {
            commands.insert((BundleItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::BilliardBall => {
            commands
                .insert((
                    BilliardBallTag,
                    PhysicsActorTag,
                    MoverBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CommonConfig => {
            commands.insert((CommonConfigTag, OaCommonConfigTag));
        }
        Class::Player => {
            commands.insert(PlayerTag);
        }
        Class::NonSpawnPlacement => {
            commands.insert(NonSpawnPlacementTag);
        }
        Class::ChessMetaGameLogic => {
            commands
                .insert((
                    ChessMetaGameLogicTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::GameSession => {
            commands.insert(GameSessionTag);
        }
        Class::PatrolNode => {
            commands
                .insert((
                    PatrolNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Trade => {
            commands.insert(TradeTag);
        }
        Class::SomaforgeItem => {
            commands.insert((SomaforgeItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::EdnaBase => {
            commands.insert((EdnaBaseTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Portal => {
            commands
                .insert((PortalTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::ServerGateway => {
            commands
                .insert((
                    ServerGatewayTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstLoginsrv => {
            commands.insert((CompinstLoginsrvTag, CompinstTag));
        }
        Class::ItemMyLandTheme => {
            commands.insert((ItemMyLandThemeTag, ItemBaseTag));
        }
        Class::Structure => {
            commands.insert((StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::BuffGroup => {
            commands.insert((BuffGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::CustomTrigger => {
            commands
                .insert((
                    CustomTriggerTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::VehicleFlying => {
            commands.insert((VehicleFlyingTag, VehicleBaseTag, NonClientBaseTag));
        }
        Class::LocalTacticNode => {
            commands
                .insert((
                    LocalTacticNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::StructureBase => {
            commands.insert((StructureBaseTag, NonClientBaseTag));
        }
        Class::CompinstMulticlusterAvatarLookup => {
            commands.insert((CompinstMulticlusterAvatarLookupTag, CompinstTag));
        }
        Class::EdnaModule => {
            commands.insert((EdnaModuleTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::CombatSystemGroup => {
            commands.insert((CombatSystemGroupTag, MetagameTag));
        }
        Class::MinigameScoreBoard => {
            commands.insert((MinigameScoreBoardTag, NonClientBaseTag));
        }
        Class::VehicleBase => {
            commands.insert((VehicleBaseTag, NonClientBaseTag));
        }
        Class::CompinstClusternode => {
            commands
                .insert((CompinstClusternodeTag, CompinstClusterappTag, CompinstTag));
        }
        Class::MypadRoomDoor => {
            commands
                .insert((
                    MypadRoomDoorTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::MoverBase => {
            commands.insert((MoverBaseTag, NonClientBaseTag));
        }
        Class::CompinstCommunitysrv => {
            commands.insert((CompinstCommunitysrvTag, CompinstTag));
        }
        Class::MyLandSettings => {
            commands
                .insert((
                    MyLandSettingsTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::ServerGatewayExitPhase => {
            commands
                .insert((
                    ServerGatewayExitPhaseTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::EdnaReceptor => {
            commands
                .insert((
                    EdnaReceptorTag,
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OtherlandStructure => {
            commands
                .insert((
                    OtherlandStructureTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::EdnaAbility => {
            commands.insert(EdnaAbilityTag);
        }
        Class::EdnaFunction => {
            commands.insert((EdnaFunctionTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Metagame => {
            commands.insert(MetagameTag);
        }
        Class::StartingPoint => {
            commands
                .insert((
                    StartingPointTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::PhysicsActor => {
            commands.insert((PhysicsActorTag, MoverBaseTag, NonClientBaseTag));
        }
        Class::ClanMember => {
            commands.insert(ClanMemberTag);
        }
        Class::PortalItem => {
            commands.insert((PortalItemTag, ItemBaseTag));
        }
        Class::QuestBeacon => {
            commands.insert((QuestBeaconTag, NonSpawnPlacementTag));
        }
        Class::LootSystem => {
            commands.insert((LootSystemTag, MetagameTag));
        }
        Class::Spawner => {
            commands.insert((SpawnerTag, SpawnerBaseTag, NonClientBaseTag));
        }
        Class::CompinstClusterapp => {
            commands.insert((CompinstClusterappTag, CompinstTag));
        }
        Class::NonClientBase => {
            commands.insert(NonClientBaseTag);
        }
        Class::BuffBase => {
            commands.insert(BuffBaseTag);
        }
        Class::CompinstMasterRedirectSrv => {
            commands.insert((CompinstMasterRedirectSrvTag, CompinstTag));
        }
        Class::WorldDisplay => {
            commands
                .insert((
                    WorldDisplayTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::ShopFilterSchema => {
            commands.insert((ShopFilterSchemaTag, OaCommonConfigTag));
        }
        Class::ItemPreset => {
            commands.insert(ItemPresetTag);
        }
        Class::CompinstDaemon => {
            commands.insert((CompinstDaemonTag, CompinstTag));
        }
        Class::CooldownGroup => {
            commands.insert(CooldownGroupTag);
        }
        Class::SpawnerBase => {
            commands.insert((SpawnerBaseTag, NonClientBaseTag));
        }
        Class::Ship => {
            commands.insert((ShipTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::AiTemplate => {
            commands.insert(AiTemplateTag);
        }
        Class::Trigger => {
            commands
                .insert((TriggerTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::NpcBase => {
            commands.insert((NpcBaseTag, NonClientBaseTag));
        }
        Class::OaZoneConfig => {
            commands.insert(OaZoneConfigTag);
        }
        Class::LifeDirector => {
            commands.insert(LifeDirectorTag);
        }
        Class::Instance => {
            commands.insert((InstanceTag, ConfigTag));
        }
        Class::JsonSchema => {
            commands.insert(JsonSchemaTag);
        }
        Class::OaBuff2 => {
            commands.insert((OaBuff2Tag, BuffBaseTag));
        }
        Class::PresetPoint => {
            commands
                .insert((
                    PresetPointTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Version => {
            commands.insert(VersionTag);
        }
        Class::Compinst => {
            commands.insert(CompinstTag);
        }
        Class::SteamDlc => {
            commands.insert(SteamDlcTag);
        }
        Class::ClanRank => {
            commands.insert(ClanRankTag);
        }
        Class::SteamItem => {
            commands.insert(SteamItemTag);
        }
        Class::CooldownGroupExternal => {
            commands
                .insert((CooldownGroupExternalTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::MinigameInfo => {
            commands.insert((MinigameInfoTag, NonClientBaseTag));
        }
        Class::StandaloneLootPartition => {
            commands.insert(StandaloneLootPartitionTag);
        }
        Class::OtherlandArea => {
            commands.insert(OtherlandAreaTag);
        }
        Class::Config => {
            commands.insert(ConfigTag);
        }
        Class::AbilityList => {
            commands.insert(AbilityListTag);
        }
        Class::Clan => {
            commands.insert(ClanTag);
        }
        Class::NpcOtherland => {
            commands.insert((NpcOtherlandTag, NpcBaseTag, NonClientBaseTag));
        }
        Class::SpawnNode => {
            commands
                .insert((
                    SpawnNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::MylandScoreboard => {
            commands.insert((MylandScoreboardTag, NonClientBaseTag));
        }
        Class::NonSpawnPlacementRadius => {
            commands.insert((NonSpawnPlacementRadiusTag, NonSpawnPlacementTag));
        }
        Class::EdnaContainer => {
            commands
                .insert((
                    EdnaContainerTag,
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Door => {
            commands.insert((DoorTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
    }
}
