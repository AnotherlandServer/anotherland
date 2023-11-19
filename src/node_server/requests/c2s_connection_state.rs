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