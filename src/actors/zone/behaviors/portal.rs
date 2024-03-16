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

use atlas::{oaPktConfirmTravel, oaPktPortalRequestAck, NativeParam, NonClientBaseParams, Param, ParamSet, PlayerAttribute, PlayerClass, PortalAckPartA, PortalClass, SpawnNodeClass, StartingPointClass, UUID_NIL};
use bevy::app::Plugin;
use bevy_ecs::{event::EventWriter, system::{In, Query, Res}};
use glam::{Quat, Vec3};
use log::{debug, error, warn};
use atlas::ParamClass;

use crate::{actors::{get_display_name, zone::{plugins::{BehaviorArguments, BehaviorExt}, resources::Broadcaster, zone_events::AvatarEventFired}, AvatarComponent, AvatarEvent, EntityType, Movement, PhysicsState, PortalExitPoint, PortalNodelink, Position, ServerAction, UuidToEntityLookup, ZoneEvent, DISPLAY_NAMES, PORTAL_HIVE_DESTINATIONS}, frontends::TravelType, util::OtherlandQuatExt};

pub struct PortalBehaviors;

impl Plugin for PortalBehaviors {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_behavior(EntityType::Portal, "portalDelete", unimplemented_behavior);
        app.add_behavior(EntityType::Portal, "portalSave", unimplemented_behavior);
        app.add_behavior(EntityType::Portal, "ConfirmTravelRequest", confirm_travel_request);
        app.add_behavior(EntityType::Portal, "DoTravel", do_travel);
        app.add_behavior(EntityType::Portal, "interact", interact);
    }
}

fn unimplemented_behavior(In((_, _, behavior)): In<BehaviorArguments>) {
    error!("Portal behavior '{}' not implemented!", behavior.join(" "));
}

fn confirm_travel_request(
    In((instigator, _, _)): In<BehaviorArguments>,
    mut ev_sender: EventWriter<AvatarEventFired>,
) {
    // todo: Validate the player is allowed to go where they want to go
    ev_sender.send(AvatarEventFired(instigator, AvatarEvent::Message(oaPktConfirmTravel {
        state: 1,
        ..Default::default()
    }.into_message())));
}

fn do_travel(
    In((instigator, target, args)): In<BehaviorArguments>,
    mut ev_sender: EventWriter<AvatarEventFired>,
    uuid_to_entity: Res<UuidToEntityLookup>,
    broadcaster: Res<Broadcaster>,
    mut player: Query<(&mut PlayerClass, &AvatarComponent, &mut Position)>,
    portals: Query<&PortalNodelink>,
    exit_points: Query<&PortalExitPoint>,
    spawn_nodes: Query<&SpawnNodeClass>,
) {
    let nodelink = if let Some(destination) = args.get(1) {
        if let Some(dest) = PORTAL_HIVE_DESTINATIONS.get().unwrap().get(destination) {
            Some(&dest.link)
        } else {
            warn!("Portal destination {} not found!", destination);
            None
        }
    } else if let Ok(nodelink) = portals.get(target) {
        Some(nodelink)
    } else {
        warn!("No nodelink for portal set");
        None
    };

    match nodelink {
        Some(PortalNodelink::RemotePortal { zone, portal }) => {
            ev_sender.send(AvatarEventFired(instigator, AvatarEvent::Travel { 
                zone: *zone, 
                destination: TravelType::Portal { 
                    uuid: *portal
                }
            }));
        },
        Some(PortalNodelink::LocalPortal(id)) => {
            debug!("Local portal travel!");

            let (mut player, avatar, mut player_pos) = player.get_mut(instigator).unwrap();

            let starting_point = uuid_to_entity.find_entity(id)
                .and_then(|ent| exit_points.get(*ent).ok())
                .and_then(|exit_point| uuid_to_entity.find_entity(&exit_point.0))
                .and_then(|ent| spawn_nodes.get(*ent).ok());

            if let Some(starting_point) = starting_point {
                let mut update = ParamSet::<PlayerAttribute>::new();
                update.insert(PlayerAttribute::Pos, Param::Vector3Uts((0, *starting_point.pos())));
                update.insert(PlayerAttribute::Rot, *starting_point.rot());

                player.apply(update.clone());

                player_pos.version = player_pos.version.wrapping_add(1);
                    player_pos.position = *starting_point.pos();
                    player_pos.rotation = Quat::from_unit_vector(*starting_point.rot());

                // update clients
                ev_sender.send(AvatarEventFired(instigator, AvatarEvent::ServerAction(
                    ServerAction::LocalPortal(avatar.id, player_pos.to_owned()))
                ));

                let _ = broadcaster.sender.send(Arc::new(ZoneEvent::AvatarMoved { 
                    avatar_id: avatar.id, 
                    movement: Movement { 
                        position: *starting_point.pos(), 
                        rotation: Quat::from_unit_vector(*starting_point.rot()), 
                        velocity: Vec3::default(), 
                        physics_state: PhysicsState::Walking, 
                        mover_key: 0, 
                        seconds: 0.0 
                    } 
                }));
            }
        },
        None => (),
    }
}

fn interact(
    In((instigator, target, _)): In<BehaviorArguments>,
    mut ev_sender: EventWriter<AvatarEventFired>,
    avatars: Query<&AvatarComponent>,
    portal: Query<&PortalClass>,
) {
    let portal = portal.get(target).unwrap();
    let instigator_avatar = avatars.get(instigator).unwrap();
    let target_avatar = avatars.get(target).unwrap();

    let destinations: Vec<_> = PORTAL_HIVE_DESTINATIONS.get().unwrap().iter().map(|(name, dest)| {
        PortalAckPartA {
            map_name: name.clone(),
            world_name: dest.world_name.to_string(),
            display_name: if dest.display_name == UUID_NIL {
                dest.name.clone()
            } else {
                get_display_name(dest.display_name).to_string()
            },
            world_texture: "UI_Otherland.PortalIcon.PortalUSpace".to_string(),
            portal_texture: "UI_Otherland.PortalIcon.PortalUSpace".to_string(),
            description: String::default(),
            level: 0,
            ..Default::default()
        }
    }).collect();
    
    ev_sender.send(AvatarEventFired(instigator, AvatarEvent::Message(oaPktPortalRequestAck {
        field_1: instigator_avatar.id.as_u64(),
        field_2: target_avatar.id.as_u64(),
        array_len_a: destinations.len() as u32,
        field_5: destinations,
        field_12: NativeParam::Struct(vec![
            NativeParam::String(String::default()),
            NativeParam::Bool(false),
            NativeParam::Int(0),
            NativeParam::Int(0),
        ]),
        ..Default::default()
    }.into_message())));
}
