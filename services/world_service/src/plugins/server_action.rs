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

use bevy::{app::{App, Plugin}, prelude::{Entity, In, Query}};
use log::debug;
use protocol::{oaPktServerAction, CPkt};
use toolkit::types::AvatarId;

use super::{Movement, NetworkExtPriv, PlayerController};

pub struct ServerActionPlugin;

impl Plugin for ServerActionPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_server_action_request);
    }
}

#[derive(Clone)]
pub enum ServerAction {
    DirectTravel(AvatarId, Option<Movement>),
    NonPortalTravel(AvatarId, Option<Movement>),
    Portal(AvatarId, Option<Movement>),
    LocalPortal(AvatarId, Movement),
    Teleport(AvatarId, Movement),
    Event(String),
}

impl ServerAction {
    pub fn into_pkt(self) -> oaPktServerAction {
        let (instigator, action, version, teleport_override) = match self {
            Self::DirectTravel(instigator, teleport_override) => (
                instigator,
                "TRAVEL:DirectTravel|DirectTravelDefault".to_owned(),
                4,
                teleport_override
            ),
            Self::NonPortalTravel(instigator, teleport_override) => (
                instigator,
                "TRAVEL:NonPortalTravel|NonPortalTravelDefault".to_owned(),
                4,
                teleport_override
            ),
            Self::Portal(instigator, teleport_override) => (
                instigator,
                "TRAVEL:DirectTravel|PortalArriveDefault".to_owned(),
                4,
                teleport_override
            ),
            Self::LocalPortal(instigator, teleport_override) => (
                instigator,
                "TRAVEL:LocalPortalArrive|PortalArriveDefault".to_owned(),
                4,
                Some(teleport_override)
            ),
            Self::Teleport(instigator, position) => (
                instigator,
                "TELEPORT:TeleportTravel|TeleportTravelDefault".to_owned(),
                4,
                Some(position)
            ),
            Self::Event(event) => (
                AvatarId::default(),
                event,
                4,
                None,
            )
        };

        if let Some(teleport_override) = teleport_override {
            oaPktServerAction {
                instigator,
                action,
                version,
                override_teleport: true,
                pos: teleport_override.position.into(),
                rot: teleport_override.rotation.into(),
                ..Default::default()
            }
        } else {
            oaPktServerAction {
                instigator,
                action,
                version,
                ..Default::default()
            }
        }
    }
}

pub fn handle_server_action_request(
    In((ent, mut pkt)): In<(Entity, oaPktServerAction)>,
    query: Query<&PlayerController>,
) {
    if let Ok(controller) = query.get(ent) {
        // TODO:
        // This should be verified in the future. Until then we just
        // set version to 2, accepting the action that way.
        pkt.version = 2;

        controller.send_packet(pkt);
    }
}