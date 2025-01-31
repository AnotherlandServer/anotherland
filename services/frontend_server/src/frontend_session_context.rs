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

use std::net::{SocketAddr, SocketAddrV4};

use bitstream_io::{ByteWriter, LittleEndian};
use core_api::{CoreApi, Session};
use log::{error, info, warn};
use obj_params::{ParamWriter, Player};
use protocol::{oaCharacter, oaPktCharacterDeleteSuccess, oaPktCharacterFailure, oaPktCharacterSelectSuccess, oaPktResponseSelectWorld, CPkt, CPktStream_126_1, CPktStream_126_5, OaPktCharacterFailureErrorCode, OaPktResponseSelectWorldErrorCode, OtherlandPacket};
use raknet::{RakNetSocket, Reliability};
use realm_api::{ClusterAddress, NodeType, RealmApi};
use futures_util::TryStreamExt;
use toolkit::{anyhow, types::Uuid};

use crate::error::FrontendError;

pub struct FrontendSessionContext {
    core_api: CoreApi,
    realm_api: RealmApi,
    socket: RakNetSocket,
    cluster_addr: Option<SocketAddrV4>,
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
            cluster_addr: None,
        };

        let mut session = None;

        tokio::spawn(async move {
            while let Ok(buf) = context.socket.recv().await {
                if let Ok((_, pkt)) = CPkt::from_bytes(&buf) {
                    if session.is_none() {
                        if let Err(e) = context.handle_unauthenticated(&pkt, &mut session).await {
                            error!("Message handler error: {:?}", e);
                        }
                    }

                    if let Some(session) = session.as_ref() {
                        if let Err(e) = context.handle(pkt, session).await {
                            error!("Message handler error: {:?}", e);
                        }
                    }
                }
            }
        });
    }

    async fn handle_unauthenticated(&mut self, pkt: &CPkt, session: &mut Option<Session>) -> Result<(), FrontendError> {
        if let CPkt::oaPktRequestCharacterList(pkt) = pkt {
            *session = self.core_api.get_session(&pkt.session_id).await?;
        }

        Ok(())
    }

    async fn handle(&mut self, pkt: CPkt, session: &Session) -> Result<(), FrontendError> {
        match pkt {
            CPkt::oaPktRequestCharacterList(pkt) => {
                let realm_characters = self.realm_api.get_characters_for_account(
                    session.account().id()
                ).await?;

                let characters: Vec<oaCharacter> = realm_characters
                    .into_iter()
                    .map(|character| {
                        let mut serialized = Vec::new();
                        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

                        character.data().write_to_client(&mut writer).unwrap();

                        oaCharacter {
                            id: character.index(),
                            name: character.name().to_string(),
                            world_id: 0,
                            params: serialized.into(),
                            ..Default::default()
                        }
                    })
                    .collect();

                self.socket.send(
                    &CPktStream_126_1 {
                        count: characters.len() as u32,
                        characters,
                        ..Default::default()
                    }.into_pkt().to_bytes(),
                    Reliability::ReliableOrdered
                ).await?;
            },
            CPkt::oaPktCharacterCreate(pkt) => {
                match self.realm_api.create_character(session.account().id(), pkt.character_name).await {
                    Ok(character) => {
                        let mut serialized = Vec::new();
                        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

                        character.data().write_to_client(&mut writer).unwrap();

                        self.socket.send(
                            &CPktStream_126_5 {
                                character: oaCharacter {
                                    id: character.index(),
                                    name: character.name().to_string(),
                                    world_id: 0,
                                    params: serialized.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }.into_pkt().to_bytes(), 
                            Reliability::ReliableOrdered
                        ).await?;
                    },
                    Err(_) => {
                        self.socket.send(
                            &oaPktCharacterFailure {
                                error_code: OaPktCharacterFailureErrorCode::NameExists,
                                ..Default::default()
                            }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                        ).await?;
                    }
                }
            },
            CPkt::oaPktCharacterDelete(pkt) => {
                if let Some(character) = self.realm_api.get_character_for_account(session.account().id(), pkt.character_id).await? {
                    character.delete().await?;

                    self.socket.send(
                        &oaPktCharacterDeleteSuccess {
                            character_id: pkt.character_id,
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                    ).await?;
                } else {
                    self.socket.send(
                        &oaPktCharacterFailure {
                            error_code: OaPktCharacterFailureErrorCode::DatabaseError,
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                    ).await?;
                }
            },
            CPkt::oaPktRequestSelectWorld(_) => {
                // Check if we can get a valid cluster node
                let cluster_node = 
                if let Some(node) = self.realm_api.get_cluster_nodes().await?
                    .into_iter()
                    .find(|node| matches!(node.ty, NodeType::Cluster))
                {
                    node
                } else {
                    self.socket.send(
                        &oaPktResponseSelectWorld {
                            error_code: OaPktResponseSelectWorldErrorCode::ServerOffline,
                            success: false,
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                    ).await?;

                    return Ok(());
                };

                let cluster_address = match cluster_node.addr {
                    ClusterAddress::Public(SocketAddr::V4(addr)) => addr,
                    ClusterAddress::Public(SocketAddr::V6(_)) => unimplemented!(),
                    ClusterAddress::Internal(_) => unreachable!(),
                };

                self.cluster_addr = Some(cluster_address);

                self.socket.send(
                    &oaPktResponseSelectWorld {
                        success: true,
                        ..Default::default()
                    }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                ).await?;
            },
            CPkt::oaPktCharacterSelect(pkt) => {
                let cluster_address = self.cluster_addr.as_ref()
                    .ok_or(anyhow::Error::msg("no cluster selected"))?;

                if let Some(character) = self.realm_api.get_character_for_account(session.account().id(), pkt.character_id).await? {
                    let _ = self.realm_api.join_game(*session.id(), *character.id()).await?;

                    // Send client to the cluster server determined earlier
                    self.socket.send(
                        &oaPktCharacterSelectSuccess {
                            cluster_ip: u32::from_le_bytes(cluster_address.ip().octets()),
                            cluster_port: cluster_address.port(),
                            ..Default::default()
                        }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                    ).await?;
                } else {
                    warn!("Account '{}' tried to select non-existing character {}", session.account().id(), pkt.character_id);

                    // We can't send any error messages back to the client
                    // once oaPktResponseSelectWorld was received by them.
                    self.socket.close().await;
                }
            },
            _ => {
                warn!("Unhandled pkt: {:?}", pkt);
            }
        }

        Ok(())
    }
}