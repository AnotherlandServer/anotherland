use std::{time::{Duration, SystemTime, UNIX_EPOCH}, collections::HashMap, fs, sync::Arc, io, net::SocketAddrV4};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite, ToByteStream};
use bson::doc;
use glam::{Vec3, Vec4, Quat};
use log::{info, debug, trace, error, warn, as_serde};
use mongodb::Database;
use nom::{multi::length_count, number::complete::le_u8};
use once_cell::sync::Lazy;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use tokio::{sync::RwLock, task::JoinHandle, time::{Interval, self, Instant}};
use log::kv::{ToValue, Value};

use crate::{CONF, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue}, WORLD_SERVER_IDS, db::{WorldDef, DatabaseRecord, realm_database, Account, Session, cluster_database, Character, Content, CashShopVendor, CashShopBundle, CashShopItem, ItemContent, ZoneDef}, ARGS, util::{AnotherlandError, AnotherlandErrorKind::ApplicationError}};
use raknet::*;
use atlas::*;
use crate::util::AnotherlandResult;

use super::community_messages::CommunityMessage;

#[derive(Clone)]
struct ClientState {
    account: Account,
    session: Session,
    peer: RakNetPeerHandle,
    remote_address: PeerAddress,
}

impl Serialize for ClientState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut state = serializer.serialize_struct("Client", 3)?;
        state.serialize_field("account", &self.account.username)?;
        state.serialize_field("session", &self.session.id)?;
        state.serialize_field("remote_address", &self.remote_address)?;
        state.end()
    }
}

impl ToValue for ClientState {
    fn to_value(&self) -> Value<'_> {
        Value::from_serde(self)
    }
}

pub struct FrontendServer {
    listener: RakNetListener,
    //worlddef: WorldDef,
    //world: World,

    cluster_db: Database,
    realm_db: Database,
    realm_id: u32,

    client_state: HashMap<Uuid, ClientState>,
    zone_channels: HashMap<Uuid, MessageQueueProducer>,
    cluster: MessageQueueProducer,
    //world_servers: HashMap<u16, Mess

    /*client_loadstate: u32,
    last_loadstate: u32,
    last_update: Instant,

    hp_test: i32,
    move_mgr: Option<Vec<u8>>,

    clients: HashMap<Uuid, RakNetPeerHandle>,*/
}

impl FrontendServer {
    async fn authenticate_request(&mut self, request: &RakNetRequest) -> AnotherlandResult<(Uuid, ClientState)> {
        let peer_id = request.peer().read().await.guid().to_owned();

        // Do we have a client state?
        if self.client_state.contains_key(&peer_id) {
            return Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()));
        }

        // Does the message contain a session id?
        use Message::*;
        let session_id = match request.message() {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => Ok(pkt.session_id.clone()),
            _ => Err(AnotherlandError::new(ApplicationError, "message without session id"))
        }?;

        // Lookup session
        match Session::get(self.cluster_db.clone(), &session_id).await? {
            Some(session) => {
                // validate world id
                if session.world_id.is_none() {
                    return Err(AnotherlandError::new(ApplicationError, "no world selected"));
                }

                // validate a character is selected
                if session.character_id.is_none() {
                    return Err(AnotherlandError::new(ApplicationError, "no character selected"));
                }

                self.client_state.insert(peer_id.clone(), ClientState { 
                    account: Account::get_by_id(self.cluster_db.clone(), &session.account).await?.unwrap(), 
                    session,
                    peer: request.peer(),
                    remote_address: request.peer().read().await.remote_address().to_owned()
                });

                Ok((peer_id.clone(), self.client_state.get(&peer_id).unwrap().clone()))
            },
            None => {
                Err(AnotherlandError::new(ApplicationError, "unknown session"))
            }
        }
    }
}

#[async_trait]
impl ServerInstance for FrontendServer {
    type ServerProperties = ();

    async fn init(_: &Self::ServerProperties) -> AnotherlandResult<Box<Self>> {
        info!("Starting frontend server...");

        // Connect to world channels, so we can communicate towards them
        let mut zone_channels = HashMap::<Uuid, MessageQueueProducer>::new();
        let zones = ZoneDef::list(realm_database().await).await?.into_iter().map(|z| z.guid);
        for zone_guid in zones {
            zone_channels.insert(zone_guid.clone(), connect_queue(MessageChannel::RealmChannel { 
                realm_id: CONF.realm.id, 
                channel: RealmChannel::ZoneChannel { zone_guid }
            }).await?.0);
        }
        /*let worlds = WorldDef::list(realm_database().await).await?.into_iter().map(|w| w.id);
        for world_id in worlds {
            world_channels.insert(world_id, connect_queue(MessageChannel::RealmChannel { 
                realm_id: CONF.realm.id, 
                channel: RealmChannel::WorldChannel { world_id } 
            }).await?.0);
        }*/

        // Start server
        let mut listener = RakNetListener::new();
        listener.listen(CONF.frontend.listen_address).await?;

        let (cluster, _) = connect_queue(MessageChannel::ClusterChannel).await?;

        Ok(Box::new(Self {
            listener,
            cluster_db: cluster_database().await,
            realm_db: realm_database().await,
            realm_id: CONF.realm.id.into(),
            client_state: HashMap::new(),
            zone_channels,
            cluster
        }))
    }

    async fn close(&mut self) {

    }

    fn raknet_listener(&self) -> Option<&RakNetListener> {
        Some(&self.listener)
    }

    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        let (peer_id, mut state) = match self.authenticate_request(&request).await {
            Ok(state) => state,
            Err(e) => {
                warn!("Failed to authenticate client: {}", e);

                // Close client connection when we can't authenticate them
                request.peer().write().await.disconnect().await;

                return Ok(())
            }
        };

        //println!("Message: {:#?}", request.message());
        match request.message() {
            /*AtlasPkt(CPkt::oaPktRequestEnterGame(_)) => {
                info!(client = state; "Player joining world!");



                debug!("Player joining world {}", self.worlddef.id);

                // Send resource notification 
                let mut worlddef = CPktResourceNotify::default();
                worlddef.resource_type = CpktResourceNotifyResourceType::WorldDef;
                worlddef.field_2 = self.worlddef.guid.clone();
                worlddef.field_3 = "".to_owned();

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, worlddef.as_message()).await?;

                // Lookup character
                let mut character = match Character::get(self.realm_db.clone(), &state.session.character_id.unwrap()).await? {
                    Some(character) => Ok(character),
                    None => Err(AnotherlandError::new(ApplicationError, "selected character not found"))
                }?;

                // Set spawn mode to Login_Normal
                character.data.set_spawn_mode(2);
                character.data.set_client_ready(true);
                character.data.set_player_loading(true);
                character.data.set_player_node_state(2);
                character.data.set_world_map_guid(self.worlddef.guid.clone());

                state.zone_guid = character.data.zone_guid().map(|v| v.to_owned());

                // Spawn avatar
                let zone = self.world.get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap();
                let avatar_id = zone.spawn_avatar(PlayerAvatar::new(character.clone()).into());
                let avatar = zone.get_avatar(&avatar_id).unwrap();

                state.avatar_id = Some(avatar_id.clone());

                // serialize data
                let mut params = Vec::new();
                let mut writer = ByteWriter::endian(&mut params, LittleEndian);
                character.data.write(&mut writer)?;
                let pos = PositionUpdate {
                    pos: avatar.position().into(),
                    rot: avatar.rotation().into(),
                    vel: avatar.velocity().into(),
                }.to_bytes();

                // Transfer character to client
                let mut avatar_blob = CPktBlob::default();
                avatar_blob.avatar_id = avatar_id.as_u64(); // local client always uses 1 for their avatar id
                avatar_blob.avatar_name = character.name.clone();
                avatar_blob.class_id = 77;
                avatar_blob.param_bytes = params.len() as u32;
                avatar_blob.params = params;
                avatar_blob.movement_bytes = pos.len() as u32;
                avatar_blob.movement = pos;
                avatar_blob.has_guid = true;
                avatar_blob.field_9 = Some(state.session.id.clone());

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, avatar_blob.as_message()).await?;
            },*/
            AtlasPkt(CPkt::oaPktFriendRequest(pkt)) => {
                let mut friend_list = CPktStream_167_0::default();
                friend_list.friend_list.count = 0;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, friend_list.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClientServerPing(pkt)) => {
                let response = pkt.clone();
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
            },
            /*AtlasPkt(CPkt::oaPktServerAction(pkt)) => {
                let mut action = pkt.clone();
                action.version = 2;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await?;
            },*/
            /*AtlasPkt(CPkt::oaPktC2SConnectionState(pkt)) => {
                //self.client_loadstate = pkt.field_1;
                state.load_state = match pkt.field_1 {
                    5 => ClientLoadState::RequestAvatarStream,
                    6 => ClientLoadState::StreamedAvatars,
                    7 => ClientLoadState::RequestSpawn,
                    8 => ClientLoadState::Spawned,
                    _ => {
                        warn!("Invalid client loadstate: {}", pkt.field_1);
                        ClientLoadState::EarlyLoadSequence
                    }
                };

                // Confirm loading state
                let mut response = pkt.clone();
                response.field_1 = state.load_state.clone().into();
                response.field_2 = pkt.field_2 + 1;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                
            },*/
            /*AtlasPkt(CPkt::CPktAvatarUpdate(pkt)) => {
                /*if pkt.avatar_id.is_none() {
                    if self.hp_test < 1000 {
                        self.hp_test += 1;
                    }

                    let mut response = CPktAvatarUpdate::default();
                    response.avatar_id = Some(1);
                    response.full_update = false;
                    response.data_len2 = pkt.data_len2;
                    response.field_16 = pkt.field_16.clone();
                    response.field_14 = 1;

                    let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                }*/
            },*/
            /*AtlasPkt(CPkt::oaPktClusterClientToCommunity(pkt)) => {
                debug!("{:#?}", pkt);

                match CommunityMessage::from_native(pkt.field_3.clone())? {
                    CommunityMessage::SocialTravel { avatar, map, travel } => {
                        if &avatar != state.avatar_id.as_ref().unwrap() {
                            // what are you doing??
                            warn!("Client tried to 'send' an avatar it doesn't has onership of: {:#?}", avatar);
                        } else {
                            if travel {
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
                    },
                    CommunityMessage::Unknown_A1 { avatar, boolean } => {
                        warn!("Unabled community message: 0xa1: {}", boolean);
                    }
                }
            },*/
            AtlasPkt(CPkt::oaPktClusterClientToCommunication(pkt)) => {
                match pkt.field_2 {
                    _ => {
                        info!("Unknown communication packet: {:#?}", pkt);
                        //todo!();
                    }
                }
                /*let mut response = oaPktCommunicationToClusterClient::default();
                response.field_1 = pkt.field_1.clone();
                response.field_2 = "Hi!".to_owned();
                response.field_3 = NativeParam::Struct(vec![
                    NativeParam::Int(0),
                    NativeParam::String("Hi!".to_owned()),
                ]);
                response.field_4 = pkt.field_4;
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            AtlasPkt(CPkt::oaPktClienToClusterNode(pkt)) => {
                match pkt.field_2 {
                    // Some kind of ping
                    0x5 => {
                        let mut response = oaPktClusterNodeToClient::default();
                        response.field_1 = Uuid::new_v4();
                        response.field_3 = NativeParam::Struct(vec![
                            NativeParam::Int(0xa8)
                        ]);
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                    },
                    _ => {
                        info!("Unknown cluster node packet: {:#?}", pkt);
                        //todo!();
                    }
                }

            },
            AtlasPkt(CPkt::oaPktFactionRequest(pkt)) => {
                let factions = vec![FactionRelation {
                    field_0: Uuid::from_str("be55863a-03a0-4f2a-807c-b794e84f537c").unwrap(),
                    field_1: "Player".to_owned(),
                    field_2: 6000.0,
                }];

                let faction_list = FactionRelationList {
                    count: factions.len() as u32,
                    factions,
                };

                let mut faction_response = oaPktFactionResponse::default();
                faction_response.field_1 = pkt.field_1;
                faction_response.field_2 = pkt.field_2;
                faction_response.field_3 = NativeParam::Struct(vec![
                    NativeParam::Buffer(faction_list.to_bytes())
                ]);

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, faction_response.as_message()).await?;
            },
            /*AtlasPkt(CPkt::CPktRouted(pkt)) => {
                //println!("Routed pkt {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                match Message::from_bytes(&pkt.field_4).unwrap().1 {
                    AtlasPkt(CPkt::oaPktMoveManagerPosUpdate(pkt)) => {
                        // Update world state
                    
                        /*let avatar = self.world
                            .get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap()
                            .get_avatar_mut(state.avatar_id.as_ref().unwrap()).unwrap();
                        avatar.set_position(pkt.pos.pos.into());
                        avatar.set_rotation(pkt.pos.rot.into());
                        avatar.set_velocity(pkt.pos.vel.into());*/
                    },
                    _ => {
                        warn!("Unhandled routed packet: {:#?}", Message::from_bytes(&pkt.field_4).unwrap());
                    },
                }
            },*/
            AtlasPkt(CPkt::oaPktCashItemVendorSyncRequest(pkt)) => {
                debug!("{:#?}", pkt);

                let vendors = CashShopVendor::list(self.cluster_db.clone()).await?;

                let mut response = oaPktCashItemVendorSyncAcknowledge::default();
                response.field_1 = 8;
                response.item_count = vendors.len() as u32;
                response.items = vendors.into_iter().map(|v| 
                    CashItemVendorEntry {
                        vendor_id: v.id.to_string(),
                        vendor_name: v.name,
                        bundle_list: v.bundle_list.into_iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","),
                        sku_list: v.sku_list.into_iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","),
                        version: v.version,
                    }).collect();
                response.deleted_ids = Vec::new();

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktSKUBundleSyncRequest(pkt)) => {
                //debug!("{:#?}", pkt);

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, oaPktSKUBundleSyncAcknowledge {
                    sku_items: Vec::new(),
                    bundle_items: Vec::new(),
                    deleted_item_ids: Vec::new(),
                    deleted_bundle_ids: Vec::new(),
                    ..Default::default()
                }.as_message()).await?;

                /*let bundle_items = CashShopBundle::list(self.cluster_db.clone()).await?;
                let sku_items = CashShopItem::list(self.cluster_db.clone()).await?;

                let mut response = oaPktSKUBundleSyncAcknowledge::default();

                response.field_1 = 123;
                response.sku_item_count = sku_items.len() as u32;
                response.sku_items = sku_items.into_iter().map(|v|
                    CashItemSKUItemEntry { 
                        cash_price: v.cash_price, 
                        rental_duration: v.rental_duration, 
                        is_in_stock: v.is_in_stock, 
                        is_hot: v.is_hot, 
                        is_new: v.is_new, 
                        version: v.version, 
                        is_visible: v.is_visible, 
                        is_tradable: v.is_tradable, 
                        is_featured: v.is_featured, 
                        quantity: v.quantity, 
                        discount: v.discount, 
                        sku_id: v.id.to_string(), 
                        display_name: v.display_name, 
                        description: v.description, 
                        reference_item_name: v.reference_item_name, 
                        reference_item_guid: v.reference_item_guid.to_string(), 
                        sku_code: v.sku_code, 
                        date_start: v.date_start.map(|d| d.to_rfc3339()).unwrap_or("invalid".to_owned()), 
                        date_end: v.date_end.map(|d| d.to_rfc3339()).unwrap_or("invalid".to_owned()), 
                    }).collect();

                response.bundle_item_count = bundle_items.len() as u32;
                response.bundle_items = bundle_items.into_iter().map(|v|
                    CashItemSKUBundleEntry { 
                        cash_price: v.cash_price, 
                        is_in_stock: v.is_in_stock, 
                        is_hot: v.is_hot, 
                        is_new: v.is_new, 
                        version: v.version, 
                        is_visible: v.is_visible, 
                        is_tradable: v.is_tradable, 
                        is_featured: v.is_featured, 
                        quantity: v.quantity, 
                        discount: v.discount, 
                        bundle_id: v.id.to_string(), 
                        display_name: v.display_name, 
                        description: v.description, 
                        icon: v.icon, 
                        item_list_and_count: v.item_list_andcount.into_iter().map(|v| format!("{}={}", v.0, v.1)).collect::<Vec<String>>().join(","), 
                        date_start: v.date_start.map(|d| d.to_rfc3339()).unwrap_or("invalid".to_owned()),
                        date_end: v.date_end.map(|d| d.to_rfc3339()).unwrap_or("invalid".to_owned()),
                    }).collect();

                response.deleted_item_ids = Vec::new();
                response.deleted_bundle_ids = Vec::new();

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;*/
            },
            /*AtlasPkt(CPkt::oaPktAvatarTellBehaviorBinary(pkt)) => {
                match pkt.field_3.as_str() {
                    "doVendorExecute" => {
                        match &pkt.field_4 {
                            NativeParam::Struct(attrib) => {
                                match self.world
                                    .get_zone_mut(state.zone_guid.as_ref().unwrap()).unwrap()
                                    .get_avatar_mut(&state.avatar_id.as_ref().unwrap()).unwrap() {

                                    Avatar::Player(player) => {
                                        // Update params
                                        {
                                            let params = player.player_param_mut();

                                            params.set_customization_gender(attrib[0].to_f32()?);
                                            params.set_customization_height(attrib[1].to_f32()?);
                                            params.set_customization_fat(attrib[2].to_f32()?);
                                            params.set_customization_skinny(attrib[3].to_f32()?);
                                            params.set_customization_muscular(attrib[4].to_f32()?);
                                            params.set_customization_bust_size(attrib[5].to_f32()?);
                                            params.set_race(attrib[6].to_i32()?);
                                            params.set_customization_brow_angle(attrib[7].to_f32()?);
                                            params.set_customization_eye_brow_pos(attrib[8].to_f32()?);
                                            params.set_customization_eye_pos_spacing(attrib[9].to_f32()?);
                                            params.set_customization_eye_pos(attrib[10].to_f32()?);
                                            params.set_customization_eye_size_length(attrib[11].to_f32()?);
                                            params.set_customization_eye_size_width(attrib[12].to_f32()?);
                                            params.set_customization_eyes_pretty(attrib[13].to_f32()?);
                                            params.set_customization_mouth_pos(attrib[14].to_f32()?);
                                            params.set_customization_mouth_width(attrib[15].to_f32()?);
                                            params.set_customization_mouth_lower_lip_thic(attrib[16].to_f32()?);
                                            params.set_customization_mouth_upper_lip_thic(attrib[17].to_f32()?);
                                            params.set_customization_mouth_expression(attrib[18].to_f32()?);
                                            params.set_customization_nose_pos_length(attrib[19].to_f32()?);
                                            params.set_customization_nose_pos_width(attrib[20].to_f32()?);
                                            params.set_customization_nose_portude(attrib[21].to_f32()?);
                                            params.set_customization_ear_size(attrib[22].to_f32()?);
                                            params.set_customization_ear_elf(attrib[23].to_f32()?);
                                            params.set_customization_cheek_bone(attrib[24].to_f32()?);
                                            params.set_customization_cheek(attrib[25].to_f32()?);
                                            params.set_customization_chin_portude(attrib[26].to_f32()?);
                                            params.set_customization_jaw_chubby(attrib[27].to_f32()?);
                                            debug!("Attrib 28: {}", attrib[28].to_string()?);
                                            debug!("Attrib 29: {:#?}", attrib[29]);
                                            // voucher 28
                                            // int items 29
                                            let mut visible_items = Vec::new();
                                            for a in attrib[30..].iter() {
                                                let item_uuid = a.to_uuid()?;
                                                debug!("Load item {}", item_uuid.to_string());
                                                let item = ItemContent::get(self.realm_db.clone(), &item_uuid).await?;
                                                visible_items.push(item.unwrap().id as i32);
                                            }

                                            if !visible_items.is_empty() {
                                                debug!("set visible item info");
                                                params.set_visible_item_info(visible_items);
                                            } else {
                                                debug!("received empty visible item info after metamorph");
                                            }
                                        }

                                        // Save changes
                                        debug!("Save avatar change");
                                        player.save(self.realm_db.clone()).await?;

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
                                    },
                                    _ => panic!(),
                                }
                            },
                            _ => panic!(),
                        }
                    },
                    _ => {
                        info!("Unknown avatar behavior: {:#?}", pkt);
                        todo!();
                    }
                }
            },*/
            /*AtlasPkt(CPkt::CPktRequestAvatarBehaviors(pkt)) => {
                match pkt.field_3.as_str() {
                    "Travel" => {
                        // todo: validate avatar id and travel location
                        let mut response = oaPktConfirmTravel::default();
                        response.field_2 = 1;
                        let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
                    },
                    _ => {
                        warn!("Unimplemented behavior request: {:#?}", pkt);
                        todo!();
                    }
                }
            },*/
            _ => {
                // Serialize the message and send it to the responsible zone server to deal with
                match self.zone_channels
                    .get(&state.session.zone_guid.as_ref().unwrap()) {

                    Some(zone_channel) => {
                        zone_channel.send(ClusterMessage::Request { 
                            session_id: state.session.id.clone(), 
                            peer_id: peer_id.clone(),
                            data: request.message().to_bytes()
                        }).await?;
                    },
                    None => {
                        warn!("No zone channel for zone {}", state.session.zone_guid.as_ref().unwrap());
                        request.peer().write().await.disconnect().await;
                    }
                }
            }
        }

        self.client_state.insert(peer_id, state);

        Ok(())
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        let mut disconnected_peers = Vec::new();

        for (peer_id, state) in self.client_state.iter_mut() {
            let peer = match self.listener.peer(peer_id).await {
                Some(peer) => peer,
                None => {
                    disconnected_peers.push(peer_id.clone());
                    continue
                },
            };



            /*match state.load_state {
                ClientLoadState::RequestAvatarStream => {
                    /*if let Some(zone) = self.world.get_zone(state.zone_guid.as_ref().unwrap()) {
                        for (id, avatar) in zone.iter() {
                            debug!("Send avatar: {}/{}", avatar.params().class_id().as_u16(), avatar.name());

                            let rot = avatar.rotation();
                            
                            let pos = PositionUpdate {
                                pos: avatar.position().into(),
                                rot: Quat::from_euler(glam::EulerRot::XYZ, rot.x, rot.y, rot.z).into(),
                                vel: avatar.velocity().into(),
                            };

                            let mut buf = Vec::new();
                            let mut writer = ByteWriter::endian(&mut buf, LittleEndian);
                    
                            let _ = writer.write_bytes(&pos.to_bytes());

                            let _ = writer.write(0u8);
                            let _ = writer.write(0u16);
                            let _ = writer.write(0u64);
                            let _ = writer.write(0u64);

                            /*let _ = writer.write(0u64);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u8);
                            let _ = writer.write(0u16);
                            let _ = writer.write(1u16);
                            let _ = writer.write(0u64);*/

                            let mut data = Vec::new();
                            let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                            avatar.params().write(&mut writer)?;

                            let mut avatar_update = CPktAvatarUpdate::default();
                            avatar_update.full_update = true;
                            avatar_update.avatar_id = Some(id.as_u64());
                            avatar_update.field_2 = Some(false);
                            avatar_update.name = Some(avatar.name().clone());
                            avatar_update.class_id = Some(avatar.params().class_id().as_u32());
                            avatar_update.field_6 = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            //avatar_update.flags = Some(2);
                            //avatar_update.flag_2_uuid = Some(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
                            avatar_update.param_bytes = data.len() as u32;
                            avatar_update.params = data;
                            
                            avatar_update.update_source = 0;
                            avatar_update.move_mgr_bytes = Some(buf.len() as u32);
                            avatar_update.move_mgr_data = Some(buf);

                            let _ = peer.write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;
                        }
                    }

                    {
                        state.load_state = ClientLoadState::StreamedAvatars;
                        
                        let mut connectionstate = oaPktS2XConnectionState::default();
                        connectionstate.field_1 = ClientLoadState::StreamedAvatars.into();
                        connectionstate.field_2 = 0;

                        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, connectionstate.as_message()).await;
                    }*/
                },
                ClientLoadState::RequestSpawn => {
                    // Synchronize time
                    {
                        let mut game_time_sync = CPktServerNotify::default();
                        game_time_sync.notify_type = 0;
                        game_time_sync.field_2 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        
                        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, game_time_sync.as_message()).await;
                    }
    
                    {
                        let mut realm_time_sync = CPktServerNotify::default();
                        realm_time_sync.notify_type = 19;
                        realm_time_sync.field_4 = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, realm_time_sync.as_message()).await;
                    }

                    // Update loadstate
                    {
                        state.load_state = ClientLoadState::Spawned;
                        
                        let mut connectionstate = oaPktS2XConnectionState::default();
                        connectionstate.field_1 = ClientLoadState::Spawned.into();
                        connectionstate.field_2 = 0;

                        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, connectionstate.as_message()).await;
                    }             

                    // Tell the client the avatar is ready to spawn
                    /*{
                        let zone = self.world.get_zone(state.zone_guid.as_ref().unwrap()).unwrap();
                        let avatar = zone.get_avatar(state.avatar_id.as_ref().unwrap()).unwrap();

                        let mut action = oaPktServerAction::default();
                        action.action = "TRAVEL:DirectTravel|DirectTravelDefault".to_owned();
                        action.version = 4;
                        action.override_teleport = false;
                        action.pos = avatar.position().into();
                        action.rot = avatar.rotation().into();
                        let _ = peer.write().await.send(Priority::High, Reliability::Reliable, action.as_message()).await;
                    }*/
                },
                ClientLoadState::Spawned => {

                },
                _ => (),
            }*/
        }

        for peer_id in disconnected_peers.iter() {
            /*if let Some(state) = self.client_state.get_mut(peer_id) {
                if let Some(avatar_id) = state.avatar_id.as_ref() {
                    if let Some(zone) = self.world.get_zone_mut(state.zone_guid.as_ref().unwrap()) {
                        zone.despawn_avatar(avatar_id);
                    }
                } 
            }*/
            self.client_state.remove(&peer_id);
        }

        // announce our presence
        self.cluster.send(ClusterMessage::FrontendServerHearthbeat { 
            realm_id: self.realm_id, 
            address: SocketAddrV4::new(ARGS.external_ip.to_string().parse().unwrap(), CONF.frontend.listen_address.port()), 
        }).await?;

        Ok(())
    }

    async fn handle_cluster_message(&mut self, message: ClusterMessage) -> AnotherlandResult<()> {
        match message {
            ClusterMessage::InvalidateSession{session_id} => {
                // Is the session id registered with us?
                match self.client_state.iter().find(|v| v.1.session.id == session_id).map(|v| v.0.clone()) {
                    Some(peer_id) => {
                        // Remove state and close connection
                        if let Some(peer) = self.listener.peer(&peer_id).await {
                            peer.write().await.disconnect().await;
                        }

                        self.client_state.remove(&peer_id);

                        Ok(())
                    },
                    None => Ok(()),
                }
            },
            ClusterMessage::Response { peer_id, data } => {
                // Send message to connected client
                match self.client_state.get(&peer_id) {
                    Some(state) => {
                        state.peer.write().await.send(Priority::High, Reliability::Reliable, Message::from_bytes(&data).unwrap().1).await?;
                        Ok(())
                    },
                    None => Ok(()),
                }
            },
            _ => Ok(())
        }
    }

    fn get_subscribed_channels(&self) -> Vec<MessageChannel> {
        vec![
            MessageChannel::ClusterChannel, 
            MessageChannel::RealmChannel { 
                realm_id: self.realm_id, 
                channel: RealmChannel::GlobalChannel,
            },
            MessageChannel::RealmChannel { 
                realm_id: self.realm_id, 
                channel: RealmChannel::FrontendChannel,
            }
        ]
    }
}