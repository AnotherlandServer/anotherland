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

use atlas::{oaPktServerAction, oaPktSteamMicroTxn, raknet::Message, AvatarId, NativeParam, Uuid};
use bevy_ecs::component::Component;
use tokio::sync::mpsc::UnboundedSender;

use crate::{actors::zone::plugins::Position, frontends::TravelType};

pub enum ServerAction {
    DirectTravel(AvatarId, Option<Position>),
    NonPortalTravel(AvatarId, Option<Position>),
    Portal(AvatarId, Option<Position>),
    LocalPortal(AvatarId, Position),
    Teleport(AvatarId, Position),
}


impl ServerAction {
    pub fn into_message(self) -> Message {
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
        };

        if let Some(teleport_override) = teleport_override {
            oaPktServerAction {
                instigator: instigator.as_u64(),
                action,
                version,
                override_teleport: true,
                pos: teleport_override.position.into(),
                rot: teleport_override.rotation.into(),
                ..Default::default()
            }.into_message()
        } else {
            oaPktServerAction {
                instigator: instigator.as_u64(),
                action,
                version,
                ..Default::default()
            }.into_message()
        }
    }
}

pub enum AvatarEvent {
    InterestAdded(Vec<AvatarId>),
    InterestRemoved(Vec<AvatarId>),
    Travel { zone: Uuid, destination: TravelType },
    Message(Message),
    ServerAction(ServerAction),
}

#[derive(Component)]
pub struct PlayerController {
    avatar_id: AvatarId,
    sender: UnboundedSender<AvatarEvent>,
}

impl PlayerController {
    pub fn new(id: AvatarId, sender: UnboundedSender<AvatarEvent>) -> Self {
        Self {
            avatar_id: id,
            sender,
        }
    }

    pub fn send_message(&self, message: Message) {
        let _ = self.sender.send(AvatarEvent::Message(message));
    }

    pub fn send_shopping_result(&self, msg: &str) {
        self.send_message(oaPktSteamMicroTxn {
            field_1: self.avatar_id.as_u64(),
            field_2: 1,
            field_3: NativeParam::Struct(vec![
                NativeParam::LongLong(0),
                NativeParam::Bool(false),
                NativeParam::String(msg.to_owned()),
            ]),
            ..Default::default()
        }.into_message());
    }

    pub fn send_server_action(&self, action: ServerAction) {
        let _ = self.sender.send(AvatarEvent::ServerAction(action));
    }

    pub fn send_interests_added(&self, ids: Vec<AvatarId>) {
        let _ = self.sender.send(AvatarEvent::InterestAdded(ids));
    }

    pub fn send_interests_removed(&self, ids: Vec<AvatarId>) {
        let _ = self.sender.send(AvatarEvent::InterestRemoved(ids));
    }

    pub fn send_travel(&self, zone: Uuid, destination: TravelType) {
        let _ = self.sender.send(AvatarEvent::Travel { zone, destination });
    }
}