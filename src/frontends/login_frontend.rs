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

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, Message, RakNetRequest, RakNetPeerHandle, Priority, Reliability}, CPkt, CPktLoginResult, CpktLoginResultUiState};
use futures::future::Remote;
use log::error;

use crate::{cluster::{frontend::Frontend, actor::{ActorRef, RemoteActorRef}}, util::{AnotherlandResult, AnotherlandError}, components::{Authenticator, LoginResult, SessionHandler}, CONF, NODE, db::Session};

pub struct LoginFrontend {
    listener: RakNetListener,
}

impl LoginFrontend {
    pub fn new() -> Self {
        LoginFrontend { 
            listener: RakNetListener::new(),
        }
    }

    async fn handle_request(authenticator: &RemoteActorRef<Authenticator>, session_handler: &mut ActorRef<SessionHandler<()>>, request: &RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;
        
        match request.message() {
            AtlasPkt(CPkt::CPktLogin(pkt)) => {
                match authenticator.login(pkt.username.to_owned(), pkt.password.to_owned()).await? {
                    LoginResult::Session(session) => {
                        Ok(())
                    },
                    LoginResult::InvalidCredentials => {
                        Self::report_login_error(request.peer(), "#UI.ERROR_PASSWORDMISMATCH").await
                    },
                    LoginResult::Banned => {
                        Self::report_login_error(request.peer(), "#UI.ERROR_BANNED").await
                    }
                }
            },
            _ => Err(AnotherlandError::app_err(format!("Unknown message: {:?}", request.message()).as_str()))
        }
    }

    async fn report_login_error(peer: RakNetPeerHandle, message: &str) -> AnotherlandResult<()> {
        let mut result = CPktLoginResult::default();
        result.login_success = false;
        result.ui_state = CpktLoginResultUiState::RealmSelection;
        
        let message = message.as_bytes();

        result.message_len = Some(message.len() as u8);
        result.message = Some(message.to_vec());

        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, result.as_message()).await?;
        Ok(())
    }
}

#[async_trait]
impl Frontend for LoginFrontend {
    async fn pre_start(&mut self) -> AnotherlandResult<()> { 
        self.listener.listen(CONF.login_server.listen_address).await?;

        Ok(())
    }

    async fn run(&mut self) -> AnotherlandResult<()> {
        let authenticator = NODE.get_remote_actor("authenticator").unwrap();
        let mut session_handler = NODE.add_actor(SessionHandler::<()>::new("login_session_handler").await);

        while let Some(request) = self.listener.next_request().await {

            if let Err(e) = Self::handle_request(&authenticator, &mut session_handler, &request).await {
                error!("Error during request handling: {:?}", e);
                request.peer().write().await.disconnect().await;
            }
        }

        Ok(()) 
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }
}