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

use core_api::{CoreApi, Session};
use log::{debug, error};
use protocol::CPkt;
use raknet::RakNetSocket;
use realm_api::RealmApi;

use crate::error::FrontendError;

pub struct FrontendSessionContext {
    core_api: CoreApi,
    realm_api: RealmApi,
    socket: RakNetSocket,
}

impl FrontendSessionContext {
    pub fn start_frontend_session(
        core_api: CoreApi, 
        realm_api: RealmApi, 
        socket: RakNetSocket
    ) {
        let mut context = Self {
            core_api,
            realm_api,
            socket,
        };

        let mut session = None;

        tokio::spawn(async move {
            loop {
                match context.socket.recv().await {
                    Ok(buf) => {
                        if let Ok((_, pkt)) = CPkt::from_bytes(&buf) {
                            if let Some(session) = session.as_ref() {
                                if let Err(e) = context.handle(pkt, session).await {
                                    error!("Message handler error: {:?}", e);
                                }
                            } else {
                                if let Err(e) = context.handle_unauthenticated(pkt, &mut session).await {
                                    error!("Message handler error: {:?}", e);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        break;
                    }
                }
            }
        });
    }

    async fn handle_unauthenticated(&mut self, pkt: CPkt, session: &mut Option<Session>) -> Result<(), FrontendError> {
        debug!("unauthenticated {:?}", pkt);
        Ok(())
    }

    async fn handle(&mut self, pkt: CPkt, session: &Session) -> Result<(), FrontendError> {
        debug!("authenticated {:?}", pkt);
        Ok(())
    }
}