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

use atlas::{oaPktConfirmTravel, oaPktPortalRequestAck, NativeParam, PortalAckPartA, PortalClass, UUID_NIL};
use bevy::app::Plugin;
use bevy_ecs::{entity::Entity, event::EventWriter, system::{In, Query}};
use log::{error, warn};

use crate::{actors::{get_display_name, zone::{plugins::{BehaviorArguments, BehaviorExt}, zone_events::AvatarEventFired}, AvatarComponent, AvatarEvent, EntityType, PortalNodelink, DISPLAY_NAMES, PORTAL_HIVE_DESTINATIONS}, frontends::TravelType};

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
    portal: Query<&PortalNodelink>,
) {
    if let Some(destination) = args.get(1) {
        if let Some(dest) = PORTAL_HIVE_DESTINATIONS.get().unwrap().get(destination) {
            travel_to_link(instigator, &mut ev_sender, &dest.link);
        } else {
            warn!("Portal destination {} not found!", destination);
        }
    } else if let Ok(nodelink) = portal.get(target) {
        travel_to_link(instigator, &mut ev_sender, nodelink);
    } else {
        warn!("No nodelink for portal set");
    }
}

fn travel_to_link(instigator: Entity, sender: &mut EventWriter<AvatarEventFired>, link: &PortalNodelink) {
    match link {
        PortalNodelink::RemotePortal { zone, portal } => {
            sender.send(AvatarEventFired(instigator, AvatarEvent::Travel { 
                zone: *zone, 
                destination: TravelType::Portal { 
                    uuid: *portal
                }
            }));
        }
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
