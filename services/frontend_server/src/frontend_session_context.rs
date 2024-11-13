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

use bitstream_io::{ByteWriter, LittleEndian};
use core_api::{CoreApi, Session};
use log::{debug, error, warn};
use obj_params::{ParamWriter, Player};
use protocol::{oaCharacter, oaPktCharacterFailure, CPkt, CPktStream_126_1, CPktStream_126_5};
use raknet::{RakNetSocket, Reliability};
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
                    },
                    Err(e) => {
                        break;
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

                let mut characters: Vec<oaCharacter> = realm_characters
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
                    Err(e) => {
                        self.socket.send(
                            &oaPktCharacterFailure {
                                error_code: protocol::OaPktCharacterFailureErrorCode::NameExists,
                                ..Default::default()
                            }.into_pkt().to_bytes(), Reliability::ReliableOrdered
                        ).await?;
                    }
                }
            },
            CPkt::oaPktCharacterDelete(pkt) => {
                todo!()
            },
            CPkt::oaPktCharacterSelect(pkt) => {
                todo!()
            },
            _ => {
                warn!("Unhandled pkt: {:?}", pkt);
            }
        }

        Ok(())
    }
}