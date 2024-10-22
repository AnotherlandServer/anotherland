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

use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, str::FromStr};

use base64::{engine::general_purpose, prelude::BASE64_STANDARD, Engine};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use log::{debug, error};
use protocol::{oaPktRealmStatusList, CPkt, CPktLoginResult, CpktLoginLoginType, CpktLoginResultUiState, RealmStatus};
use raknet::{RakNetSocket, Reliability};
use steamworks::SteamId;
use tokio::select;
use toolkit::types::Uuid;

use crate::{error::AppError, graphql::auth_service::{AuthInfo, EmailAuthInfo, LoginAccount, LoginAccountVariables, RealmList, SteamAuthInfo}, ARGS};

pub struct AuthSessionContext {
    socket: RakNetSocket,
    authenticated: bool,
}

impl AuthSessionContext {
    pub fn start_auth_session(socket: RakNetSocket) {
        let mut context = Self {
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
                let auth = match pkt.login_type {
                    CpktLoginLoginType::Normal => {
                        AuthInfo {
                            email_auth: Some(EmailAuthInfo {
                                username_or_mail: pkt.username.clone(), 
                                password: pkt.password.clone()
                            }),
                            steam_auth: None,
                        }
                    },
                    CpktLoginLoginType::Steam => {
                        AuthInfo {
                            email_auth: None,
                            steam_auth: Some(SteamAuthInfo {
                                steam_id: SteamId::from_raw(pkt.steam_id).steamid32(),  
                                web_auth_token: general_purpose::STANDARD.encode(pkt.steam_auth_session_ticket),
                            }),
                        }
                    },
                };

                let query = LoginAccount::build(LoginAccountVariables { auth });
                let response = reqwest::Client::new()
                    .post(ARGS.service_auth_url.clone())
                    .run_graphql(query)
                    .await?;

                if 
                    let Some(e) = response.errors &&
                    let Some(e) = e.first()
                {
                    debug!("Login err: {}", e.message);

                    // Send error to client
                    let err = match e.message.as_str() {
                        "WRONG_CREDENTIALS" => "#Login.ERROR_USERNOTFOUND#",
                        "ACCOUNT_BANNED" => "#Login.ERROR_BANNED#",
                        "SERVER_LOCKED" => "#Login.ERROR_SERVERSLOCKED#",
                        _ => "Unknown Error",
                    };

                    self.socket.send(&CPktLoginResult {
                        login_success: false,
                        ui_state: CpktLoginResultUiState::RealmSelection,
                        message_len: Some(err.len() as u8),
                        message: Some(err.as_bytes().to_vec()),
                        ..Default::default()
                    }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;
                } else if let Some(session) = response.data {
                    let session = session.create_session;

                    self.authenticated = true;

                    // We've got a session! 
                    // Now get the realm list.
                    let response = reqwest::Client::new()
                        .post(ARGS.service_auth_url.clone())
                        .run_graphql(RealmList::build(()))
                        .await?;

                    let realm_result = response.data.map(|list| list.realms);
                    
                    // Send login result
                    if 
                        let Some(realms) = &realm_result &&
                        realms.len() == 1 &&
                        let Some(realm) = realms.first() &&
                        let Some(endpoint) = realm.endpoint.as_ref() &&
                        let Some(addr) = SocketAddr::from_str(endpoint).ok() &&
                        let IpAddr::V4(ip) = addr.ip()
                    {
                        // Login result
                        self.socket.send(&CPktLoginResult {
                            login_success: true,
                            ui_state: CpktLoginResultUiState::CharacterSelection,
                            user_id: Some(1),
                            username: Some("".to_string()),
                            magic_bytes: Some(pkt.fingerprint.clone()),
                            session_id: Some(Uuid::parse_str(&session.id.0).unwrap()),
                            realm_id: Some(realm.id),
                            realm_ip: Some(ip.to_bits()),
                            realm_port: Some(addr.port()),
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
                            session_id: Some(Uuid::parse_str(&session.id.0).unwrap()),
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered).await?;

                        // Realm list
                        let realms = realm_result.unwrap_or(vec![]).into_iter()
                            .map(|realm| {
                                let name = if realm.endpoint.is_some() {
                                    realm.name
                                } else {
                                    format!("[OFFLINE] {}", realm.name)
                                };

                                RealmStatus {
                                    id: realm.id,
                                    name,
                                    channel_count: 1,
                                    channel_id: vec![1],
                                    channel_population_count: 1,
                                    channel_population: vec![realm.population as f32],
                                }
                            })
                            .collect::<Vec<_>>();

                        self.socket.send(&oaPktRealmStatusList {
                            realm_count: realms.len() as u32,
                            realms,
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

