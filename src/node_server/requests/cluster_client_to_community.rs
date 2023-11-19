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

use atlas::{oaPktServerAction, oaPktClusterClientToCommunity};
use log::{debug, warn};

use crate::{node_server::{NodeServer, ClientState}, util::AnotherlandResult, cluster::{CommunityMessage, connect_queue, RealmChannel, ClusterMessage, MessageChannel}, db::{ZoneDef, realm_database}};
use crate::cluster::TravelType::DirectTravel;

impl NodeServer {
    pub(in crate::node_server) async fn request_cluster_client_to_community(&self, state: &mut ClientState, pkt: oaPktClusterClientToCommunity) -> AnotherlandResult<()> {
        debug!("{:#?}", pkt);

        match CommunityMessage::from_native(pkt.field_3.clone())? {
            CommunityMessage::SocialTravel { avatar, map, travel } => {
                if avatar != state.avatar_id {
                    // what are you doing??
                    warn!("Client tried to 'send' an avatar it doesn't has onership of: {:#?}", avatar);
                } else {
                    if travel {
                        if let Some(target_zone) = ZoneDef::get_by_name(realm_database().await, &map).await? {
                            // we initiate the travel handshake. First, we inform the target zone server, that
                            // we'd like to initiate travel. After it has confirmed, we notify the frontend server 
                            // about the change and remove the client from this zone server.
                            let (dest_server, _) = connect_queue(MessageChannel::RealmChannel { 
                                realm_id: self.read().await.realm_id, 
                                channel: RealmChannel::NodeChannel { zone_guid: target_zone.guid.clone() } 
                            }).await?;

                            dest_server.send(ClusterMessage::ZoneTravelRequest { 
                                session_id: state.session.id.clone(), 
                                peer_id: state.peer_id.clone(),
                                avatar_id: state.avatar_id.clone(),
                                current_zone: self.read().await.zone.read().await.zonedef().guid.clone(),
                                destination_zone: target_zone.guid, 
                                travel_type: DirectTravel
                            }).await?;
                        } else {
                            // todo: inform the player, that the travel destination was invalid
                        }

                      

                        /*match self.world
                        .get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap()
                        .get_avatar_mut(&avatar).unwrap() {

                            Avatar::Player(player) => {
                                // get world
                                let world = WorldDef::get_by_name(self.realm_db.clone(), &map).await?.unwrap();

                                // Send resource notification 
                                /*let mut worlddef = CPktResourceNotify::default();
                                worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                                worlddef.field_2 = world.unwrap().guid.clone();
                                worlddef.field_3 = "".to_owned();
                
                                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, worlddef.as_message()).await?;*/

                                let params = player.player_param_mut();
                                params.set_world_map_guid(world.guid.clone());
                                params.set_zone_guid(Uuid::from_str("b1bbd5c5-0990-454b-bcfa-5dfe176c6756").unwrap());

                                // Update avatar
                                let mut data = Vec::new();
                                let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                                player.params().write(&mut writer)?;
    
                                let mut avatar_update = CPktAvatarUpdate::default();
                                avatar_update.full_update = false;
                                avatar_update.avatar_id = Some(state.avatar_id.as_ref().unwrap().as_u64());
                                avatar_update.update_source = 0;
                                avatar_update.param_bytes = data.len() as u32;
                                avatar_update.params = data;
                                
                                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;
                                /*let mut action = oaPktServerAction::default();
                                action.action = format!{"TELEPORT:TeleportTravel:TeleportTravelDefault",};
                                action.version = 4;
                                action.override_teleport = true;
                                action.pos = NetworkVec3 { x: 0.0, y: 0.0, z: 0.0};
                                action.rot = NetworkVec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0};
                                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await;*/
                            },
                            _ => {}
                        }*/
                    }
                }

                Ok(())
            },
            CommunityMessage::Unknown_A1 { avatar, boolean } => {
                warn!("Unabled community message: 0xa1: {}", boolean);

                Ok(())
            },
        }
    }
}