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

use atlas::{NonClientBaseParams, Param, ParamClass, ParamSet, PlayerAttribute, PlayerClass, PlayerParams, StartingPointClass};
use bevy::app::Plugin;
use bevy_ecs::{entity::Entity, event::EventWriter, query::{With, Without}, system::{In, Query, Res}};
use glam::{Quat, Vec3};
use log::{debug, error, warn};

use crate::{actors::{get_player_height, zone::{behavior::{BehaviorArguments, BehaviorExt}, resources::Broadcaster, zone_events::AvatarEventFired}, AvatarComponent, AvatarEvent, DefaultPos, EntityType, Movement, PhysicsState, Position, ServerAction, ZoneEvent}, util::OtherlandQuatExt};

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
    error!("Player behavior '{}' not implemented!", behavior.join(" "));
}

fn respawn_now(
    In((instigator, target, behavior)): In<BehaviorArguments>,
    mut player_query: Query<(&mut Position, &mut PlayerClass, &AvatarComponent), Without<StartingPointClass>>,
    starting_points: Query<&StartingPointClass, With<StartingPointClass>>,
    default_pos: Res<DefaultPos>,
    broadcaster: Res<Broadcaster>,
    mut ev_sender: EventWriter<AvatarEventFired>,
) {
    if let Some(mode) = behavior.get(1).map(|v| v.as_str()) {
        match mode {
            "NearestPortal" => {
                let (mut player_pos, mut player, avatar) = player_query.get_mut(instigator).unwrap();

                let mut positions: Vec<_> = starting_points.iter()
                    .map(|starting_point| (starting_point.pos(), starting_point.rot())).collect();

                positions.sort_by(|a, b| {
                    a.0.distance_squared(player_pos.position)
                        .total_cmp(&b.0.distance_squared(player_pos.position))
                });

                let (respawn_pos, respawn_rot) = if let Some((pos, rot)) = positions.first() {
                    debug!("Respawn pos: {:?}", pos);
                    (**pos + Vec3::new(0.0, 0.0, get_player_height(&*player) / 2.0), **rot)
                } else {
                    warn!("No portal for respawning found. Moving to default location");
                    (default_pos.pos, default_pos.rot)
                };

                // revive & teleport player
                let mut update = ParamSet::<PlayerAttribute>::new();
                update.insert(PlayerAttribute::Alive, true);
                update.insert(PlayerAttribute::HpCur, player.hp_max());
                update.insert(PlayerAttribute::Pos, Param::Vector3Uts((0, respawn_pos)));
                update.insert(PlayerAttribute::Rot, respawn_rot);
                update.insert(PlayerAttribute::IsUnAttackable, true); 
    
                player.apply(update.clone());

                player_pos.version = player_pos.version.wrapping_add(1);
                    player_pos.position = respawn_pos;
                    player_pos.rotation = Quat::from_unit_vector(respawn_rot);

                // update clients
                ev_sender.send(AvatarEventFired(instigator, AvatarEvent::ServerAction(
                    ServerAction::Teleport(avatar.id, player_pos.to_owned()))
                ));

                let _ = broadcaster.sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                    avatar_id: avatar.id, 
                    movement: Movement { 
                        position: respawn_pos, 
                        rotation: Quat::from_unit_vector(respawn_rot), 
                        velocity: Vec3::default(), 
                        physics_state: PhysicsState::Walking, 
                        mover_key: 0, 
                        seconds: 0.0 
                    } 
                }));

                let _ = broadcaster.sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                    avatar_id: avatar.id, 
                    params: update.into_box(),
                }));

                let _ = broadcaster.sender.send(Arc::new(ZoneEvent::CombatHpUpdate { 
                    avatar_id: avatar.id, 
                    hp: player.hp_cur(),
                }));
            },
            _ => error!("Respawn mode {} not implemented!", mode),
        }
    }
}

fn disable_invulnerability(
    In((instigator, target, behavior)): In<BehaviorArguments>,
    mut player_query: Query<(&mut PlayerClass, &AvatarComponent)>,
    broadcaster: Res<Broadcaster>,
) {
    if let Ok((mut player, avatar)) = player_query.get_mut(instigator) {
        let mut update = ParamSet::<PlayerAttribute>::new();

        update.insert(PlayerAttribute::IsUnAttackable, false);
        player.apply(update.clone());

        let _ = broadcaster.sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
            avatar_id: avatar.id, 
            params: update.into_box(),
        }));
    }
}