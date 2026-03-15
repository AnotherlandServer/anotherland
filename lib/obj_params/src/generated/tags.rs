// Copyright (C) 2026 AnotherlandServer
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
pub struct AiTemplateTag;
#[derive(Component)]
pub struct InstanceTag;
#[derive(Component)]
pub struct CooldownGroupExternalTag;
#[derive(Component)]
pub struct TriggerTag;
#[derive(Component)]
pub struct LocalTacticNodeTag;
#[derive(Component)]
pub struct BilliardBallTag;
#[derive(Component)]
pub struct ServerGatewayTag;
#[derive(Component)]
pub struct MinigameScoreBoardTag;
#[derive(Component)]
pub struct MinigameItemTag;
#[derive(Component)]
pub struct PortalTag;
#[derive(Component)]
pub struct PartyTag;
#[derive(Component)]
pub struct SteamDlcTag;
#[derive(Component)]
pub struct StandaloneLootPartitionTag;
#[derive(Component)]
pub struct NpcShopConfigTag;
#[derive(Component)]
pub struct SpawnerBaseTag;
#[derive(Component)]
pub struct ItemMyLandThemeTag;
#[derive(Component)]
pub struct EdnaContainerTag;
#[derive(Component)]
pub struct SpawnerTag;
#[derive(Component)]
pub struct CompinstMasterRedirectSrvTag;
#[derive(Component)]
pub struct EdnaModuleTag;
#[derive(Component)]
pub struct PresetPointTag;
#[derive(Component)]
pub struct ChessMetaGameLogicTag;
#[derive(Component)]
pub struct VehicleBaseTag;
#[derive(Component)]
pub struct CompinstCommunitysrvTag;
#[derive(Component)]
pub struct ItemBaseTag;
#[derive(Component)]
pub struct PortalItemTag;
#[derive(Component)]
pub struct CombatSystemGroupTag;
#[derive(Component)]
pub struct NonSpawnPlacementTag;
#[derive(Component)]
pub struct SomaforgeItemTag;
#[derive(Component)]
pub struct ClanTag;
#[derive(Component)]
pub struct ItemEdnaTag;
#[derive(Component)]
pub struct MyLandSettingsTag;
#[derive(Component)]
pub struct PhysicsActorTag;
#[derive(Component)]
pub struct OaZoneConfigTag;
#[derive(Component)]
pub struct BuffBaseTag;
#[derive(Component)]
pub struct CompinstClustersrvTag;
#[derive(Component)]
pub struct ItemPresetTag;
#[derive(Component)]
pub struct ClassItemTag;
#[derive(Component)]
pub struct ClanRankTag;
#[derive(Component)]
pub struct ClanMemberTag;
#[derive(Component)]
pub struct MetagameTag;
#[derive(Component)]
pub struct NpcOtherlandTag;
#[derive(Component)]
pub struct VersionTag;
#[derive(Component)]
pub struct CommonConfigTag;
#[derive(Component)]
pub struct AbilityListTag;
#[derive(Component)]
pub struct CompinstMulticlusterAvatarLookupTag;
#[derive(Component)]
pub struct TradeTag;
#[derive(Component)]
pub struct SkillGroupTag;
#[derive(Component)]
pub struct StructureBaseTag;
#[derive(Component)]
pub struct StartingPointTag;
#[derive(Component)]
pub struct CustomTriggerTag;
#[derive(Component)]
pub struct NonClientBaseTag;
#[derive(Component)]
pub struct LootSystemTag;
#[derive(Component)]
pub struct ShopFilterSchemaTag;
#[derive(Component)]
pub struct NonSpawnPlacementRadiusTag;
#[derive(Component)]
pub struct CompinstLoginsrvTag;
#[derive(Component)]
pub struct ServerGatewayExitPhaseTag;
#[derive(Component)]
pub struct CompinstTag;
#[derive(Component)]
pub struct SteamItemTag;
#[derive(Component)]
pub struct LootScatterContainerTag;
#[derive(Component)]
pub struct CompinstClusternodeTag;
#[derive(Component)]
pub struct ShipTag;
#[derive(Component)]
pub struct MinigameInfoTag;
#[derive(Component)]
pub struct CompinstCommunicationsrvTag;
#[derive(Component)]
pub struct MylandScoreboardTag;
#[derive(Component)]
pub struct CompinstClusterappTag;
#[derive(Component)]
pub struct WorldDisplayTag;
#[derive(Component)]
pub struct MoverBaseTag;
#[derive(Component)]
pub struct LifeDirectorTag;
#[derive(Component)]
pub struct EdnaBaseTag;
#[derive(Component)]
pub struct DoorTag;
#[derive(Component)]
pub struct JsonSchemaTag;
#[derive(Component)]
pub struct MypadRoomDoorTag;
#[derive(Component)]
pub struct NpcBaseTag;
#[derive(Component)]
pub struct OaCommonConfigTag;
#[derive(Component)]
pub struct OaBuff2Tag;
#[derive(Component)]
pub struct EdnaReceptorTag;
#[derive(Component)]
pub struct SpawnNodeTag;
#[derive(Component)]
pub struct PlayerTag;
#[derive(Component)]
pub struct QuestBeaconTag;
#[derive(Component)]
pub struct MinigameMineTag;
#[derive(Component)]
pub struct OtherlandStructureTag;
#[derive(Component)]
pub struct MailTag;
#[derive(Component)]
pub struct CtfGameFlagTag;
#[derive(Component)]
pub struct CompinstFrontendsrvTag;
#[derive(Component)]
pub struct VehicleFlyingTag;
#[derive(Component)]
pub struct BundleItemTag;
#[derive(Component)]
pub struct CompinstDaemonTag;
#[derive(Component)]
pub struct EdnaFunctionTag;
#[derive(Component)]
pub struct ConfigTag;
#[derive(Component)]
pub struct BuffGroupTag;
#[derive(Component)]
pub struct StructureTag;
#[derive(Component)]
pub struct PlanetTag;
#[derive(Component)]
pub struct EdnaAbilityTag;
#[derive(Component)]
pub struct InteractObjectTag;
#[derive(Component)]
pub struct CooldownGroupTag;
#[derive(Component)]
pub struct ChessPieceTag;
#[derive(Component)]
pub struct PatrolNodeTag;
#[derive(Component)]
pub struct OtherlandAreaTag;
#[derive(Component)]
pub struct GameSessionTag;
#[derive(Component)]
pub struct FactionTag;
pub fn tag_gameobject_entity(data: &GameObjectData, commands: &mut EntityCommands<'_>) {
    match data.class() {
        Class::AiTemplate => {
            commands.insert(AiTemplateTag);
        }
        Class::Instance => {
            commands.insert((InstanceTag, ConfigTag));
        }
        Class::CooldownGroupExternal => {
            commands
                .insert((CooldownGroupExternalTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::Trigger => {
            commands
                .insert((TriggerTag, StructureTag, StructureBaseTag, NonClientBaseTag));
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
        Class::BilliardBall => {
            commands
                .insert((
                    BilliardBallTag,
                    PhysicsActorTag,
                    MoverBaseTag,
                    NonClientBaseTag,
                ));
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
        Class::MinigameScoreBoard => {
            commands.insert((MinigameScoreBoardTag, NonClientBaseTag));
        }
        Class::MinigameItem => {
            commands.insert((MinigameItemTag, ItemBaseTag));
        }
        Class::Portal => {
            commands
                .insert((PortalTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::Party => {
            commands.insert(PartyTag);
        }
        Class::SteamDlc => {
            commands.insert(SteamDlcTag);
        }
        Class::StandaloneLootPartition => {
            commands.insert(StandaloneLootPartitionTag);
        }
        Class::NpcShopConfig => {
            commands.insert((NpcShopConfigTag, ConfigTag));
        }
        Class::SpawnerBase => {
            commands.insert((SpawnerBaseTag, NonClientBaseTag));
        }
        Class::ItemMyLandTheme => {
            commands.insert((ItemMyLandThemeTag, ItemBaseTag));
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
        Class::Spawner => {
            commands.insert((SpawnerTag, SpawnerBaseTag, NonClientBaseTag));
        }
        Class::CompinstMasterRedirectSrv => {
            commands.insert((CompinstMasterRedirectSrvTag, CompinstTag));
        }
        Class::EdnaModule => {
            commands.insert((EdnaModuleTag, ItemEdnaTag, ItemBaseTag));
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
        Class::ChessMetaGameLogic => {
            commands
                .insert((
                    ChessMetaGameLogicTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::VehicleBase => {
            commands.insert((VehicleBaseTag, NonClientBaseTag));
        }
        Class::CompinstCommunitysrv => {
            commands.insert((CompinstCommunitysrvTag, CompinstTag));
        }
        Class::ItemBase => {
            commands.insert(ItemBaseTag);
        }
        Class::PortalItem => {
            commands.insert((PortalItemTag, ItemBaseTag));
        }
        Class::CombatSystemGroup => {
            commands.insert((CombatSystemGroupTag, MetagameTag));
        }
        Class::NonSpawnPlacement => {
            commands.insert(NonSpawnPlacementTag);
        }
        Class::SomaforgeItem => {
            commands.insert((SomaforgeItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Clan => {
            commands.insert(ClanTag);
        }
        Class::ItemEdna => {
            commands.insert((ItemEdnaTag, ItemBaseTag));
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
        Class::PhysicsActor => {
            commands.insert((PhysicsActorTag, MoverBaseTag, NonClientBaseTag));
        }
        Class::OaZoneConfig => {
            commands.insert(OaZoneConfigTag);
        }
        Class::BuffBase => {
            commands.insert(BuffBaseTag);
        }
        Class::CompinstClustersrv => {
            commands.insert((CompinstClustersrvTag, CompinstClusterappTag, CompinstTag));
        }
        Class::ItemPreset => {
            commands.insert(ItemPresetTag);
        }
        Class::ClassItem => {
            commands.insert((ClassItemTag, ItemBaseTag));
        }
        Class::ClanRank => {
            commands.insert(ClanRankTag);
        }
        Class::ClanMember => {
            commands.insert(ClanMemberTag);
        }
        Class::Metagame => {
            commands.insert(MetagameTag);
        }
        Class::NpcOtherland => {
            commands.insert((NpcOtherlandTag, NpcBaseTag, NonClientBaseTag));
        }
        Class::Version => {
            commands.insert(VersionTag);
        }
        Class::CommonConfig => {
            commands.insert((CommonConfigTag, OaCommonConfigTag));
        }
        Class::AbilityList => {
            commands.insert(AbilityListTag);
        }
        Class::CompinstMulticlusterAvatarLookup => {
            commands.insert((CompinstMulticlusterAvatarLookupTag, CompinstTag));
        }
        Class::Trade => {
            commands.insert(TradeTag);
        }
        Class::SkillGroup => {
            commands.insert((SkillGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::StructureBase => {
            commands.insert((StructureBaseTag, NonClientBaseTag));
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
        Class::CustomTrigger => {
            commands
                .insert((
                    CustomTriggerTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::NonClientBase => {
            commands.insert(NonClientBaseTag);
        }
        Class::LootSystem => {
            commands.insert((LootSystemTag, MetagameTag));
        }
        Class::ShopFilterSchema => {
            commands.insert((ShopFilterSchemaTag, OaCommonConfigTag));
        }
        Class::NonSpawnPlacementRadius => {
            commands.insert((NonSpawnPlacementRadiusTag, NonSpawnPlacementTag));
        }
        Class::CompinstLoginsrv => {
            commands.insert((CompinstLoginsrvTag, CompinstTag));
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
        Class::Compinst => {
            commands.insert(CompinstTag);
        }
        Class::SteamItem => {
            commands.insert(SteamItemTag);
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
        Class::CompinstClusternode => {
            commands
                .insert((CompinstClusternodeTag, CompinstClusterappTag, CompinstTag));
        }
        Class::Ship => {
            commands.insert((ShipTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::MinigameInfo => {
            commands.insert((MinigameInfoTag, NonClientBaseTag));
        }
        Class::CompinstCommunicationsrv => {
            commands.insert((CompinstCommunicationsrvTag, CompinstTag));
        }
        Class::MylandScoreboard => {
            commands.insert((MylandScoreboardTag, NonClientBaseTag));
        }
        Class::CompinstClusterapp => {
            commands.insert((CompinstClusterappTag, CompinstTag));
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
        Class::MoverBase => {
            commands.insert((MoverBaseTag, NonClientBaseTag));
        }
        Class::LifeDirector => {
            commands.insert(LifeDirectorTag);
        }
        Class::EdnaBase => {
            commands.insert((EdnaBaseTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Door => {
            commands.insert((DoorTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::JsonSchema => {
            commands.insert(JsonSchemaTag);
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
        Class::NpcBase => {
            commands.insert((NpcBaseTag, NonClientBaseTag));
        }
        Class::OaCommonConfig => {
            commands.insert(OaCommonConfigTag);
        }
        Class::OaBuff2 => {
            commands.insert((OaBuff2Tag, BuffBaseTag));
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
        Class::SpawnNode => {
            commands
                .insert((
                    SpawnNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Player => {
            commands.insert(PlayerTag);
        }
        Class::QuestBeacon => {
            commands.insert((QuestBeaconTag, NonSpawnPlacementTag));
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
        Class::OtherlandStructure => {
            commands
                .insert((
                    OtherlandStructureTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Mail => {
            commands.insert(MailTag);
        }
        Class::CtfGameFlag => {
            commands
                .insert((
                    CtfGameFlagTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstFrontendsrv => {
            commands.insert((CompinstFrontendsrvTag, CompinstTag));
        }
        Class::VehicleFlying => {
            commands.insert((VehicleFlyingTag, VehicleBaseTag, NonClientBaseTag));
        }
        Class::BundleItem => {
            commands.insert((BundleItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::CompinstDaemon => {
            commands.insert((CompinstDaemonTag, CompinstTag));
        }
        Class::EdnaFunction => {
            commands.insert((EdnaFunctionTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Config => {
            commands.insert(ConfigTag);
        }
        Class::BuffGroup => {
            commands.insert((BuffGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::Structure => {
            commands.insert((StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::Planet => {
            commands
                .insert((PlanetTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::EdnaAbility => {
            commands.insert(EdnaAbilityTag);
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
        Class::CooldownGroup => {
            commands.insert(CooldownGroupTag);
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
        Class::PatrolNode => {
            commands
                .insert((
                    PatrolNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OtherlandArea => {
            commands.insert(OtherlandAreaTag);
        }
        Class::GameSession => {
            commands.insert(GameSessionTag);
        }
        Class::Faction => {
            commands.insert(FactionTag);
        }
    }
}
