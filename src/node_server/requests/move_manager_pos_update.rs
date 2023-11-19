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

use std::ops::Deref;

use atlas::{PlayerComponent, Player, oaPktMoveManagerPosUpdate, NetworkVec4};
use glam::Vec3;
use legion::EntityStore;
use log::debug;

use crate::{node_server::{ClientState, NodeServer, world::AvatarComponent}, util::AnotherlandResult};

impl NodeServer {
    pub(in crate::node_server) async fn request_move_manager_pos_update(&self, state: &mut ClientState, pkt: oaPktMoveManagerPosUpdate) -> AnotherlandResult<()> {
        // Update world state
        let instance =  self.read().await.zone.read().await.instance().clone();

        // update world
        {
            let mut instance_s = instance.write().await;

            if let Ok(mut entry) = instance_s.entry_mut(state.entity) {
                let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();

                player_component.set_pos(pkt.pos.pos.into());
                player_component.set_rot(pkt.pos.rot.clone().into());
            }
        }

        // check interests
        let instance_s = instance.read().await;

        if let Ok(entry) = instance_s.entry_ref(state.entity) {
            let player_component = entry.get_component::<PlayerComponent>().unwrap();

            let interests = Self::query_interests(
                player_component.pos().unwrap().clone(), 
                *player_component.aware_range().unwrap(), 
                instance_s.deref());

            // stream avatars which newly came into range
            for interest in interests.iter() {
                if !state.interest_list.contains(&interest) {
                    state.avatar_upload_queue.push_back(interest.clone());
                    state.interest_list.insert(interest.clone());
                }
            }

            // remove the ones out of our awarenes range
            for interest in state.interest_list.clone() {
                if !interests.contains(&interest) {
                    if let Ok(entry) = instance_s.entry_ref(interest) {
                        let avatar_component = entry.get_component::<AvatarComponent>().unwrap();
                        state.avatar_despawn_queue.push_back(avatar_component.id.clone());
                        state.interest_list.remove(&interest);
                    }
                }
            }
        }

        let vec_rot: Vec3 = pkt.pos.rot.clone().into();

        debug!("Rot Quat: {:#?}", pkt.pos.rot);
        debug!("Rot: {:#?}", vec_rot);

        let vec4: NetworkVec4 = vec_rot.into();
        debug!("Rot Quat2: {:#?}", vec4);
                    
        /*let avatar = self.world
            .get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap()
            .get_avatar_mut(state.avatar_id.as_ref().unwrap()).unwrap();
        avatar.set_position(pkt.pos.pos.into());
        avatar.set_rotation(pkt.pos.rot.into());
        avatar.set_velocity(pkt.pos.vel.into());*/

        Ok(())
    }
}