use std::{collections::HashMap, net::SocketAddrV4};

use once_cell::sync::Lazy;
use serde_derive::{Serialize, Deserialize};
use tokio::sync::{broadcast::{Sender, self, Receiver}, RwLock};
use rabbitmq_stream_client::{Environment, Producer, Consumer, NoDedup, types::Message};
use tokio_stream::StreamExt;

use crate::{ARGS, util::AnotherlandResult};
use atlas::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShutdownSubject {
    All,
    Realm(u32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClusterMessage {
    Shutdown{subject: ShutdownSubject},
    InvalidateSession{session_id: Uuid},
    RealmServerHearthbeat{realm_id: u32, name: String, population: usize, address: SocketAddrV4},
    FrontendServerHearthbeat{realm_id: u32, address: SocketAddrV4},
    Request{session_id: Uuid, peer_id: Uuid, data: Vec<u8>},
    Response{peer_id: Uuid, data: Vec<u8>},
}

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
}

#[derive(Serialize, Deserialize)]
pub enum RealmChannel {
    FrontendChannel,
    GlobalChannel,
    WorldChannel{world_id: u16},
    DungeonChannel{dungeon_id: Uuid},
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