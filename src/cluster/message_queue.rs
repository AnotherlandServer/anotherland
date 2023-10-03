use once_cell::sync::Lazy;
use serde_derive::{Serialize, Deserialize};
use tokio::sync::broadcast::{Sender, self, Receiver};
use rabbitmq_stream_client::{Environment, Producer, Consumer, NoDedup, types::Message};
use tokio_stream::StreamExt;

use crate::{ARGS, util::AnotherlandResult};
use atlas::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum ClusterMessage {
    InvalidateSession(Uuid),
    RealmShutdown(u32),
    Shutdown(),
}

enum MessageQueueImpl {
    Broadcast { producer: Sender<ClusterMessage>, consumer: Receiver<ClusterMessage> },
    RabbitMq { producer: Producer<NoDedup>, consumer: Consumer },
}

static INTERNAL_MESSAGE_QUEUE: Lazy<Sender<ClusterMessage>> = Lazy::new(|| {
    let (producer, _) = broadcast::channel(16);
    producer
});

pub struct MessageQueue {
    queue: MessageQueueImpl,
}

impl MessageQueue {
    pub async fn connect() -> AnotherlandResult<Self> {
        let queue = match ARGS.start_command {
            // For standalone servers, use tokio broadcasts instead of rabbitmq
            crate::StartCommand::StandaloneServer { .. } => {
                MessageQueueImpl::Broadcast { 
                    producer: INTERNAL_MESSAGE_QUEUE.clone(), 
                    consumer: INTERNAL_MESSAGE_QUEUE.subscribe(),
                }
            },
            _ => {
                let env = Environment::builder().build().await?;
                let producer = env.producer().build("cluster").await?;
                let consumer = env.consumer().build("cluster").await?;

                MessageQueueImpl::RabbitMq { producer, consumer }
            }
        };

        Ok(Self { queue })
    }

    pub async fn send(&self, msg: ClusterMessage) -> AnotherlandResult<()> {
        match &self.queue {
            MessageQueueImpl::Broadcast { producer, .. } => {
                let _ = producer.send(msg);
                Ok(())
            },
            MessageQueueImpl::RabbitMq { producer, .. } => {
                producer.send_with_confirm(Message::builder().body(serde_json::to_string(&msg).unwrap()).build()).await?;
                Ok(())
            }
        }
    }

    pub async fn recv(&mut self) -> AnotherlandResult<ClusterMessage> {
        match &mut self.queue {
            MessageQueueImpl::Broadcast { consumer, .. } => {
                Ok(consumer.recv().await?)
            },
            MessageQueueImpl::RabbitMq { consumer, .. } => {
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
