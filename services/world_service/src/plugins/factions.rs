// Copyright (C) 2025 AnotherlandServer
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
use protocol::{oaPktFactionRequest, oaPktFactionResponse};
use toolkit::NativeParam;

use super::{NetworkExtPriv, PlayerController};

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_message_handler(handle_faction_request);
    }
}


fn handle_faction_request(
    In((ent, _pkt)): In<(Entity, oaPktFactionRequest)>,
    query: Query<&PlayerController>,
) {
    if let Ok(controller) = query.get(ent) {
        controller.send_packet(oaPktFactionResponse {
            field_2: 1,
            field_3: NativeParam::Struct(vec![
                NativeParam::Buffer(vec![])
            ]),
            ..Default::default()
        });
    }
}