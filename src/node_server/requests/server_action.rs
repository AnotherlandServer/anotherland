use atlas::oaPktServerAction;

use crate::{node_server::{NodeServer, ClientState}, util::AnotherlandResult};

impl NodeServer {
    pub(in crate::node_server) async fn request_server_action(&self, state: &mut ClientState, pkt: oaPktServerAction) -> AnotherlandResult<()> {
        let mut action = pkt.clone();
        action.version = 2;
        self.send(&state.peer_id, action.as_message()).await?;

        Ok(())
    }
}