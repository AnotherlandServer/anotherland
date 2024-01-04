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

use std::{collections::{HashSet, HashMap}, sync::Arc, time::{Duration, Instant, UNIX_EPOCH, SystemTime}, net::SocketAddrV4};

use async_trait::async_trait;
use atlas::{raknet::{RakNetListener, RakNetPeer, Message, Priority, Reliability}, AvatarId, CPkt, CPktStream_167_0, oaPktClusterNodeToClient, NativeParam, FactionRelation, FactionRelationList, oaPktFactionResponse, oaPktCashItemVendorSyncAcknowledge, CashItemVendorEntry, oaPktSKUBundleSyncAcknowledge};
use log::{warn, error, trace, info, debug};
use rand::random;
use tokio::{sync::Mutex, time, select};
use tokio_util::{task::TaskTracker, sync::CancellationToken};
use uuid::Uuid;

use crate::{cluster::{frontend::Frontend, CommunityMessage, ActorRef}, util::AnotherlandResult, CONF, NODE, components::{ZoneRegistry, SessionHandler, Realm, SessionRef}, ARGS, db::{Session, CashShopVendor, realm_database, ZoneDef, WorldDef}, frontends::{ZoneServerClient, ZoneMessage}};
use crate::db::DatabaseRecord;

use super::{ZoneRouter, ZoneRouterConnection};

pub struct ClusterFrontend {
    //listener: RakNetListener,
    avatar_ids: Arc<Mutex<HashSet<AvatarId>>>,
    tasks: TaskTracker,
    
    zone_router: ZoneRouter,
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
        let session_handler = NODE.add_actor(SessionHandler::<()>::initialize("cluster_session_handler").await);

        let mut heartbeat_interval = time::interval(Duration::from_secs(1));

        'accept_loop: loop {
            select! {
                Ok(peer) = listener.accept() => {                   
                    let frontend_session = ClusterFrontendSession::new(peer, &session_handler, self.zone_router.clone(), self.avatar_ids.clone()).await;
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
    session_handler: ActorRef<SessionHandler<()>>,
    session_ref: Option<SessionRef<()>>,
    zone_router: ZoneRouter,
    zone_connection: Option<Arc<ZoneRouterConnection>>,
    avatar_ids: Arc<Mutex<HashSet<AvatarId>>>,
    avatar_id: AvatarId,
    session_id: Option<Uuid>,
}

impl ClusterFrontendSession {
    async fn new(peer: RakNetPeer, session_handler: &ActorRef<SessionHandler<()>>, zone_router: ZoneRouter, avatar_ids: Arc<Mutex<HashSet<AvatarId>>>) -> Self {
        // generate new avatar id for this player
        let avatar_id = loop {
            let random_component = random::<u32>();
            let time_component = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32;

            let avatar_id = AvatarId::new((time_component as u64) << 40 | (random_component as u64) << 8, atlas::AvatarType::Player);
            let mut avatar_id_container = avatar_ids.lock().await;
            if avatar_id_container.contains(&avatar_id) {
                continue;
            } else {
                avatar_id_container.insert(avatar_id.clone());
                break avatar_id;
            }
        };

        trace!(peer = peer.id().to_string(), avatar_id = avatar_id.to_string(); "Started cluster session");

        Self {
            peer,
            session_handler: session_handler.to_owned(),
            session_ref: None,
            zone_router,
            zone_connection: None,
            avatar_ids,
            avatar_id,
            session_id: None,
        }
    }

    async fn receive_from_zone(zone: Option<Arc<ZoneRouterConnection>>) -> Option<Message> {
        if let Some(zone_connection) = zone.as_ref() {
            zone_connection.receive().await
        } else {
            None
        }
    }

    fn run(mut self, tasks: &TaskTracker, token: CancellationToken) {
        tasks.spawn(async move {
            'net_loop: loop {
                let current_zone = self.zone_connection.as_ref().map(|v| v.clone());

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
                        if let Some(message) = message {
                            let _ = self.peer.send(Priority::High, Reliability::Reliable, message).await;
                        } else {
                            error!(
                                peer = self.peer.id().to_string(), 
                                avatar_id = self.avatar_id.to_string(); 
                                "Lost connection to zone server");
                            break 'net_loop;
                        }
                    },
                    _ = token.cancelled() => {
                        break 'net_loop;
                    }
                }
            }

            self.zone_connection.take();
            self.peer.disconnect().await;

            // once the client disconnected here, the session is no longer needed (and wanted)
            if let Some(session_id) = self.session_id {
                let _ = self.session_handler.destroy_session(session_id).await;
            }

            // free avatar_id
            self.avatar_ids.lock().await.remove(&self.avatar_id);
        });
    }

    async fn handle_message(&mut self, message: Message) -> AnotherlandResult<()> {
        use atlas::raknet::Message::*;

        //debug!("{:#?}", message);

        match &message {
            AtlasPkt(CPkt::oaPktRequestEnterGame(pkt)) => {
                match self.session_handler.initiate(self.peer.id().clone(), pkt.session_id.clone(), pkt.magic_bytes.clone()).await {
                    Ok(session) => {
                        self.session_ref = Some(session.clone());

                        let session = session.lock().await;
                        self.session_id = Some(session.session().id.clone().into());
                        
                        if let Some(zone) = session.session().zone_guid.as_ref() {
                            trace!(
                                peer = self.peer.id().to_string(), 
                                session_id = self.session_id.map(|v| v.to_string()), 
                                avatar_id = self.avatar_id.to_string(); 
                                "Connecting to zone {}", zone);

                            match self.zone_router.connect_zone(&zone.to_uuid_1(), &session.session().id.into(), &self.avatar_id).await {
                                
                                Ok(connection) => {
                                    trace!(
                                        peer = self.peer.id().to_string(), 
                                        session_id = self.session_id.map(|v| v.to_string()), 
                                        avatar_id = self.avatar_id.to_string(); 
                                        "Connected to zone, forwarding initial message.");

                                    if let Err(e) = connection.send(&message).await {
                                        error!(
                                            peer = self.peer.id().to_string(), 
                                            session_id = self.session_id.map(|v| v.to_string()), 
                                            avatar_id = self.avatar_id.to_string(); 
                                            "Failed to forward message to zone: {:#?}", e);
                                        self.peer.disconnect().await;
                                    } else {
                                        self.zone_connection = Some(Arc::new(connection));
                                    }
                                    //let _ = connection.send(&message).await;
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
                match pkt.field_2 {
                    _ => {
                        info!(
                            peer = self.peer.id().to_string(), 
                            session = self.session_id.map(|v| v.to_string()); 
                            "Unknown communication packet: {:#?}", pkt);
                    }
                }
            },
            AtlasPkt(CPkt::oaPktClientToClusterNode(pkt)) => {
                match pkt.field_2 {
                    // Some kind of ping
                    0x5 => {
                        let mut response = oaPktClusterNodeToClient::default();
                        response.field_1 = Uuid::new_v4();
                        response.field_3 = NativeParam::Struct(vec![
                            NativeParam::Int(0xa8)
                        ]);

                        self.peer.send(Priority::High, Reliability::Reliable, response.into_message()).await?;
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

                let mut faction_response = oaPktFactionResponse::default();
                faction_response.field_1 = pkt.field_1;
                faction_response.field_2 = pkt.field_2;
                faction_response.field_3 = NativeParam::Struct(vec![
                    NativeParam::Buffer(faction_list.to_bytes())
                ]);

                self.peer.send(Priority::High, Reliability::Reliable, faction_response.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktCashItemVendorSyncRequest(pkt)) => {
                // todo: move into own component
                let vendors = CashShopVendor::list(realm_database().await).await?;

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

                self.peer.send(Priority::High, Reliability::Reliable, response.into_message()).await?;
            },
            AtlasPkt(CPkt::oaPktSKUBundleSyncRequest(_pkt)) => {
                self.peer.send(Priority::High, Reliability::Reliable, oaPktSKUBundleSyncAcknowledge {
                    sku_items: Vec::new(),
                    bundle_items: Vec::new(),
                    deleted_item_ids: Vec::new(),
                    deleted_bundle_ids: Vec::new(),
                    ..Default::default()
                }.into_message()).await?;
            }
            AtlasPkt(CPkt::CPktChat(_pkt)) => {
            },
            AtlasPkt(CPkt::CPktChannelChat(_pkt)) => {
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
                            warn!("Client tried to 'send' an avatar it doesn't has onership of: {:#?}", avatar);
                        } else {
                            if travel {
                                let db = realm_database().await;

                                if let Some(target_zone) = ZoneDef::get_by_name(db.clone(), &map).await? {   
                                    let target_world = WorldDef::get_by_guid(db.clone(), &target_zone.worlddef_guid).await?.unwrap();

                                    // disconnect current zone
                                    self.zone_connection.take();

                                    // update session
                                    let mut session_s = self.session_ref.as_ref().unwrap().lock().await;
                                    session_s.select_world(target_world.id).await?;
                                    session_s.select_zone(target_zone.guid.clone().into()).await?;

                                    // connect to new zone
                                    match self.zone_router.connect_zone(&target_zone.guid.into(), &session_s.session().id.into(), &self.avatar_id).await {
                                
                                        Ok(connection) => {
                                            trace!(
                                                peer = self.peer.id().to_string(), 
                                                session_id = self.session_id.map(|v| v.to_string()), 
                                                avatar_id = self.avatar_id.to_string(); 
                                                "Connected to zone, forwarding initial message.");
        
                                            if let Err(e) = connection.send(&message).await {
                                                error!(
                                                    peer = self.peer.id().to_string(), 
                                                    session_id = self.session_id.map(|v| v.to_string()), 
                                                    avatar_id = self.avatar_id.to_string(); 
                                                    "Failed to forward message to zone: {:#?}", e);
                                                self.peer.disconnect().await;
                                            } else {
                                                
                                            }

                                            self.zone_connection = Some(Arc::new(connection));
                                            //let _ = connection.send(&message).await;
                                        },
                                        Err(e) => {
                                            error!(
                                                peer = self.peer.id().to_string(), 
                                                session = session_s.session().id.to_string(); 
                                                "Zone connection failed: {:#?}", e);
                                            self.peer.disconnect().await;
                                        }
                                    }

                                    // we initiate the travel handshake. First, we inform the target zone server, that
                                    // we'd like to initiate travel. After it has confirmed, we notify the frontend server 
                                    // about the change and remove the client from this zone server.
                                    /*let (dest_server, _) = connect_queue(MessageChannel::RealmChannel { 
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
                                    }).await?;*/
                                } else {
                                    // todo: inform the player, that the travel destination was invalid
                                }
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
                    if let Err(_) = connection.send(&message).await {
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
