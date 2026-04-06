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
use crate::ObjectReceiver;
#[derive(Component)]
pub struct BuffBaseTag;
#[derive(Component)]
pub struct CompinstClustersrvTag;
#[derive(Component)]
pub struct BundleItemTag;
#[derive(Component)]
pub struct ItemMyLandThemeTag;
#[derive(Component)]
pub struct AiTemplateTag;
#[derive(Component)]
pub struct InteractObjectTag;
#[derive(Component)]
pub struct ChessMetaGameLogicTag;
#[derive(Component)]
pub struct CompinstMulticlusterAvatarLookupTag;
#[derive(Component)]
pub struct StructureBaseTag;
#[derive(Component)]
pub struct CompinstCommunicationsrvTag;
#[derive(Component)]
pub struct ServerGatewayExitPhaseTag;
#[derive(Component)]
pub struct OtherlandStructureTag;
#[derive(Component)]
pub struct VehicleFlyingTag;
#[derive(Component)]
pub struct OaBuff2Tag;
#[derive(Component)]
pub struct CompinstDaemonTag;
#[derive(Component)]
pub struct StructureTag;
#[derive(Component)]
pub struct MinigameScoreBoardTag;
#[derive(Component)]
pub struct QuestBeaconTag;
#[derive(Component)]
pub struct ShopFilterSchemaTag;
#[derive(Component)]
pub struct PartyTag;
#[derive(Component)]
pub struct NpcBaseTag;
#[derive(Component)]
pub struct PhysicsActorTag;
#[derive(Component)]
pub struct CompinstClusterappTag;
#[derive(Component)]
pub struct ClanRankTag;
#[derive(Component)]
pub struct PatrolNodeTag;
#[derive(Component)]
pub struct NpcShopConfigTag;
#[derive(Component)]
pub struct EdnaBaseTag;
#[derive(Component)]
pub struct ClanTag;
#[derive(Component)]
pub struct VehicleBaseTag;
#[derive(Component)]
pub struct NonSpawnPlacementRadiusTag;
#[derive(Component)]
pub struct CompinstCommunitysrvTag;
#[derive(Component)]
pub struct GameSessionTag;
#[derive(Component)]
pub struct CompinstFrontendsrvTag;
#[derive(Component)]
pub struct SkillGroupTag;
#[derive(Component)]
pub struct CooldownGroupExternalTag;
#[derive(Component)]
pub struct StandaloneLootPartitionTag;
#[derive(Component)]
pub struct VersionTag;
#[derive(Component)]
pub struct PortalItemTag;
#[derive(Component)]
pub struct CombatSystemGroupTag;
#[derive(Component)]
pub struct ItemBaseTag;
#[derive(Component)]
pub struct EdnaContainerTag;
#[derive(Component)]
pub struct OaZoneConfigTag;
#[derive(Component)]
pub struct PortalTag;
#[derive(Component)]
pub struct CompinstTag;
#[derive(Component)]
pub struct NpcOtherlandTag;
#[derive(Component)]
pub struct ItemPresetTag;
#[derive(Component)]
pub struct EdnaModuleTag;
#[derive(Component)]
pub struct MyLandSettingsTag;
#[derive(Component)]
pub struct LootSystemTag;
#[derive(Component)]
pub struct MoverBaseTag;
#[derive(Component)]
pub struct SpawnerBaseTag;
#[derive(Component)]
pub struct DoorTag;
#[derive(Component)]
pub struct ShipTag;
#[derive(Component)]
pub struct ItemEdnaTag;
#[derive(Component)]
pub struct BilliardBallTag;
#[derive(Component)]
pub struct OtherlandAreaTag;
#[derive(Component)]
pub struct NonSpawnPlacementTag;
#[derive(Component)]
pub struct StartingPointTag;
#[derive(Component)]
pub struct InstanceTag;
#[derive(Component)]
pub struct WorldDisplayTag;
#[derive(Component)]
pub struct CtfGameFlagTag;
#[derive(Component)]
pub struct CooldownGroupTag;
#[derive(Component)]
pub struct MetagameTag;
#[derive(Component)]
pub struct NonClientBaseTag;
#[derive(Component)]
pub struct LocalTacticNodeTag;
#[derive(Component)]
pub struct JsonSchemaTag;
#[derive(Component)]
pub struct SpawnNodeTag;
#[derive(Component)]
pub struct TradeTag;
#[derive(Component)]
pub struct EdnaAbilityTag;
#[derive(Component)]
pub struct CompinstClusternodeTag;
#[derive(Component)]
pub struct MailTag;
#[derive(Component)]
pub struct PlanetTag;
#[derive(Component)]
pub struct AbilityListTag;
#[derive(Component)]
pub struct MypadRoomDoorTag;
#[derive(Component)]
pub struct CompinstMasterRedirectSrvTag;
#[derive(Component)]
pub struct MinigameItemTag;
#[derive(Component)]
pub struct TriggerTag;
#[derive(Component)]
pub struct ChessPieceTag;
#[derive(Component)]
pub struct PresetPointTag;
#[derive(Component)]
pub struct ServerGatewayTag;
#[derive(Component)]
pub struct SteamDlcTag;
#[derive(Component)]
pub struct FactionTag;
#[derive(Component)]
pub struct EdnaFunctionTag;
#[derive(Component)]
pub struct LootScatterContainerTag;
#[derive(Component)]
pub struct SomaforgeItemTag;
#[derive(Component)]
pub struct MinigameInfoTag;
#[derive(Component)]
pub struct MylandScoreboardTag;
#[derive(Component)]
pub struct CustomTriggerTag;
#[derive(Component)]
pub struct ClanMemberTag;
#[derive(Component)]
pub struct CommonConfigTag;
#[derive(Component)]
pub struct SteamItemTag;
#[derive(Component)]
pub struct ClassItemTag;
#[derive(Component)]
pub struct BuffGroupTag;
#[derive(Component)]
pub struct SpawnerTag;
#[derive(Component)]
pub struct LifeDirectorTag;
#[derive(Component)]
pub struct MinigameMineTag;
#[derive(Component)]
pub struct OaCommonConfigTag;
#[derive(Component)]
pub struct CompinstLoginsrvTag;
#[derive(Component)]
pub struct EdnaReceptorTag;
#[derive(Component)]
pub struct PlayerTag;
#[derive(Component)]
pub struct ConfigTag;
pub(crate) fn tag_gameobject_entity<T: ObjectReceiver>(class: Class, target: &mut T) {
    match class {
        Class::BuffBase => {
            target.insert(BuffBaseTag);
        }
        Class::CompinstClustersrv => {
            target.insert((CompinstClustersrvTag, CompinstClusterappTag, CompinstTag));
        }
        Class::BundleItem => {
            target.insert((BundleItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::ItemMyLandTheme => {
            target.insert((ItemMyLandThemeTag, ItemBaseTag));
        }
        Class::AiTemplate => {
            target.insert(AiTemplateTag);
        }
        Class::InteractObject => {
            target
                .insert((
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::ChessMetaGameLogic => {
            target
                .insert((
                    ChessMetaGameLogicTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstMulticlusterAvatarLookup => {
            target.insert((CompinstMulticlusterAvatarLookupTag, CompinstTag));
        }
        Class::StructureBase => {
            target.insert((StructureBaseTag, NonClientBaseTag));
        }
        Class::CompinstCommunicationsrv => {
            target.insert((CompinstCommunicationsrvTag, CompinstTag));
        }
        Class::ServerGatewayExitPhase => {
            target
                .insert((
                    ServerGatewayExitPhaseTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OtherlandStructure => {
            target
                .insert((
                    OtherlandStructureTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::VehicleFlying => {
            target.insert((VehicleFlyingTag, VehicleBaseTag, NonClientBaseTag));
        }
        Class::OaBuff2 => {
            target.insert((OaBuff2Tag, BuffBaseTag));
        }
        Class::CompinstDaemon => {
            target.insert((CompinstDaemonTag, CompinstTag));
        }
        Class::Structure => {
            target.insert((StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::MinigameScoreBoard => {
            target.insert((MinigameScoreBoardTag, NonClientBaseTag));
        }
        Class::QuestBeacon => {
            target.insert((QuestBeaconTag, NonSpawnPlacementTag));
        }
        Class::ShopFilterSchema => {
            target.insert((ShopFilterSchemaTag, OaCommonConfigTag));
        }
        Class::Party => {
            target.insert(PartyTag);
        }
        Class::NpcBase => {
            target.insert((NpcBaseTag, NonClientBaseTag));
        }
        Class::PhysicsActor => {
            target.insert((PhysicsActorTag, MoverBaseTag, NonClientBaseTag));
        }
        Class::CompinstClusterapp => {
            target.insert((CompinstClusterappTag, CompinstTag));
        }
        Class::ClanRank => {
            target.insert(ClanRankTag);
        }
        Class::PatrolNode => {
            target
                .insert((
                    PatrolNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::NpcShopConfig => {
            target.insert((NpcShopConfigTag, ConfigTag));
        }
        Class::EdnaBase => {
            target.insert((EdnaBaseTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::Clan => {
            target.insert(ClanTag);
        }
        Class::VehicleBase => {
            target.insert((VehicleBaseTag, NonClientBaseTag));
        }
        Class::NonSpawnPlacementRadius => {
            target.insert((NonSpawnPlacementRadiusTag, NonSpawnPlacementTag));
        }
        Class::CompinstCommunitysrv => {
            target.insert((CompinstCommunitysrvTag, CompinstTag));
        }
        Class::GameSession => {
            target.insert(GameSessionTag);
        }
        Class::CompinstFrontendsrv => {
            target.insert((CompinstFrontendsrvTag, CompinstTag));
        }
        Class::SkillGroup => {
            target.insert((SkillGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::CooldownGroupExternal => {
            target.insert((CooldownGroupExternalTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::StandaloneLootPartition => {
            target.insert(StandaloneLootPartitionTag);
        }
        Class::Version => {
            target.insert(VersionTag);
        }
        Class::PortalItem => {
            target.insert((PortalItemTag, ItemBaseTag));
        }
        Class::CombatSystemGroup => {
            target.insert((CombatSystemGroupTag, MetagameTag));
        }
        Class::ItemBase => {
            target.insert(ItemBaseTag);
        }
        Class::EdnaContainer => {
            target
                .insert((
                    EdnaContainerTag,
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OaZoneConfig => {
            target.insert(OaZoneConfigTag);
        }
        Class::Portal => {
            target.insert((PortalTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::Compinst => {
            target.insert(CompinstTag);
        }
        Class::NpcOtherland => {
            target.insert((NpcOtherlandTag, NpcBaseTag, NonClientBaseTag));
        }
        Class::ItemPreset => {
            target.insert(ItemPresetTag);
        }
        Class::EdnaModule => {
            target.insert((EdnaModuleTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::MyLandSettings => {
            target
                .insert((
                    MyLandSettingsTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::LootSystem => {
            target.insert((LootSystemTag, MetagameTag));
        }
        Class::MoverBase => {
            target.insert((MoverBaseTag, NonClientBaseTag));
        }
        Class::SpawnerBase => {
            target.insert((SpawnerBaseTag, NonClientBaseTag));
        }
        Class::Door => {
            target.insert((DoorTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::Ship => {
            target.insert((ShipTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::ItemEdna => {
            target.insert((ItemEdnaTag, ItemBaseTag));
        }
        Class::BilliardBall => {
            target
                .insert((
                    BilliardBallTag,
                    PhysicsActorTag,
                    MoverBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OtherlandArea => {
            target.insert(OtherlandAreaTag);
        }
        Class::NonSpawnPlacement => {
            target.insert(NonSpawnPlacementTag);
        }
        Class::StartingPoint => {
            target
                .insert((
                    StartingPointTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Instance => {
            target.insert((InstanceTag, ConfigTag));
        }
        Class::WorldDisplay => {
            target
                .insert((
                    WorldDisplayTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CtfGameFlag => {
            target
                .insert((
                    CtfGameFlagTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CooldownGroup => {
            target.insert(CooldownGroupTag);
        }
        Class::Metagame => {
            target.insert(MetagameTag);
        }
        Class::NonClientBase => {
            target.insert(NonClientBaseTag);
        }
        Class::LocalTacticNode => {
            target
                .insert((
                    LocalTacticNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::JsonSchema => {
            target.insert(JsonSchemaTag);
        }
        Class::SpawnNode => {
            target
                .insert((
                    SpawnNodeTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Trade => {
            target.insert(TradeTag);
        }
        Class::EdnaAbility => {
            target.insert(EdnaAbilityTag);
        }
        Class::CompinstClusternode => {
            target.insert((CompinstClusternodeTag, CompinstClusterappTag, CompinstTag));
        }
        Class::Mail => {
            target.insert(MailTag);
        }
        Class::Planet => {
            target.insert((PlanetTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::AbilityList => {
            target.insert(AbilityListTag);
        }
        Class::MypadRoomDoor => {
            target
                .insert((
                    MypadRoomDoorTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::CompinstMasterRedirectSrv => {
            target.insert((CompinstMasterRedirectSrvTag, CompinstTag));
        }
        Class::MinigameItem => {
            target.insert((MinigameItemTag, ItemBaseTag));
        }
        Class::Trigger => {
            target
                .insert((TriggerTag, StructureTag, StructureBaseTag, NonClientBaseTag));
        }
        Class::ChessPiece => {
            target
                .insert((
                    ChessPieceTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::PresetPoint => {
            target
                .insert((
                    PresetPointTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::ServerGateway => {
            target
                .insert((
                    ServerGatewayTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::SteamDlc => {
            target.insert(SteamDlcTag);
        }
        Class::Faction => {
            target.insert(FactionTag);
        }
        Class::EdnaFunction => {
            target.insert((EdnaFunctionTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::LootScatterContainer => {
            target
                .insert((
                    LootScatterContainerTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::SomaforgeItem => {
            target.insert((SomaforgeItemTag, ItemEdnaTag, ItemBaseTag));
        }
        Class::MinigameInfo => {
            target.insert((MinigameInfoTag, NonClientBaseTag));
        }
        Class::MylandScoreboard => {
            target.insert((MylandScoreboardTag, NonClientBaseTag));
        }
        Class::CustomTrigger => {
            target
                .insert((
                    CustomTriggerTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::ClanMember => {
            target.insert(ClanMemberTag);
        }
        Class::CommonConfig => {
            target.insert((CommonConfigTag, OaCommonConfigTag));
        }
        Class::SteamItem => {
            target.insert(SteamItemTag);
        }
        Class::ClassItem => {
            target.insert((ClassItemTag, ItemBaseTag));
        }
        Class::BuffGroup => {
            target.insert((BuffGroupTag, CombatSystemGroupTag, MetagameTag));
        }
        Class::Spawner => {
            target.insert((SpawnerTag, SpawnerBaseTag, NonClientBaseTag));
        }
        Class::LifeDirector => {
            target.insert(LifeDirectorTag);
        }
        Class::MinigameMine => {
            target
                .insert((
                    MinigameMineTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::OaCommonConfig => {
            target.insert(OaCommonConfigTag);
        }
        Class::CompinstLoginsrv => {
            target.insert((CompinstLoginsrvTag, CompinstTag));
        }
        Class::EdnaReceptor => {
            target
                .insert((
                    EdnaReceptorTag,
                    InteractObjectTag,
                    StructureTag,
                    StructureBaseTag,
                    NonClientBaseTag,
                ));
        }
        Class::Player => {
            target.insert(PlayerTag);
        }
        Class::Config => {
            target.insert(ConfigTag);
        }
    }
}
