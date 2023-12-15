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

use super::{Message, RakNetPeerHandle};

pub struct RakNetRequest {
    peer: RakNetPeerHandle,
    message: Message,
}

impl RakNetRequest {
    pub fn new(peer: RakNetPeerHandle, message: Message) -> Self {
        Self {
            peer,
            message
        }
    }

    pub fn peer(&self) -> RakNetPeerHandle {
        self.peer.clone()
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}