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

use std::{collections::{HashMap, HashSet}, marker::PhantomData, net::{Ipv6Addr, SocketAddr}, sync::Arc, time::Duration};

use atlas::{AvatarId, Param, ParamSetBox, Uuid};
use glam::Vec3;
use log::{trace, debug};
use nom::AsBytes;
use quinn::{RecvStream, Endpoint, ServerConfig, Connecting, VarInt, Connection, ClientConfig};

use serde::{Serialize, Deserialize};
use tokio::{task::JoinHandle, sync::mpsc::{self, Receiver, Sender}, select};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{actors::AvatarState, util::{AnotherlandErrorKind, AnotherlandResult}};


#[derive(Serialize, Deserialize, Debug)]
pub enum ZoneUpstreamMessage {
    EnterZone { session_id: Uuid, avatar_id: AvatarId },
    Travel { session_id: Uuid, destination: TravelType },
    Message { session_id: Uuid, message: Vec<u8> },
    LeaveZone { session_id: Uuid },
    IngameCommand { session_id: Uuid, command: String },
    ApiCommand(ApiCommand),

    #[serde(skip)]
    SessionApiCommand { downstream: Sender<ZoneDownstreamMessage>, command: ApiCommand }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiCommand {
    GetPlayerAvatarId { session_id: Uuid },
    GetPlayerInterestList { session_id: Uuid },
    GetAvatar { session_id: Uuid, avatar_id: AvatarId },
    GetSelectedAvatar { session_id: Uuid, avatar_id: Option<AvatarId> },
    UpdateAvatarParams { session_id: Uuid, avatar_id: AvatarId, params: HashMap<String, Param> },
}

impl ApiCommand {
    pub fn session_id(&self) -> Option<Uuid> {
        match self {
            ApiCommand::GetPlayerAvatarId { session_id } => Some(*session_id),
            ApiCommand::GetPlayerInterestList { session_id } => Some(*session_id),
            ApiCommand::GetAvatar { session_id, .. } => Some(*session_id),
            ApiCommand::GetSelectedAvatar { session_id, .. } => Some(*session_id),
            ApiCommand::UpdateAvatarParams { session_id, .. } => Some(*session_id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ZoneDownstreamMessage {
    Message { session_id: Uuid, message: Vec<u8> },
    RequestTravel { session_id: Uuid, zone: Uuid, travel: TravelType },
    ApiResult(ApiResult)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiResult {
    Ok,
    Error(String),
    PlayerAvatar(AvatarId),
    PlayerInterestList(Vec<AvatarId>),
    Avatar(AvatarState),
    AvatarId(Option<AvatarId>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TravelType {
    Login,
    Portal{ uuid: Uuid },
    Position{ pos: Vec3, rot: Vec3 },
    EntryPoint,
}

pub struct ZoneServerListener {
    endpoint: Endpoint,
    token: CancellationToken,
    tasks: TaskTracker,
    pending_accept: Option<JoinHandle<AnotherlandResult<ZoneServerClient<ZoneDownstreamMessage, ZoneUpstreamMessage>>>>,
}

impl ZoneServerListener {
    pub async fn listen(mut server_config: ServerConfig, addr: SocketAddr) -> AnotherlandResult<Self> {
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.keep_alive_interval(Some(Duration::from_millis(250)));

        Ok(Self {
            endpoint: Endpoint::server(server_config, addr)?,
            token: CancellationToken::new(),
            tasks: TaskTracker::new(),
            pending_accept: None,
        })
    }

    pub async fn accept(&mut self) -> Option<ZoneServerClient<ZoneDownstreamMessage, ZoneUpstreamMessage>> {
        if let Some(accept) = self.pending_accept.as_mut() {
            let res = accept.await.ok()?;
            self.pending_accept.take();
            res.ok()
        } else if let Some(incoming) = self.endpoint.accept().await {
            // spawn accept logic into it's own task, as this method needs to be 
            // cancel safe.
            self.pending_accept = Some(self.tasks.spawn(async move {
                ZoneServerClient::accept(incoming).await
            }));

            let res = self.pending_accept.as_mut().unwrap().await.ok()?;
            self.pending_accept.take();
            res.ok()
        } else {
            None
        }
    }

    pub async fn close(&mut self) {
        self.token.cancel();
        self.endpoint.close(VarInt::from_u32(0), &[]);

        self.tasks.close();
        self.tasks.wait().await;
    }

    pub fn local_addr(&self) -> AnotherlandResult<SocketAddr> { 
        Ok(self.endpoint.local_addr()?)
    }
}


pub struct ZoneServerClient<S, R> 
    where S: Serialize + Send, R: for<'de> Deserialize<'de> + Send
{
    endpoint: Option<Endpoint>,
    connection: Connection,
    token: CancellationToken,
    tasks: TaskTracker,
    receiver: Receiver<R>,
    _marker_s: PhantomData<S>,
    _marker_r: PhantomData<R>,
}

impl<S, R> ZoneServerClient<S, R> 
    where S: Serialize + Send, R: for<'de> Deserialize<'de> + Send + 'static
{
    fn spawn_task(connection: Connection, tasks: TaskTracker, token: CancellationToken, sender: Sender<R>) {
        tasks.spawn({
            let tasks = tasks.clone();
            async move {
                'accept_loop: loop {
                    select! {
                        res = connection.accept_uni() => {
                            match res {
                                Ok(channel) => {
                                    let sender = sender.clone();
                                    tasks.spawn(async move {
                                        if let Ok(message) = Self::read_message(channel).await {
                                            let _ = sender.send(message).await;
                                        }
                                    });
                                },
                                Err(e) => {
                                    debug!("Error while accepting new messages: {:#?}", e);
                                    break 'accept_loop;
                                }
                            }
                        },
                        _ = token.cancelled() => break 'accept_loop,
                    }                   
                }

                trace!("Zone server connection ended");
            }
        });
    }

    async fn read_message(mut stream: RecvStream) -> AnotherlandResult<R> {
        let mut buffer = Vec::new();

        while let Some(chunk) = stream.read_chunk(usize::MAX, false).await? {
            let computed_size = chunk.bytes.len() + chunk.offset as usize;

            if buffer.len() < computed_size {
                buffer.resize(computed_size, 0);
            }

            buffer[chunk.offset as usize..(chunk.offset as usize + chunk.bytes.len())].copy_from_slice(chunk.bytes.as_bytes());
        }

        Ok(bson::from_slice(buffer.as_slice()).map_err(|_| AnotherlandErrorKind::Parse)?)
    }

    async fn accept(incoming: Connecting) -> AnotherlandResult<ZoneServerClient<S,R>> {
        let connection = incoming.await?;
        let (sender, receiver) = mpsc::channel(100);

        let tasks = TaskTracker::new();
        let token = CancellationToken::new();
        
        // start connection task
        Self::spawn_task(connection.clone(), tasks.clone(), token.clone(), sender);

        Ok(Self{
            endpoint: None,
            connection,
            token: token.clone(),
            tasks,
            receiver,
            _marker_r: PhantomData,
            _marker_s: PhantomData,
        })
    }

    pub async fn connect(addr: SocketAddr) -> AnotherlandResult<Self> {
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(ZoneServerSkipVerification::new())
            .with_no_client_auth();

        let mut endpoint = Endpoint::client(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0))?;
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(config)));

        let connection = endpoint
            .connect(addr, "localhost")?
            .await?;
        let (sender, receiver) = mpsc::channel(100);

        let tasks = TaskTracker::new();
        let token = CancellationToken::new();
        
        // start connection task
        Self::spawn_task(connection.clone(), tasks.clone(), token.clone(), sender);

        Ok(Self{
            endpoint: Some(endpoint),
            connection,
            token: token.clone(),
            tasks,
            receiver,
            _marker_r: PhantomData,
            _marker_s: PhantomData,
        })
    }

    pub async fn send(&self, message: &S) -> AnotherlandResult<()> {
        let connection = self.connection.clone();
        let mut buffer = Vec::new();

        bson::to_bson(message).unwrap()
            .as_document().unwrap()
            .to_writer(&mut buffer).unwrap();

        self.tasks.spawn(async move {
            let mut channel = connection.open_uni().await?;
            channel.write_all(&buffer).await?;
            channel.finish().await?;

            Ok(())
        }).await.unwrap()
    }

    pub async fn recv(&mut self) -> Option<R> {
        self.receiver.recv().await
    }

    pub async fn close(&mut self) {
        // only close endpoint if we are owning the connection
        if let Some(endpoint) = self.endpoint.as_ref() {
            self.token.cancel();
            self.connection.close(VarInt::from_u32(0), &[]);
        
            endpoint.close(VarInt::from_u32(0), &[]);
        }

        self.tasks.close();
        self.tasks.wait().await;
    }
}

pub struct ZoneServerSkipVerification;

impl ZoneServerSkipVerification {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for ZoneServerSkipVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}