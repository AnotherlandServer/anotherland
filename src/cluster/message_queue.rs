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

use std::{collections::HashMap, net::SocketAddrV4, fmt::Display};

use atlas::Uuid;

use once_cell::sync::Lazy;
use serde_derive::{Serialize, Deserialize};
use tokio::sync::{broadcast::{Sender, self, Receiver}, RwLock};
use rabbitmq_stream_client::{Environment, Producer, Consumer, NoDedup, types::Message};
use tokio_stream::StreamExt;

use crate::{ARGS, util::AnotherlandResult}; //api_server::schema::Account

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShutdownSubject {
    All,
    Realm(u32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TravelType {
    Direct,
    Portal,
    NonPortal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiRequest {
    CreateAccout {
        name: String,
        email: Option<String>,
        password: String,
    },
    QueryAccount { id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiError {
    NotFound,
    Custom{message: String},
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => f.write_str("not found"),
            ApiError::Custom { message } => f.write_str(message),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiResponse {
    Error(ApiError),
    //Account(Account)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChatType {
    Say,
    Whisper { character: String },
    Shout,
    Party,
    Clan,
    Officer,
    Channel { name: String },
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClusterMessage {
    SessionDestroyed{session_id: Uuid},
    RealmServerHearthbeat{realm_id: u32, name: String, channels: Vec<(u32, f32)>, address: SocketAddrV4},
}

#[derive(Clone)]
pub enum MessageQueueProducer {
    Broadcast(Sender<ClusterMessage>),
    RabbitMq(Producer<NoDedup>),
}

impl MessageQueueProducer {
    pub async fn send(&self, msg: ClusterMessage) -> AnotherlandResult<()> {
        match &self {
            Self::Broadcast(producer) => {
                let _ = producer.send(msg);
                Ok(())
            },
            Self::RabbitMq(producer) => {
                producer.send_with_confirm(Message::builder().body(serde_json::to_string(&msg).unwrap()).build()).await?;
                Ok(())
            }
        }
    }
}

pub enum MessageQueueConsumer {
    Broadcast(Receiver<ClusterMessage>),
    RabbitMq(Consumer),
}

impl MessageQueueConsumer {
    pub async fn recv(&mut self) -> AnotherlandResult<ClusterMessage> {
        match self {
            Self::Broadcast(consumer) => {
                Ok(consumer.recv().await?)
            },
            Self::RabbitMq(consumer) => {
                loop {
                    if let Some(delivery) = consumer.next().await {
                        if let Some(data) = delivery?.message().data() {
                            return Ok(serde_json::from_slice::<ClusterMessage>(data)?);
                        }
                    }
                }
            }
        }
    }
}

static INTERNAL_MESSAGE_QUEUES: Lazy<RwLock<HashMap<String, Sender<ClusterMessage>>>> = Lazy::new(|| {
    /*let (producer, _) = broadcast::channel(16);
    producer*/
    RwLock::new(HashMap::new())
});

#[derive(Serialize, Deserialize)]
pub enum MessageChannel {
    ClusterChannel,
    RealmChannel{realm_id: u32, channel: RealmChannel},

    //ApiFrontend,
    //ClusterApiChannel,
    //RealmApiChannel{realm_id: u32}
}

#[derive(Serialize, Deserialize)]
pub enum RealmChannel {
    //FrontendChannel,
    //GlobalChannel,
    //SocialChannel,
    //WorldChannel{world_id: u16},
    //NodeChannel{zone_guid: Uuid},
    //DungeonChannel{dungeon_id: Uuid},
}

pub async fn connect_queue(channel: MessageChannel) -> AnotherlandResult<(MessageQueueProducer, MessageQueueConsumer)> {
    let channel_name = serde_json::to_string(&channel).unwrap();

    match ARGS.start_command {
        // For standalone servers, use tokio broadcasts instead of rabbitmq
        crate::StartCommand::StandaloneServer { .. } => {
            let mut queues = INTERNAL_MESSAGE_QUEUES.write().await;
            match queues.get(&channel_name) {
                Some(channel) => {
                    Ok((MessageQueueProducer::Broadcast(channel.clone()), MessageQueueConsumer::Broadcast(channel.subscribe())))
                },
                None => {
                    let (producer, consumer) = broadcast::channel(16);

                    queues.insert(channel_name, producer.clone());

                    Ok((MessageQueueProducer::Broadcast(producer), MessageQueueConsumer::Broadcast(consumer)))
                }
            }
        },
        _ => {
            let env = Environment::builder().build().await?;
            let producer = env.producer().build(&channel_name).await?;
            let consumer = env.consumer().build(&channel_name).await?;

            Ok((MessageQueueProducer::RabbitMq(producer), MessageQueueConsumer::RabbitMq(consumer)))
        }
    }
}