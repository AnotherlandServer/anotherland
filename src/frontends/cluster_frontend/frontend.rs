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

use std::{collections::HashSet, sync::Arc, time::{Duration, UNIX_EPOCH, SystemTime}, net::SocketAddrV4};

use async_trait::async_trait;
use atlas::{oaPktCashItemVendorSyncAcknowledge, oaPktClusterNodeToClient, oaPktFactionResponse, oaPktSKUBundleSyncAcknowledge, raknet::{Message, Priority, RakNetListener, RakNetPeer, Reliability}, AvatarId, CPkt, CPktChannelChat, CPktChat, CPktStream_167_0, CashItemSKUBundleEntry, CashItemSKUItemEntry, CashItemVendorEntry, CpktChatChatType, FactionRelation, FactionRelationList, NativeParam, Uuid};
use log::{warn, error, trace, info, debug};
use rand::random;
use tokio::{select, sync::{mpsc::Receiver, Mutex, RwLock}, time};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{actors::{ChatChannel, Realm, Social, SocialEvent}, cluster::{frontend::Frontend, ActorRef, CommunityMessage}, components::{SessionHandler, SessionRef}, db::{realm_database, CashShopBundle, CashShopItem, CashShopVendor, WorldDef, ZoneDef}, frontends::TravelType, util::AnotherlandResult, ARGS, CONF, NODE};
use crate::db::DatabaseRecord;

use super::{ZoneRouter, ZoneRouterConnection, ZoneRouterMessage};

pub struct ClusterFrontend {
    //listener: RakNetListener,
    avatar_ids: Arc<Mutex<HashSet<AvatarId>>>,
    tasks: TaskTracker,
    
    zone_router: Arc<ZoneRouter>,
}

impl ClusterFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self { 
            //listener: RakNetListener::bind(CONF.frontend.listen_address).await?,
            avatar_ids: Arc::new(Mutex::new(HashSet::new())),
            tasks: TaskTracker::new(),
            zone_router: ZoneRouter::new(),
        })
    }
}

#[async_trait]
impl Frontend for ClusterFrontend {
    fn name(&self) -> &str { "cluster" }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let mut listener = RakNetListener::bind(CONF.frontend.listen_address).await?;

        let mut realm = NODE.get_remote_actor::<Realm>("realm").unwrap();
        let session_handler = SessionHandler::new();

        let mut heartbeat_interval = time::interval(Duration::from_secs(1));

        'accept_loop: loop {
            select! {
                Ok(peer) = listener.accept() => {                   
                    let frontend_session = ClusterFrontendSession::new(peer, session_handler.clone(), self.zone_router.clone(), self.avatar_ids.clone()).await;
                    frontend_session.run(&self.tasks, token.clone());
                },
                _ = heartbeat_interval.tick() => {
                    realm.update_cluster_frontend_address(SocketAddrV4::new(ARGS.external_ip, CONF.frontend.listen_address.port())).await;
                },
                _ = token.cancelled() => break 'accept_loop,
            }
        }

        Ok(())
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> { 
        self.tasks.close();
        self.tasks.wait().await;

        Ok(()) 
    }
}

struct ClusterFrontendSession {
    peer: RakNetPeer,
    session_handler: Arc<RwLock<SessionHandler>>,
    session_ref: Option<SessionRef>,
    zone_router: Arc<ZoneRouter>,
    zone_connection: Option<Arc<ZoneRouterConnection>>,
    avatar_ids: Arc<Mutex<HashSet<AvatarId>>>,
    avatar_id: AvatarId,
    session_id: Option<Uuid>,
    social: ActorRef<Social>,
    social_events: Receiver<SocialEvent>,
}

impl ClusterFrontendSession {
    async fn new(peer: RakNetPeer, session_handler: Arc<RwLock<SessionHandler>>, zone_router: Arc<ZoneRouter>, avatar_ids: Arc<Mutex<HashSet<AvatarId>>>) -> Self {
        // generate new avatar id for this player
        let avatar_id = loop {
            let random_component = random::<u32>();
            let time_component = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;

            let avatar_id = AvatarId::new((time_component as u64) << 40 | (random_component as u64) << 8, atlas::AvatarType::Player);
            let mut avatar_id_container = avatar_ids.lock().await;
            if avatar_id_container.contains(&avatar_id) {
                continue;
            } else {
                avatar_id_container.insert(avatar_id);
                break avatar_id;
            }
        };

        trace!(peer = peer.id().to_string(), avatar_id = avatar_id.to_string(); "Started cluster session");

        let mut social = NODE.get_actor("social").expect("Social actor missing");
        let social_events = social.register_avatar(avatar_id).await;

        Self {
            peer,
            session_handler: session_handler.to_owned(),
            session_ref: None,
            zone_router,
            zone_connection: None,
            avatar_ids,
            avatar_id,
            session_id: None,
            social,
            social_events,
        }
    }

    async fn receive_from_zone(zone: Option<Arc<ZoneRouterConnection>>) -> Option<ZoneRouterMessage> {
        if let Some(zone_connection) = zone.as_ref() {
            zone_connection.receive().await
        } else {
            None
        }
    }

    async fn travel(&mut self, zone: &Uuid, travel: TravelType) -> AnotherlandResult<()> {
        let db = realm_database().await;

        if let Some(target_zone) = ZoneDef::get(db.clone(), zone).await? {   
            let target_world = WorldDef::get_by_guid(db.clone(), &target_zone.worlddef_guid).await?.unwrap();

            // disconnect current zone
            self.zone_connection.take();

            // update session
            let mut session_s = self.session_ref.as_ref().unwrap().lock().await;
            session_s.select_world(target_world.id).await?;
            session_s.select_zone(target_zone.guid).await?;

            // connect to new zone
            let connection = self.zone_router.connect_zone(&target_zone.guid, &session_s.session().id, &self.avatar_id).await?;

            trace!(
                peer = self.peer.id().to_string(), 
                session_id = self.session_id.map(|v| v.to_string()), 
                avatar_id = self.avatar_id.to_string(); 
                "Connected to zone, notify travel");

            if let Err(e) = connection.notify_travel(travel).await {
                error!(
                    peer = self.peer.id().to_string(), 
                    session_id = self.session_id.map(|v| v.to_string()), 
                    avatar_id = self.avatar_id.to_string(); 
                    "Failed to travel to zone: {:#?}", e);
                self.peer.disconnect().await;
            }

            self.zone_connection = Some(Arc::new(connection));

            Ok(())
        } else {
            // todo: inform the player, that the travel destination was invalid

            Ok(())
        }
    }

    fn run(mut self, tasks: &TaskTracker, token: CancellationToken) {
        tasks.spawn(async move {
            'net_loop: loop {
                let current_zone = self.zone_connection.clone();

                select! {
                    result = self.peer.recv() => {
                        if let Ok(message) = result {
                            if let Err(e) = self.handle_message(message).await {
                                error!(
                                    peer = self.peer.id().to_string(), 
                                    avatar_id = self.avatar_id.to_string(); 
                                    "Failed to handle client message: {:#?}", e);
                            }
                        } else {
                            break 'net_loop;
                        }
                    },
                    message = Self::receive_from_zone(current_zone), if current_zone.is_some() => {
                        match message {
                            Some(ZoneRouterMessage::Message(message)) => {
                                let _ = self.peer.send(Priority::High, Reliability::Reliable, message).await;    
                            },
                            Some(ZoneRouterMessage::TravelRequest { zone, travel }) => {
                                let _ = self.travel(&zone, travel).await;
                            },
                            None => {
                                error!(
                                    peer = self.peer.id().to_string(), 
                                    avatar_id = self.avatar_id.to_string(); 
                                    "Lost connection to zone server");
                                break 'net_loop;
                            }
                        }
                    },
                    Some(event) = self.social_events.recv() => {
                        match event {
                            SocialEvent::Chat(chat) => {
                                if let ChatChannel::Generic(channel) = chat.channel {
                                    let _ = self.peer.send(Priority::High, Reliability::Reliable, CPktChannelChat {
                                        channel,
                                        message: format!("{}: {}", chat.sender, chat.message),
                                        ..Default::default()
                                    }.into_message()).await;
                                } else {
                                    let _ = self.peer.send(Priority::High, Reliability::Reliable, CPktChat {
                                        chat_type: match chat.channel {
                                            ChatChannel::Party => CpktChatChatType::Party,
                                            ChatChannel::Clan => CpktChatChatType::Clan,
                                            ChatChannel::Officer => CpktChatChatType::Officer,
                                            ChatChannel::Whisper { .. } => CpktChatChatType::Whisper,
                                            _ => unreachable!(),
                                        },
                                        sender: chat.sender,
                                        receiver: if let ChatChannel::Whisper { receiver } = chat.channel {
                                            receiver
                                        } else {
                                            String::default()
                                        },
                                        message: chat.message,
                                        ..Default::default()
                                    }.into_message()).await;
                                }
                            }
                        }
                    },
                    _ = token.cancelled() => {
                        break 'net_loop;
                    }
                }
            }

            // unregister from social actor
            self.social.unregister_avatar(self.avatar_id).await;

            self.zone_connection.take();
            self.peer.disconnect().await;

            // once the client disconnected here, the session is no longer needed (and wanted)
            if let Some(session_id) = self.session_id {
                let _ = self.session_handler.write().await.destroy_session(session_id).await;
            }

            // free avatar_id
            self.avatar_ids.lock().await.remove(&self.avatar_id);
        });
    }

    async fn handle_message(&mut self, message: Message) -> AnotherlandResult<()> {
        use atlas::raknet::Message::*;

        match message {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => {
                match self.session_handler.write().await.initiate(*self.peer.id(), pkt.session_id, pkt.magic_bytes.clone()).await {
                    Ok(session) => {
                        self.session_ref = Some(session.clone());

                        let session = session.lock().await;
                        self.session_id = Some(session.session().id);

                        self.social.update_avatar(self.avatar_id, session.session().id).await?;
                        
                        if let Some(zone) = session.session().zone_guid.as_ref() {
                            trace!(
                                peer = self.peer.id().to_string(), 
                                session_id = self.session_id.map(|v| v.to_string()), 
                                avatar_id = self.avatar_id.to_string(); 
                                "Connecting to zone {}", zone);

                            match self.zone_router.connect_zone(zone, &session.session().id, &self.avatar_id).await {
                                
                                Ok(connection) => {
                                    trace!(
                                        peer = self.peer.id().to_string(), 
                                        session_id = self.session_id.map(|v| v.to_string()), 
                                        avatar_id = self.avatar_id.to_string(); 
                                        "Connected to zone, forwarding initial message.");

                                    if let Err(e) = connection.send(&pkt.into_message()).await {
                                        error!(
                                            peer = self.peer.id().to_string(), 
                                            session_id = self.session_id.map(|v| v.to_string()), 
                                            avatar_id = self.avatar_id.to_string(); 
                                            "Failed to forward message to zone: {:#?}", e);
                                        self.peer.disconnect().await;
                                    } else {
                                        self.zone_connection = Some(Arc::new(connection));
                                    }
                                },
                                Err(e) => {
                                    error!(
                                        peer = self.peer.id().to_string(), 
                                        session = pkt.session_id.to_string(); 
                                        "Zone connection failed: {:#?}", e);
                                    self.peer.disconnect().await;
                                }
                            }
                        } else {
                            error!(
                                peer = self.peer.id().to_string(), 
                                session = pkt.session_id.to_string(); 
                                "No zone selected!");
                            self.peer.disconnect().await;
                        }
                    },
                    Err(e) => {
                        warn!(
                            peer = self.peer.id().to_string(), 
                            session = pkt.session_id.to_string(); 
                            "Failed to initialize session: {:#?}", e);
                        self.peer.disconnect().await;
                    }
                }
            },
            AtlasPkt(CPkt::oaPktFriendRequest(_pkt)) => {
                // todo: Implement friends list
                let mut friend_list = CPktStream_167_0::default();
                friend_list.friend_list.count = 0;

                self.peer.send(Priority::High, Reliability::Reliable, friend_list.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClientServerPing(pkt)) => {
                let pong = pkt.clone();
                self.peer.send(Priority::High, Reliability::Reliable, pong.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunication(pkt)) => {
                info!(
                    peer = self.peer.id().to_string(), 
                    session = self.session_id.map(|v| v.to_string()); 
                    "Unknown communication packet: {:#?}", pkt);
            },
            AtlasPkt(CPkt::oaPktClientToClusterNode(pkt)) => {
                match pkt.field_2 {
                    // Some kind of ping
                    0x5 => {
                        self.peer.send(Priority::High, Reliability::Reliable, oaPktClusterNodeToClient {
                            field_1: Uuid::new(),
                            field_3: NativeParam::Struct(vec![
                                NativeParam::Int(0xa8)
                            ]),
                            ..Default::default()
                        }.into_message()).await?;
                    },
                    _ => {
                        info!("Unknown cluster node packet: {:#?}", pkt);
                        //todo!();
                    }
                }
            },
            AtlasPkt(CPkt::oaPktFactionRequest(pkt)) => {
                let factions = vec![FactionRelation {
                    field_0: Uuid::parse_str("be55863a-03a0-4f2a-807c-b794e84f537c").unwrap(),
                    field_1: "Player".to_owned(),
                    field_2: 6000.0,
                }];

                let faction_list = FactionRelationList {
                    count: factions.len() as u32,
                    factions,
                };

                self.peer.send(Priority::High, Reliability::Reliable, oaPktFactionResponse {
                    field_1: pkt.field_1,
                    field_2: pkt.field_2,
                    field_3: NativeParam::Struct(vec![
                        NativeParam::Buffer(faction_list.to_bytes())
                    ]),
                    ..Default::default()
                }.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktCashItemVendorSyncRequest(_pkt)) => {
                // todo: move into own component
                let vendors = CashShopVendor::list(realm_database().await).await?;

                self.peer.send(Priority::High, Reliability::Reliable, oaPktCashItemVendorSyncAcknowledge {
                    field_1: 8,
                    item_count: vendors.len() as u32,
                    items: vendors.into_iter().map(|v| 
                        CashItemVendorEntry {
                            vendor_id: v.id.to_string(),
                            vendor_name: v.name,
                            bundle_list: v.bundle_list.into_iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","),
                            sku_list: v.sku_list.into_iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","),
                            version: v.version,
                        }).collect(),
                    deleted_ids: vec![],
                    ..Default::default()
                }.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktSKUBundleSyncRequest(_pkt)) => {
                // todo: move into own component
                let items: Vec<_> = CashShopItem::list(realm_database().await).await?
                    .iter()
                    .map(|i| CashItemSKUItemEntry {
                        cash_price: i.cash_price,
                        rental_duration: i.rental_duration,
                        is_in_stock: i.is_in_stock,
                        is_hot: i.is_hot,
                        is_new: i.is_new,
                        version: i.version,
                        is_visible: i.is_visible,
                        is_tradable: i.is_tradable,
                        is_featured: i.is_featured,
                        quantity: i.quantity,
                        discount: i.discount,
                        sku_id: i.id.to_string(),
                        display_name: i.display_name.to_owned(),
                        description: i.description.to_owned(),
                        reference_item_name: i.reference_item_name.to_owned(),
                        reference_item_guid: i.reference_item_guid.to_string(),
                        sku_code: i.sku_code.to_owned(),
                        date_start: String::from("invalid"),
                        date_end: String::from("invalid"),
                    })
                    .collect();

                let bundles: Vec<_> = CashShopBundle::list(realm_database().await).await?
                    .iter()
                    .map(|b| CashItemSKUBundleEntry {
                        cash_price: b.cash_price,
                        is_in_stock: b.is_in_stock,
                        is_hot: b.is_hot,
                        is_new: b.is_new,
                        version: b.version,
                        is_visible: b.is_visible,
                        is_tradable: b.is_tradable,
                        is_featured: b.is_featured,
                        quantity: b.quantity,
                        discount: b.discount,
                        bundle_id: b.id.to_string(),
                        display_name: b.display_name.to_owned(),
                        description: b.description.to_owned(),
                        icon: b.icon.to_owned(),
                        item_list_and_count: b.item_list_andcount
                            .iter()
                            .map(
                                |(item, count)| format!("{}={}", item, count)
                            )
                            .collect::<Vec<_>>()
                            .join(","),
                        date_start: String::from("invalid"),
                        date_end: String::from("invalid"),
                    })
                    .collect();

                self.peer.send(Priority::High, Reliability::Reliable, oaPktSKUBundleSyncAcknowledge {
                    sku_item_count: items.len() as u32,
                    sku_items: items,
                    bundle_item_count: bundles.len() as u32,
                    bundle_items: bundles,
                    deleted_item_ids: Vec::new(),
                    deleted_bundle_ids: Vec::new(),
                    ..Default::default()
                }.into_message()).await?;
            },
            AtlasPkt(CPkt::CPktChannelChat(pkt)) => {
                self.social.chat(self.avatar_id, ChatChannel::Generic(pkt.channel), pkt.message).await;
            },
            AtlasPkt(CPkt::CPktChat(pkt)) => {
                match pkt.chat_type {
                    CpktChatChatType::Party => self.social.chat(self.avatar_id, ChatChannel::Party, pkt.message).await,
                    CpktChatChatType::Clan => self.social.chat(self.avatar_id, ChatChannel::Clan, pkt.message).await,
                    CpktChatChatType::Officer => self.social.chat(self.avatar_id, ChatChannel::Officer, pkt.message).await,
                    CpktChatChatType::Whisper => self.social.chat(self.avatar_id, ChatChannel::Whisper { receiver: pkt.receiver }, pkt.message).await,
                    _ => {
                        // forward proximity based chat to zone server
                        if let Some(connection) = self.zone_connection.as_ref() {
                            if connection.send(&pkt.into_message()).await.is_err() {
                                error!(
                                    peer = self.peer.id().to_string(), 
                                    session = self.session_id.map(|v| v.to_string()); 
                                    "Forward to zone server failed, closing connection.");
                                self.peer.disconnect().await;
                            }
                        }
                    }
                }
            },
            AtlasPkt(CPkt::oaPktClusterClientToCommunity(pkt)) => {
                debug!("{:#?}", pkt);
                debug!("{:#?}", CommunityMessage::from_native(pkt.field_3.clone())?);

                match CommunityMessage::from_native(pkt.field_3.clone())? {
                    // This is called after character creation, when choosing to play as a "social" character.
                    // No other instances are kown yet.
                    CommunityMessage::SocialTravel { avatar, map, travel } => {
                        if avatar != self.avatar_id {
                            // what are you doing??
                            warn!("Client tried to 'send' an avatar it doesn't has onership of: {}", avatar);
                            self.peer.disconnect().await;
                        } else if travel {
                            let db = realm_database().await;

                            if let Some(target_zone) = ZoneDef::get_by_name(db.clone(), &map).await? {  
                                if let Err(e) = self.travel(&target_zone.guid, TravelType::EntryPoint).await {
                                    let session_s = self.session_ref.as_ref().unwrap().lock().await;
                                    
                                    error!(
                                        peer = self.peer.id().to_string(), 
                                        session = session_s.session().id.to_string(); 
                                        "Zone connection failed: {:#?}", e);
                                    self.peer.disconnect().await;
                                }
                            } else {
                                // todo: inform the player, that the travel destination was invalid
                            }
                        }
                    },
                    CommunityMessage::UnknownA1 { boolean, .. } => {
                        warn!("Unimplemented community message: 0xa1: {}", boolean);
                    },
                    CommunityMessage::Unknown77 { avatar } => {
                        warn!("Unimplemented community message: 0x77: {:#?}", avatar);
                    }
                }
            },
            AtlasPkt(_) => {
                // forward messages to connected zone server
                if let Some(connection) = self.zone_connection.as_ref() {
                    if connection.send(&message).await.is_err() {
                        error!(
                            peer = self.peer.id().to_string(), 
                            session = self.session_id.map(|v| v.to_string()); 
                            "Forward to zone server failed, closing connection.");
                        self.peer.disconnect().await;
                    }
                } else {
                    warn!(
                        peer = self.peer.id().to_string(), 
                        session = self.session_id.map(|v| v.to_string()); 
                        "Client not connected to zone server!");
                    self.peer.disconnect().await;
                }
            },
            _ => {},
        }

        Ok(())
    }
}
