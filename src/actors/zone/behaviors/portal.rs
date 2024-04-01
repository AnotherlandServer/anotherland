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

use atlas::{oaPktConfirmTravel, oaPktPortalRequestAck, NativeParam, ParamBox, PlayerComponent, PortalAckPartA, PortalComponent, SpawnNodeComponent, SpawnNodeParams, UUID_NIL};
use bevy::app::Plugin;
use bevy_ecs::{query::{With, Without}, system::{In, Query, Res}};
use glam::Quat;
use log::{debug, error, warn};

use crate::{actors::{get_display_name, zone::plugins::{BehaviorArguments, BehaviorExt, PlayerController, Position, ServerAction}, AvatarComponent, EntityType, PortalExitPoint, PortalNodelink, UuidToEntityLookup, PORTAL_HIVE_DESTINATIONS}, frontends::TravelType, util::OtherlandQuatExt};

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
    player: Query<&PlayerController, With<PlayerComponent>>,
) {
    // todo: Validate the player is allowed to go where they want to go
    if let Ok(controller) = player.get(instigator) {
        controller.send_message(oaPktConfirmTravel {
            state: 1,
            ..Default::default()
        }.into_message());
    }
}

fn do_travel(
    In((instigator, target, args)): In<BehaviorArguments>,
    uuid_to_entity: Res<UuidToEntityLookup>,
    mut player: Query<(&AvatarComponent, &mut Position, &PlayerController), With<PlayerComponent>>,
    portals: Query<&PortalNodelink>,
    exit_points: Query<&PortalExitPoint>,
    spawn_nodes: Query<&ParamBox, (With<SpawnNodeComponent>, Without<PlayerComponent>)>,
) {
    let (avatar, mut player_pos, controller) = player.get_mut(instigator).unwrap();

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
            controller.send_travel(*zone, TravelType::Portal { 
                uuid: *portal
            });
        },
        Some(PortalNodelink::LocalPortal(id)) => {
            debug!("Local portal travel!");

            let starting_point = uuid_to_entity.find_entity(id)
                .and_then(|ent| exit_points.get(*ent).ok())
                .and_then(|exit_point| uuid_to_entity.find_entity(&exit_point.0))
                .and_then(|ent| spawn_nodes.get(*ent).ok())
                .and_then(|p| p.get_impl::<dyn SpawnNodeParams>());

            if let Some(starting_point) = starting_point {
                player_pos.version = player_pos.version.wrapping_add(1);
                player_pos.position = *starting_point.pos();
                player_pos.rotation = Quat::from_unit_vector(*starting_point.rot());

                // update clients
                controller.send_server_action(ServerAction::LocalPortal(avatar.id, player_pos.to_owned()));
            }
        },
        None => (),
    }
}

fn interact(
    In((instigator, target, _)): In<BehaviorArguments>,
    players: Query<(&AvatarComponent, &PlayerController), With<PlayerComponent>>,
    portals: Query<&AvatarComponent, With<PortalComponent>>,
) {
    let (player_avatar, controller) = players.get(instigator).unwrap();
    let portal_avatar = portals.get(target).unwrap();

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

    controller.send_message(oaPktPortalRequestAck {
        field_1: player_avatar.id.as_u64(),
        field_2: portal_avatar.id.as_u64(),
        array_len_a: destinations.len() as u32,
        field_5: destinations,
        field_12: NativeParam::Struct(vec![
            NativeParam::String(String::default()),
            NativeParam::Bool(false),
            NativeParam::Int(0),
            NativeParam::Int(0),
        ]),
        ..Default::default()
    }.into_message());
}
