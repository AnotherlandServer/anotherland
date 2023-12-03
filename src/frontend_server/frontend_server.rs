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

use std::{collections::HashMap, net::SocketAddrV4};

use async_trait::async_trait;
use bitstream_io::{ByteWriter, LittleEndian};
use log::{info, debug, warn};
use mongodb::Database;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use log::kv::{ToValue, Value};

use crate::{CONF, cluster::{ServerInstance, ClusterMessage, MessageChannel, RealmChannel, MessageQueueProducer, connect_queue, ChatType}, db::{DatabaseRecord, realm_database, Account, Session, cluster_database, Character, CashShopVendor, ZoneDef}, ARGS, util::{AnotherlandError, AnotherlandErrorKind::ApplicationError}};
use raknet::*;
use atlas::*;
use crate::util::AnotherlandResult;

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
    social: MessageQueueProducer,
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

    async fn forward_message_to_node(&self, state: &ClientState, request: RakNetRequest) -> AnotherlandResult<()> {
        // Serialize the message and send it to the responsible zone server to deal with
        match self.zone_channels
        .get(&state.session.zone_guid.as_ref().unwrap()) {

            Some(zone_channel) => {
                zone_channel.send(ClusterMessage::Request { 
                    session_id: state.session.id.clone(), 
                    peer_id: state.peer.read().await.guid().clone(),
                    data: request.message().to_bytes()
                }).await?;
            },
            None => {
                warn!("No zone channel for zone {}", state.session.zone_guid.as_ref().unwrap());
                request.peer().write().await.disconnect().await;
            }
        }

        Ok(())
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
                channel: RealmChannel::NodeChannel { zone_guid }
            }).await?.0);
        }

        let (social_channel, _) = connect_queue(MessageChannel::RealmChannel { 
            realm_id: CONF.realm.id, 
            channel: RealmChannel::SocialChannel 
        }).await?;

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
            cluster,
            social: social_channel,
        }))
    }

    async fn close(&mut self) {

    }

    fn raknet_listener(&self) -> Option<&RakNetListener> {
        Some(&self.listener)
    }

    async fn handle_request(&mut self, request: RakNetRequest) -> AnotherlandResult<()> {
        use Message::*;

        let (peer_id, state) = match self.authenticate_request(&request).await {
            Ok(state) => state,
            Err(e) => {
                warn!("Failed to authenticate client: {}", e);

                // Close client connection when we can't authenticate them
                request.peer().write().await.disconnect().await;

                return Ok(())
            }
        };

        match request.message() {
            AtlasPkt(CPkt::oaPktFriendRequest(_pkt)) => {
                let mut friend_list = CPktStream_167_0::default();
                friend_list.friend_list.count = 0;

                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, friend_list.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClientServerPing(pkt)) => {
                let response = pkt.clone();
                let _ = request.peer().write().await.send(Priority::High, Reliability::Reliable, response.as_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunication(pkt)) => {
                match pkt.field_2 {
                    _ => {
                        info!("Unknown communication packet: {:#?}", pkt);
                        //todo!();
                    }
                }
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
            AtlasPkt(CPkt::oaPktSKUBundleSyncRequest(_pkt)) => {
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
            AtlasPkt(CPkt::CPktChat(pkt)) => {
                debug!("{:#?}", pkt);

                match pkt.chat_type {
                    // As say and shout are proximity based, we let the node handle them
                    CpktChatChatType::Say | CpktChatChatType::Shout => {
                        self.forward_message_to_node(&state, request).await?;
                    },
                    CpktChatChatType::Party => {
                        self.social.send(ClusterMessage::SocialEvent { 
                            event: crate::cluster::SocialEvent::Chat { 
                                peer_id: state.peer.read().await.guid().clone(), 
                                chat_type: ChatType::Party, 
                                message: pkt.message.clone()
                            } 
                        }).await?;
                    },
                    CpktChatChatType::Clan => {
                        self.social.send(ClusterMessage::SocialEvent { 
                            event: crate::cluster::SocialEvent::Chat { 
                                peer_id: state.peer.read().await.guid().clone(), 
                                chat_type: ChatType::Clan, 
                                message: pkt.message.clone()
                            } 
                        }).await?;
                    },
                    CpktChatChatType::Officer => {
                        self.social.send(ClusterMessage::SocialEvent { 
                            event: crate::cluster::SocialEvent::Chat { 
                                peer_id: state.peer.read().await.guid().clone(), 
                                chat_type: ChatType::Officer, 
                                message: pkt.message.clone()
                            } 
                        }).await?;
                    },
                    CpktChatChatType::Whisper => {
                        self.social.send(ClusterMessage::SocialEvent { 
                            event: crate::cluster::SocialEvent::Chat { 
                                peer_id: state.peer.read().await.guid().clone(), 
                                chat_type: ChatType::Whisper { character: pkt.receiver.clone() }, 
                                message: pkt.message.clone()
                            } 
                        }).await?;
                    }
                }
            },
            AtlasPkt(CPkt::CPktChannelChat(pkt)) => {
                debug!("{:#?}", pkt);

                self.social.send(ClusterMessage::SocialEvent { 
                    event: crate::cluster::SocialEvent::Chat { 
                        peer_id: state.peer.read().await.guid().clone(), 
                        chat_type: ChatType::Channel { name: pkt.channel.clone() }, 
                        message: pkt.message.clone()
                    } 
                }).await?;
            },
            _ => {
                self.forward_message_to_node(&state, request).await?;
            }
        }

        self.client_state.insert(peer_id, state);

        Ok(())
    }

    async fn tick(&mut self) -> AnotherlandResult<()> {
        let mut disconnected_peers = Vec::new();

        for (peer_id, _state) in self.client_state.iter_mut() {
            let _peer = match self.listener.peer(peer_id).await {
                Some(peer) => peer,
                None => {
                    disconnected_peers.push(peer_id.clone());
                    continue
                },
            };
        }

        // Remove client states and invalidate session
        for peer_id in disconnected_peers.iter() {
            if let Some(state) = self.client_state.remove(&peer_id) {
                self.cluster.send(ClusterMessage::InvalidateSession{session_id: state.session.id.clone()}).await?;
                state.session.delete(self.cluster_db.clone()).await?;
            }
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
            ClusterMessage::ZoneTravelFinished { session_id, avatar_id, world_id, zone_id } => {
                // Lookup the client state by session id
                match self.client_state.iter_mut().find(|v| v.1.session.id == session_id) {
                    Some((_, state)) => {
                        debug!("Notify client about travel finish");

                        // redirect session to new zone id
                        state.session.world_id = Some(world_id);
                        state.session.zone_guid = Some(zone_id);
                        state.session.save(self.realm_db.clone()).await?;

                        // update client avatar 
                        let player = Character::get(self.realm_db.clone(), &state.session.character_id.unwrap()).await?.unwrap();

                        let mut data = Vec::new();
                        let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                        player.data.write(&mut writer)?;
    
                        let mut avatar_update = CPktAvatarUpdate::default();
                        avatar_update.full_update = false;
                        avatar_update.avatar_id = Some(avatar_id.as_u64());
                        avatar_update.update_source = 0;
                        avatar_update.param_bytes = data.len() as u32;
                        avatar_update.params = data;
                        
                        let _ = state.peer.write().await.send(Priority::High, Reliability::Reliable, avatar_update.as_message()).await?;

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