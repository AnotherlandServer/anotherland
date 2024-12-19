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

use std::{collections::HashMap, sync::Arc};

use cluster::ClusterResult;
use core_api::{CoreApi, Session};
use log::{debug, error};
use obj_params::Player;
use once_cell::sync::Lazy;
use protocol::CPkt;
use raknet::RakNetSocket;
use realm_api::{ClusterAddress, RealmApi, SessionState};
use tokio::{select, sync::{mpsc::Sender, Mutex, Notify}};
use toolkit::types::Uuid;
use world_service::{WorldClient, WorldRequest};

use crate::{error::ClusterFrontendResult, router::Router};

pub struct ClusterContext {
    core_api: CoreApi,
    realm_api: RealmApi,
    router: Router,
    socket: RakNetSocket,
    session: Session,
    session_state: SessionState,
    world_sender: Sender<WorldRequest>,
}

impl ClusterContext {
    pub async fn create_and_start(
        core_api: CoreApi, 
        realm_api: RealmApi, 
        router: Router,
        socket: RakNetSocket,
        session: Session,
    ) -> ClusterFrontendResult<Arc<Notify>> {
        // Get extended session from realm
        let session_state = realm_api.get_session_state(*session.id()).await?
            .ok_or(anyhow::Error::msg("session ext not found"))?;

        // Get selected character
        let mut character = realm_api.get_character(
                session_state.character()
            ).await?
            .ok_or(anyhow::Error::msg("character not found"))?;

        let (instance, mut receiver, sender) = router.join_instance(
            socket.id().into(), 
            *session.id(), 
            *character.data().get(Player::ZoneGuid)?, 
            character.data().get::<_, String>(Player::InstanceZoneKey)?
                .parse::<Uuid>()
                .ok()
            ).await?;

        // Notify world
        let _ = sender.send(WorldRequest::ClientConnected { 
            peer: socket.id().into(), 
            session: *session.id() 
        }).await;

        character.data_mut()
            .set(Player::InstanceZoneKey, instance.key.as_ref().map(Uuid::to_string).unwrap_or_default());
        

        let context = Self {
            core_api,
            realm_api,
            router,
            socket,
            session,
            session_state,
            world_sender: sender,
        };

        let semaphore = Arc::new(Notify::new());
        let ret_semaphore = semaphore.clone();

        tokio::spawn(async move {
            loop {
                select! {
                    _ = semaphore.notified() => break,
                    res = receiver.recv() => {
                        match res {
                            Some(res) => {
                                match res {
                                    world_service::WorldResponse::ServerMessage { data, .. } => {
                                        let _ = context.socket.send(&data, raknet::Reliability::ReliableOrdered).await;
                                    },
                                    world_service::WorldResponse::Travel { data, .. } => todo!(),
                                }
                            }
                            None => {
                                context.socket.close().await;
                                break;
                            }
                        }
                    }
                    req = context.socket.recv() => {
                        match req {
                            Ok(pkt) => {
                                if let Err(e) = context.world_sender.send(WorldRequest::ClientMessage { 
                                        peer: context.socket.id().into(), 
                                        data: pkt
                                    }).await 
                                {
                                    error!("Failed to handle client request: {:#?}", e);
                                    context.socket.close().await;
                                    break;
                                }
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            } 
        });

        Ok(ret_semaphore)
    }
}