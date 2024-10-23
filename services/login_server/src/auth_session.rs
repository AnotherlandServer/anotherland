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

use core_api::{CoreApi, LoginError};
use log::{debug, error};
use protocol::{oaPktRealmStatusList, CPkt, CPktLoginResult, CpktLoginLoginType, CpktLoginResultUiState, RealmStatus};
use raknet::{RakNetSocket, Reliability};
use steamworks::SteamId;
use tokio::select;

use crate::error::AppError;

pub struct AuthSessionContext {
    auth_api: CoreApi,
    socket: RakNetSocket,
    authenticated: bool,
}

impl AuthSessionContext {
    pub fn start_auth_session(auth_api: CoreApi, socket: RakNetSocket) {
        let mut context = Self {
            auth_api,
            socket,
            authenticated: false,
        };

        tokio::spawn(async move {
            loop {
                select! {
                    Ok(buf) = context.socket.recv() => {
                        if let Ok((_, pkt)) = CPkt::from_bytes(&buf) {
                            if let Err(e) = context.handle(pkt).await {
                                error!("Message handler error: {:?}", e);
                            }
                        }
                    }
                }
            }
        });
    }

    async fn handle(&mut self, pkt: CPkt) -> Result<(), AppError> {
        debug!("Pkt: {:?}", pkt);

        match pkt {
            CPkt::CPktLogin(pkt) => {
                let result = match pkt.login_type {
                    CpktLoginLoginType::Normal => {
                        self.auth_api.login_username(&pkt.username, &pkt.password).await?
                    },
                    CpktLoginLoginType::Steam => {
                        self.auth_api.login_steam(SteamId::from_raw(pkt.steam_id), &pkt.steam_auth_session_ticket).await?
                    }
                };

                match result {
                    Ok(session) => {
                        self.authenticated = true;

                        // We've got a session! 
                        // Now get the realm list.
                        let realms = self.auth_api.get_realms().await?;
                        
                        // Send login result
                        if let Some(realm) = realms.first() && realms.len() == 1 {
                            // Login result
                            self.socket.send(&CPktLoginResult {
                                login_success: true,
                                ui_state: CpktLoginResultUiState::CharacterSelection,
                                user_id: Some(1),
                                username: Some("".to_string()),
                                magic_bytes: Some(pkt.fingerprint.clone()),
                                session_id: Some(*session.id()),
                                realm_id: Some(realm.id()),
                                realm_ip: Some(realm.endpoint().ip().to_bits()),
                                realm_port: Some(realm.endpoint().port()),
                                ..Default::default()
                            }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                        } else {
                            // Send login result
                            self.socket.send(&CPktLoginResult {
                                login_success: true,
                                ui_state: CpktLoginResultUiState::RealmSelection,
                                user_id: Some(1),
                                username: Some("".to_string()),
                                magic_bytes: Some(pkt.fingerprint.clone()),
                                session_id: Some(*session.id()),
                                ..Default::default()
                            }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
    
                            // Realm list
                            let realms = realms.into_iter()
                                .map(|realm| {   
                                    RealmStatus {
                                        id: realm.id(),
                                        name: realm.name().to_string(),
                                        channel_count: 1,
                                        channel_id: vec![1],
                                        channel_population_count: 1,
                                        channel_population: vec![realm.population() as f32],
                                    }
                                })
                                .collect::<Vec<_>>();
    
                            self.socket.send(&oaPktRealmStatusList {
                                realm_count: realms.len() as u32,
                                realms,
                                ..Default::default()
                            }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                        }
                    },
                    Err(e) => {
                        let err = match e {
                            LoginError::WrongCredentials => "#Login.ERROR_USERNOTFOUND#",
                            LoginError::Banned => "#Login.ERROR_BANNED#",
                            LoginError::ServerLocked => "#Login.ERROR_SERVERSLOCKED#",
                        };

                        self.socket.send(&CPktLoginResult {
                            login_success: false,
                            ui_state: CpktLoginResultUiState::RealmSelection,
                            message_len: Some(err.len() as u8),
                            message: Some(err.as_bytes().to_vec()),
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                    }
                }

                Ok(())
            },
            _ => {
                debug!("Unhandled message: {:?}", pkt);
                Ok(())
            }
        }
    }
}

