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

use core_api::{CoreApi, LoginError, Realm};
use log::{debug, error};
use protocol::{oaPktRealmStatusList, CPkt, CPktLoginResult, CpktLoginLoginType, CpktLoginResultUiState, OtherlandPacket, RealmStatus};
use raknet::{RakNetSocket, Reliability};
use steamworks::SteamId;
use tokio::{select, sync::broadcast};
use toolkit::types::Uuid;

use crate::error::AppError;

pub struct AuthSessionContext {
    auth_api: CoreApi,
    socket: RakNetSocket,
    session_id: Option<Uuid>,
}

fn serialize_realm(realm: &Realm) -> RealmStatus {
    if let Some(population) = realm.population() {
        RealmStatus {
            id: realm.id(),
            name: realm.name().to_string(),
            channel_count: 1,
            channel_id: vec![realm.id() as u32], // Due to a client bug, we have to identify realms based on channel ids. 
            channel_population_count: 1,
            channel_population: vec![population as f32],
        }
    } else {
        RealmStatus {
            id: realm.id(),
            name: format!("[OFFLINE] {}", realm.name()),
            channel_count: 1,
            channel_id: vec![realm.id() as u32], // Due to a client bug, we have to identify realms based on channel ids. 
            channel_population_count: 1,
            channel_population: vec![0.0],
        }
    }
}

impl AuthSessionContext {
    pub fn start_auth_session(auth_api: CoreApi, socket: RakNetSocket, mut realm_update: broadcast::Receiver<()>) {
        let mut context = Self {
            auth_api,
            socket,
            session_id: None,
        };

        tokio::spawn(async move {
            loop {
                select! {
                    res = context.socket.recv() => {
                        match res {
                            Ok(buf) => {
                                if 
                                    let Ok((_, pkt)) = CPkt::from_bytes(&buf) &&
                                    let Err(e) = context.handle(pkt).await
                                {
                                    error!("Message handler error: {e:?}");
                                }
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    },
                    _ = realm_update.recv() => {
                        let _ = context.update_realm_list().await;
                    }
                }
            }
        });
    }

    async fn update_realm_list(&self) -> Result<(), AppError> {
        if self.session_id.is_none() { return Ok(()); }
        
        // Get realm list
        let realms = self.auth_api.get_realms().await?;

        // Realm list
        let realms = realms.iter()
            .map(serialize_realm)
            .collect::<Vec<_>>();

        self.socket.send(&oaPktRealmStatusList {
            realm_count: realms.len() as u32,
            realms,
            ..Default::default()
        }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;

        Ok(())
    }

    async fn handle(&mut self, pkt: CPkt) -> Result<(), AppError> {
        debug!("Pkt: {pkt:?}");

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
                        self.session_id = Some(*session.id());

                        // We've got a session! 
                        // Now get the realm list.
                        let realms = self.auth_api.get_realms().await?;
                        
                        // Send login result
                        if 
                            let Some(realm) = realms.first() && realms.len() == 1 &&
                            let Some(endpoint) = realm.endpoint()
                        {
                            // Login result
                            self.socket.send(&CPktLoginResult {
                                login_success: true,
                                ui_state: CpktLoginResultUiState::CharacterSelection,
                                user_id: Some(1),
                                username: Some("".to_string()),
                                magic_bytes: Some(pkt.fingerprint.clone()),
                                session_id: Some(*session.id()),
                                realm_id: Some(realm.id()),
                                realm_ip: Some(u32::from_le_bytes(endpoint.ip().octets())),
                                realm_port: Some(endpoint.port()),
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
                            let realms = realms.iter()
                                .map(serialize_realm)
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
            CPkt::oaPktRealmSelect(pkt) => {
                /*
                    The client returns an index of the client-internal realm array as realm id,
                    which does not necessarily matches with the real realm id.
                    Therefore we use the channel_id to identify the realm.
                    We can do this, because we currently always assume only a single channel per realm.
                */
                if 
                    let Some(realm) = self.auth_api.get_realm(pkt.channel_id).await? &&
                    let Some(endpoint) = realm.endpoint()
                {
                    // Login result
                    self.socket.send(&CPktLoginResult {
                        login_success: true,
                        ui_state: CpktLoginResultUiState::CharacterSelection,
                        user_id: Some(1),
                        username: Some("".to_string()),
                        magic_bytes: Some(vec![0; 16]),
                        session_id: self.session_id,
                        realm_id: Some(realm.id()),
                        realm_ip: Some(u32::from_le_bytes(endpoint.ip().octets())),
                        realm_port: Some(endpoint.port()),
                        ..Default::default()
                    }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                } else {
                    let err = "#Login.ERROR_NOFRONTENDSERVER#";

                    self.socket.send(&CPktLoginResult {
                        login_success: false,
                        ui_state: CpktLoginResultUiState::RealmSelection,
                        message_len: Some(err.len() as u8),
                        message: Some(err.as_bytes().to_vec()),
                        ..Default::default()
                    }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                }

                Ok(())
            },
            _ => {
                debug!("Unhandled message: {pkt:?}");
                Ok(())
            }
        }
    }
}

