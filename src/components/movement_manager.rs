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

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::{AvatarId, ParamClassContainer};
use glam::{Vec4, Quat, Vec3};
use tokio::select;
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{cluster::actor::{Actor, ActorRef}, util::AnotherlandResult, NODE, components::ZoneEvent};

use super::{SessionManager, Zone};

pub struct MovementManager {
    name: String,
    zone: ActorRef<Zone>,
    token: CancellationToken,
    tasks: TaskTracker,
}

impl MovementManager {
    pub async fn initialize(name: &str, zone: ActorRef<Zone>) -> AnotherlandResult<Self> {
        Ok(Self {
            name: name.to_owned(),
            zone,
            token: CancellationToken::new(),
            tasks: TaskTracker::new(),
        })
    }
}

#[async_trait]
impl Actor for MovementManager {
    fn name(&self) -> &str { &self.name }

    async fn starting(&mut self) -> AnotherlandResult<()> {
        self.tasks.spawn({
            let mut local_actor = NODE.get_actor::<MovementManager>(&self.name).unwrap();
            let mut zone_events = self.zone.subscribe().await;
            let token = self.token.clone();

            async move {
                'event_loop: loop {
                    select! {
                        Ok(event) = zone_events.recv() => {
                            match event.as_ref() {
                                ZoneEvent::AvatarSpawned { avatar_id, params } => {
                                    match params {
                                        ParamClassContainer::Player(params) => todo!(),
                                        
                                    }
                                },
                                ZoneEvent::AvatarDespawned { avatar_id } => {
                                    local_actor.despawn_avatar(avatar_id.clone()).await;
                                },
                                _ => (),
                            }
                        },
                        _ = token.cancelled() => {
                            break 'event_loop;
                        }
                    }
                }

            }
        });

        Ok(()) 
    }

    async fn started(&mut self) -> AnotherlandResult<()> {
        Ok(())
    }

    async fn stopping(&mut self) -> AnotherlandResult<()> {
        self.token.cancel();
        self.tasks.close();
        self.tasks.wait().await;

        Ok(())
    }
}

#[actor_actions]
impl MovementManager {
    pub async fn spawn_avatar(&mut self, avatar_id: AvatarId, pos: Vec3, rot: Vec3) {
        todo!()
    }

    pub async fn despawn_avatar(&mut self, avatar_id: AvatarId) {
        todo!()
    }

    pub async fn update_position(&mut self, avatar_id: AvatarId, pos: Vec3, rot: Vec3, vel: Vec3) -> AnotherlandResult<()> {
        todo!()
    }
}