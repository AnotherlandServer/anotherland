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

use std::{collections::HashMap, net::SocketAddrV4};

use actor_macros::actor_actions;
use async_trait::async_trait;

use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{cluster::{actor::Actor, connect_queue, MessageChannel, ClusterMessage}, util::AnotherlandResult};

#[derive(Clone)]
pub struct RealmEntry {
    pub id: u32,
    pub name: String,
    pub channels: Vec<RealmChannel>,
    pub address: SocketAddrV4,
}

#[derive(Clone)]
pub struct RealmChannel {
    pub id: u32,
    pub population: f32,
}

pub struct RealmList {
    realms: HashMap<u32, RealmEntry>,
    cancellation_token: CancellationToken,
    subtasks: TaskTracker,
}

impl RealmList {
    pub async fn initialize() -> Self {
        Self {
            realms: HashMap::new(),
            cancellation_token: CancellationToken::new(),
            subtasks: TaskTracker::new(),
        }
    }
}

#[async_trait]
impl Actor for RealmList {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { Some("realm_list") }

    async fn started(&mut self, mut handle: ActorRef<Self>) -> AnotherlandResult<()> {
        let cancellation_token = self.cancellation_token.clone();
        self.subtasks.spawn(async move {
            let (_, mut cluster_channel) = connect_queue(MessageChannel::ClusterChannel).await.unwrap();

            'message_loop: loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => { break 'message_loop; },
                    Ok(msg) = cluster_channel.recv() => {
                        if let ClusterMessage::RealmServerHearthbeat{realm_id, name, channels, address} = msg {
                            handle.update_realm(realm_id, name, channels, address).await
                        }
                    },
                }
            }
        });

        Ok(()) 
    }

    async fn stopping(&mut self) -> AnotherlandResult<()> { 
        self.cancellation_token.cancel();

        self.subtasks.close();
        self.subtasks.wait().await;

        Ok(()) 
    }
}

#[actor_actions]
impl RealmList {
    #[rpc]
    pub fn get_realms(&self) -> Vec<RealmEntry> {
        self.realms.values().cloned().collect()
    }

    #[rpc]
    pub fn get_realm(&self, id: u32) -> Option<RealmEntry> {
        self.realms.get(&id).cloned()
    }

    pub fn update_realm(&mut self, realm_id: u32, name: String, channels: Vec<(u32, f32)>, address: SocketAddrV4) {
        self.realms.insert(realm_id, RealmEntry {
            id: realm_id,
            name,
            channels: channels.into_iter().map(|(id, population)| {
                RealmChannel {
                    id,
                    population,
                }
            }).collect(),
            address,
        });
    }
}