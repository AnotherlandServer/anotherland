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

use log::{debug, error};
use protocol::CPkt;
use raknet::RakNetSocket;

use crate::error::AuthError;

pub struct AuthSessionContext {
    socket: RakNetSocket,
}

impl AuthSessionContext {
    pub fn start_auth_session(socket: RakNetSocket) {
        let mut context = Self {
            socket
        };

        tokio::spawn(async move {
            loop {
                if 
                    let Ok(buf) = context.socket.recv().await &&
                    let Ok((_, pkt)) = CPkt::from_bytes(&buf)
                {
                    debug!("{:?}", pkt);
                    if let Err(e) = context.handle(pkt) {
                        error!("Message handler error: {}", e);
                    }
                }
            }
        });
    }

    fn handle(&mut self, pkt: CPkt) -> Result<(), AuthError> {
        match pkt {
            CPkt::CPktLogin(pkt) => {
                Ok(())
            },
            _ => {
                debug!("Unhandled message: {:?}", pkt);
                Ok(())
            }
        }
    }
}

