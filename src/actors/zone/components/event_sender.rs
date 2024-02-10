// Copyright (C) 2023 AnotherlandServer
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

use atlas::{raknet::Message, AvatarId, Uuid};

use tokio::sync::mpsc;
use bevy_ecs::prelude::*;

use crate::{actors::{ProximityChatRange, ServerAction}, frontends::TravelType};

pub enum AvatarEvent {
    InterestAdded { ids: Vec<AvatarId> },
    InterestRemoved { ids: Vec<AvatarId> },
    Travel { zone: Uuid, destination: TravelType },
    Message(Message),
    ServerAction(ServerAction),
    ChatMessage { range: ProximityChatRange, sender: String, message: String },
}

#[derive(Component)]
pub struct AvatarEventSender(pub mpsc::Sender<AvatarEvent>);