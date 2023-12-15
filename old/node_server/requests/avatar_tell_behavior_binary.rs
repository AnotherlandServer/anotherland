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

use atlas::{oaPktAvatarTellBehaviorBinary, NativeParam, PlayerComponent, Player, PlayerParam, ParamEntity, CPktAvatarUpdate, BoundParamClass};
use bitstream_io::{ByteWriter, LittleEndian};
use legion::EntityStore;
use log::{debug, info};

use crate::{node_server::{NodeServer, ClientState}, util::AnotherlandResult, db::{realm_database, ItemContent, DatabaseRecord, Character}};

impl NodeServer {
    pub(in crate::node_server) async fn request_avatar_tell_behavior_binary(&self, state: &mut ClientState, pkt: oaPktAvatarTellBehaviorBinary) -> AnotherlandResult<()> {
        match pkt.field_3.as_str() {
            "doVendorExecute" => {
                match &pkt.field_4 {
                    NativeParam::Struct(attrib) => {
                        let instance =  self.read().await.zone.read().await.instance().clone();
                        let mut instance_s = instance.write().await;

                        let db: mongodb::Database = realm_database().await;

                        if let Ok(mut entry) = instance_s.entry_mut(state.entity) {
                            let player_component = entry.get_component_mut::<PlayerComponent>().unwrap();

                            player_component.set_customization_gender(attrib[0].to_f32()?);
                            player_component.set_customization_height(attrib[1].to_f32()?);
                            player_component.set_customization_fat(attrib[2].to_f32()?);
                            player_component.set_customization_skinny(attrib[3].to_f32()?);
                            player_component.set_customization_muscular(attrib[4].to_f32()?);
                            player_component.set_customization_bust_size(attrib[5].to_f32()?);
                            player_component.set_race(attrib[6].to_i32()?);
                            player_component.set_customization_brow_angle(attrib[7].to_f32()?);
                            player_component.set_customization_eye_brow_pos(attrib[8].to_f32()?);
                            player_component.set_customization_eye_pos_spacing(attrib[9].to_f32()?);
                            player_component.set_customization_eye_pos(attrib[10].to_f32()?);
                            player_component.set_customization_eye_size_length(attrib[11].to_f32()?);
                            player_component.set_customization_eye_size_width(attrib[12].to_f32()?);
                            player_component.set_customization_eyes_pretty(attrib[13].to_f32()?);
                            player_component.set_customization_mouth_pos(attrib[14].to_f32()?);
                            player_component.set_customization_mouth_width(attrib[15].to_f32()?);
                            player_component.set_customization_mouth_lower_lip_thic(attrib[16].to_f32()?);
                            player_component.set_customization_mouth_upper_lip_thic(attrib[17].to_f32()?);
                            player_component.set_customization_mouth_expression(attrib[18].to_f32()?);
                            player_component.set_customization_nose_pos_length(attrib[19].to_f32()?);
                            player_component.set_customization_nose_pos_width(attrib[20].to_f32()?);
                            player_component.set_customization_nose_portude(attrib[21].to_f32()?);
                            player_component.set_customization_ear_size(attrib[22].to_f32()?);
                            player_component.set_customization_ear_elf(attrib[23].to_f32()?);
                            player_component.set_customization_cheek_bone(attrib[24].to_f32()?);
                            player_component.set_customization_cheek(attrib[25].to_f32()?);
                            player_component.set_customization_chin_portude(attrib[26].to_f32()?);
                            player_component.set_customization_jaw_chubby(attrib[27].to_f32()?);
                            debug!("Attrib 28: {}", attrib[28].to_string()?);
                            debug!("Attrib 29: {:#?}", attrib[29]);
                            // voucher 28
                            // int items 29

                            let mut visible_items = Vec::new();
                            for a in attrib[30..].iter() {
                                let item_uuid = a.to_uuid()?;
                                debug!("Load item {}", item_uuid.to_string());
                            let db: mongodb::Database = realm_database().await;
                            let item = ItemContent::get(db.clone(), &item_uuid).await?;
                                visible_items.push(item.unwrap().id as i32);
                            }

                            if !visible_items.is_empty() {
                                debug!("set visible item info");
                                player_component.set_visible_item_info(visible_items);
                            } else {
                                debug!("received empty visible item info after metamorph");
                            }
                        }

                        // Save changes
                        debug!("Save avatar change");

                        let params = PlayerParam::from_component(&instance_s, state.entity).unwrap();
                        
                        let mut character = Character::get(db.clone(), &state.session.character_id.unwrap()).await.unwrap().unwrap();
                        character.data = params.clone();
                        character.save(db.clone()).await?;

                        // Update avatar
                        let mut data = Vec::new();
                        let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                        params.write(&mut writer)?;

                        let mut avatar_update = CPktAvatarUpdate::default();
                        avatar_update.full_update = false;
                        avatar_update.avatar_id = Some(state.avatar_id.as_u64());
                        avatar_update.update_source = 0;
                        avatar_update.param_bytes = data.len() as u32;
                        avatar_update.params = data;
                        
                        self.send(&state.peer_id, avatar_update.as_message()).await?;
                        //let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;
                    },
                    _ => panic!(),
                }
            },
            _ => {
                info!("Unknown avatar behavior: {:#?}", pkt);
                todo!();
            }
        }

        Ok(())
    }
}