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

use atlas::{oaPkt_SplineSurfing_Acknowledge, NonClientBaseComponent, NonClientBaseParams, Param, ParamBox, ParamClass, ParamSet, PlayerAttribute, PlayerClass, PlayerComponent, PlayerParams, PortalClass, PortalComponent, PortalParams, SpawnNodeClass, SpawnNodeComponent, StartingPointClass, Uuid};
use bevy::app::Plugin;
use bevy_ecs::{event::EventWriter, query::{With, Without}, system::{Commands, In, Query, Res}};
use components::{PortalExitPoint, Spawned};
use glam::{Quat, Vec3};
use log::{debug, error, info, warn};
use regex::Regex;

use crate::{actors::{get_player_height, zone::{components, plugins::{Behavior, BehaviorArguments, BehaviorExt, PlayerController, Position, ReviveEvent, ServerAction}}, AvatarComponent, DefaultPos, EntityType, SplineSurfing, UuidToEntityLookup}, util::OtherlandQuatExt};
use crate::actors::zone::FLIGHT_TUBES;

pub struct PlayerBehaviors;

impl Plugin for PlayerBehaviors {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_behavior(EntityType::Player, "RespawnNow", respawn_now);
        app.add_behavior(EntityType::Player, "PromptCooldown", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "DisableInvulnerability", disable_invulnerability);
        app.add_behavior(EntityType::Player, "CancelChannel", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "AdminRent", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "GlobalRent", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "ConvertItem", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RemoveFragmentSlot", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RedeemFragmentSlot", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestSaveShortcut", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestExchangeShortcut", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestSendToShortcut", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestDeleteShortcut", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestDeleteSlot", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestUnequip", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "SwapWeapons", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestUseItem", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestDiscardItem", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestSwitchOutfit", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "MyBank", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "IncreaseInventorySize", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "itemLevel", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "Outfit", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "ConfirmInGame", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "travel", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "ModParam", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "FlightTube", start_spline_surfing);
        app.add_behavior(EntityType::Player, "SplineTurn", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "SplineBrake", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "SplineJump", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "PlaceBall", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "LeaveGame", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestSelectWeapon", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "SplitItemStack", unimplemented_behavior);
        app.add_behavior(EntityType::Player, "RequestDecayItem", unimplemented_behavior);
    }
}

fn unimplemented_behavior(In((_, _, behavior)): In<BehaviorArguments>) {
    error!("Player behavior '{:?}' not implemented!", behavior);
}

fn respawn_now(
    In((instigator, _target, behavior)): In<BehaviorArguments>,
    mut player_query: Query<(&mut Position, &mut ParamBox, &AvatarComponent, &PlayerController), (With<PlayerComponent>, Without<NonClientBaseComponent>)>,
    portals: Query<(&ParamBox, &PortalExitPoint), (With<PortalComponent>, With<NonClientBaseComponent>, With<components::RespawnPoint>, With<Spawned>)>,
    exit_points: Query<&ParamBox, (With<SpawnNodeComponent>, With<NonClientBaseComponent>)>,
    default_pos: Res<DefaultPos>,
    uuid_to_entity: Res<UuidToEntityLookup>,
    mut ev_revive: EventWriter<ReviveEvent>,
) {
    if 
        let Behavior::String(_, args) = behavior && 
        let Some(mode) = args.get(0).map(|v| v.as_str()) 
    {
        match mode {
            "NearestPortal" => {
                let (mut player_pos, mut player, avatar, controller) = player_query.get_mut(instigator).unwrap();

                let mut positions: Vec<_> = portals.iter()
                    .filter_map(|(_, exitpoint)| {
                        info!("Got exitpoint: {}", exitpoint.0);

                        uuid_to_entity.find_entity(&exitpoint.0)
                            .and_then(|entity| exit_points.get(*entity).ok())
                    })
                    .map(|starting_point| starting_point.get_impl::<dyn NonClientBaseParams>().unwrap())
                    .map(|starting_point| (starting_point.pos(), starting_point.rot())).collect();

                positions.sort_by(|a, b| {
                    a.0.distance_squared(player_pos.position)
                        .total_cmp(&b.0.distance_squared(player_pos.position))
                });

                let (respawn_pos, respawn_rot) = if let Some((pos, rot)) = positions.first() {
                    debug!("Respawn pos: {:?}", pos);
                    (**pos + Vec3::new(0.0, 0.0, get_player_height(player.get_impl::<dyn PlayerParams>().unwrap()) / 2.0), **rot)
                } else {
                    warn!("No portal for respawning found. Moving to default location");
                    (default_pos.pos, default_pos.rot)
                };

                // teleport player
                if let Ok(player) = player.get_mut::<PlayerClass>() {
                    player.set_is_un_attackable(true);
                }

                player_pos.version = player_pos.version.wrapping_add(1);
                player_pos.position = respawn_pos;
                player_pos.rotation = Quat::from_unit_vector(respawn_rot);

                // revive player
                ev_revive.send(ReviveEvent(instigator));

                // update clients
                controller.send_server_action(ServerAction::Teleport(avatar.id, player_pos.to_owned()));
            },
            _ => error!("Respawn mode {} not implemented!", mode),
        }
    }
}

fn disable_invulnerability(
    In((instigator, _target, _behavior)): In<BehaviorArguments>,
    mut player_query: Query<&mut ParamBox, With<PlayerComponent>>,
) {
    if let Ok(mut player) = player_query.get_mut(instigator) {
        player.get_mut::<PlayerClass>().unwrap().set_is_un_attackable(false);
    }
}

fn start_spline_surfing(
    In((instigator, _target, behavior)): In<BehaviorArguments>,
    player_query: Query<(&AvatarComponent, &PlayerController), (With<PlayerComponent>, Without<SplineSurfing>)>,
    mut commands: Commands,
) {
    if 
        let Behavior::String(_, args) = behavior && 
        let Ok((avatar, controller)) = player_query.get(instigator) 
    {
        let re = Regex::new(r"SplineID=([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}) InverseTravel=([0-1]) Loc=\[ -?(\d+\.?\d*) -?(\d+\.?\d*) -?(\d+\.?\d*) \]").unwrap();
        if let Some(captures) = re.captures(&args.join(" ")) {
            let spline_id = Uuid::parse_str(&captures[1]).unwrap();
            let inverse_travel = &captures[2] == "1";
            let loc = Vec3::new(
                captures[3].parse().unwrap(), 
                captures[4].parse().unwrap(), 
                captures[5].parse().unwrap(),
            );

            // lookup flight tube
            if let Some(spline) = FLIGHT_TUBES.get().unwrap().get(&spline_id) {
                // start moving along the spline
                commands.entity(instigator).insert(SplineSurfing::new(spline.to_owned(), inverse_travel));

                controller.send_message(oaPkt_SplineSurfing_Acknowledge {
                    avatar_id: avatar.id.as_u64(),
                    spline_id,
                    acknowledged: true,
                    inverse_travel,
                    loc: loc.into(),
                    ..Default::default()
                }.into_message());
            } else {
                controller.send_message(oaPkt_SplineSurfing_Acknowledge {
                    avatar_id: avatar.id.as_u64(),
                    spline_id,
                    acknowledged: false,
                    inverse_travel,
                    loc: loc.into(),
                    ..Default::default()
                }.into_message());
            }
        }
    }
}
