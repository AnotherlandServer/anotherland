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

use bevy::{app::Plugin, prelude::{App, Entity, In, Query}};
use protocol::{oaPktFriendRequest, CPkt, CPktStream_167_0};

use crate::plugins::NetworkExtPriv;

use super::PlayerController;

pub struct SocialPlugin;

impl Plugin for SocialPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler::<oaPktFriendRequest, _, _>(handle_oapkt_friend_request);
    }
}

fn handle_oapkt_friend_request(
    In((ent, _pkt)): In<(Entity, oaPktFriendRequest)>,
    query: Query<&PlayerController>,
) {
    if let Ok(controller) = query.get(ent) {
        controller.send_packet(CPktStream_167_0::default());
    }
}