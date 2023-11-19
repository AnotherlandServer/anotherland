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

use atlas::oaPktC2SConnectionState;
use log::warn;

use crate::{node_server::{ClientState, NodeServer, ClientLoadState}, util::AnotherlandResult};

impl NodeServer {
    pub(in crate::node_server) async fn request_c2s_connection_state(&self, state: &mut ClientState, pkt: oaPktC2SConnectionState) -> AnotherlandResult<()> {
        state.load_state = match pkt.field_1 {
            5 => ClientLoadState::RequestAvatarStream,
            6 => ClientLoadState::StreamedAvatars,
            7 => ClientLoadState::RequestSpawn,
            8 => ClientLoadState::Spawned,
            _ => {
                warn!(client = state; "Invalid client loadstate: {}", pkt.field_1);
                ClientLoadState::EarlyLoadSequence
            }
        };

        // Confirm loading state
        let mut response = pkt.clone();
        response.field_1 = state.load_state.clone().into();
        response.field_2 = pkt.field_2 + 1;

        self.send(&state.peer_id, response.as_message()).await?;
        
        Ok(())
    }
}