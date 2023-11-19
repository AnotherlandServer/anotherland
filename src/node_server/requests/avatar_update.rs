use atlas::{PlayerParam, CPktAvatarUpdate, BoundParamClass, ParamClass};
use log::debug;

use crate::{node_server::{NodeServer, ClientState}, util::AnotherlandResult};

impl NodeServer {
    pub(in crate::node_server) async fn request_avatar_update(&self, state: &mut ClientState, pkt: CPktAvatarUpdate) -> AnotherlandResult<()> {
        if pkt.avatar_id.unwrap() == state.avatar_id.as_u64() {
            if let Ok(param) = PlayerParam::read(&pkt.params) {
                //debug!("{:#?}", param.1.as_anyclass());
            }
        }

        Ok(())
    }
}