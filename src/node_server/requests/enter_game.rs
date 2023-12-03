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

use atlas::{oaPktRequestEnterGame, CPktResourceNotify, CpktResourceNotifyResourceType, PlayerComponent, PlayerParam, PositionUpdate, CPktBlob, BoundParamClass, ParamEntity};
use bitstream_io::{ByteWriter, LittleEndian};
use log::{info, debug};
use atlas::Player;
use std::ops::DerefMut;

use crate::{util::{AnotherlandResult, AnotherlandError}, node_server::{NodeServer, ClientState, world::{AvatarComponent, PlayerSpawnMode}}, cluster::{ClusterMessage, SocialEvent}};

impl NodeServer {
    pub(in crate::node_server) async fn request_enter_game(&self, state: &mut ClientState, _pkt: oaPktRequestEnterGame) -> AnotherlandResult<()> {
        info!(client = state; "Player joining world!");

        // Send resource notification 
        let mut worlddef = CPktResourceNotify::default();
        worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
        worlddef.field_2 = self.read().await.worlddef.guid.clone();
        worlddef.field_3 = "".to_owned();

        self.send(&state.peer_id, worlddef.as_message()).await?;

        // Update and get avatar data
        let (name, params, pos) = {
            let mut param_buffer = Vec::new();
            let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

            let world_state = self.read().await;
            let zone = world_state.zone.read().await;
            //let zones = world_state.zones.read().await;
            //let zone = zones.get(&state.zone)
            //.ok_or(AnotherlandError::app_err("zone not found"))?;

            // update player state
            let character_component = {
                let mut instance = zone.instance().write().await;
                let mut entry = instance
                    .entry(state.entity).ok_or(AnotherlandError::app_err("entity not found"))?;

                let character_component = entry.get_component::<AvatarComponent>().unwrap().to_owned();

                let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();
                player_component.set_spawn_mode(PlayerSpawnMode::LoginNormal.into());
                player_component.set_client_ready(false);
                player_component.set_player_loading(true);
                player_component.set_player_node_state(2);
                /*player_component.set_world_map_guid(&world_state.worlddef.guid.to_string());
                player_component.set_zone(&zone.zonedef().zone.to_string());
                player_component.set_zone_guid(state.zone.clone());*/

                character_component
            };

            let params = PlayerParam::from_component(zone.instance().write().await.deref_mut(), state.entity)?;
            params.write(&mut writer)?;

            (
                character_component.name,
                param_buffer,
                PositionUpdate {
                    pos: params.pos().unwrap().to_owned().into(),
                    rot: params.rot().unwrap().to_owned().into(),
                    vel: character_component.vel.into(),
                }.to_bytes()
            )
        };

        self.read().await.social.send(ClusterMessage::SocialEvent { 
            event: SocialEvent::PeerEnter { peer_id: state.peer_id.clone(), zone: state.zone.clone() }
        }).await?;

        debug!("Player avatar id: {:#?}", state.avatar_id);

        // Transfer character to client
        let mut avatar_blob = CPktBlob::default();
        avatar_blob.avatar_id = state.avatar_id.as_u64();
        avatar_blob.avatar_name = name;
        avatar_blob.class_id = PlayerParam::CLASS_ID.as_u32();
        avatar_blob.param_bytes = params.len() as u32;
        avatar_blob.params = params;
        avatar_blob.movement_bytes = pos.len() as u32;
        avatar_blob.movement = pos;
        avatar_blob.has_guid = true;
        avatar_blob.field_9 = Some(state.session.id.clone());

        self.send(&state.peer_id, avatar_blob.as_message()).await?;

        Ok(())
    }
}