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

use atlas::oaPktConfirmTravel;
use bevy::app::Plugin;
use bevy_ecs::{event::EventWriter, system::{In, Query}};
use log::{error, warn};

use crate::{actors::{zone::{behavior::{BehaviorArguments, BehaviorExt}, zone_events::AvatarEventFired}, AvatarEvent, EntityType, PortalNodelink}, frontends::TravelType};

pub struct PortalBehaviors;

impl Plugin for PortalBehaviors {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_behavior(EntityType::Portal, "portalDelete", unimplemented_behavior);
        app.add_behavior(EntityType::Portal, "portalSave", unimplemented_behavior);
        app.add_behavior(EntityType::Portal, "ConfirmTravelRequest", confirm_travel_request);
        app.add_behavior(EntityType::Portal, "DoTravel", do_travel);
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
    In((instigator, target, _)): In<BehaviorArguments>,
    mut ev_sender: EventWriter<AvatarEventFired>,
    portal: Query<&PortalNodelink>,
) {
    if let Ok(nodelink) = portal.get(target) {
        match nodelink {
            PortalNodelink::RemotePortal { zone, portal } => {
                ev_sender.send(AvatarEventFired(instigator, AvatarEvent::Travel { 
                    zone: *zone, 
                    destination: TravelType::Portal { 
                        uuid: *portal
                    }
                }));
            }
        }
        
    } else {
        warn!("No nodelink for portal set");
    }
}