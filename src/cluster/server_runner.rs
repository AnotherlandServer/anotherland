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

use std::{time::Duration, any::type_name};

use log::error;
use tokio::{task::JoinHandle, sync::mpsc, sync::mpsc::Sender, spawn, select, time};
use futures::{future::join_all, stream::FuturesUnordered, StreamExt};

use crate::util::AnotherlandResult;

use super::{server_instance::ServerInstance, connect_queue, MessageQueueConsumer, ClusterMessage};

#[allow(unused)]
enum ServerCommand {
    Stop,
}

#[allow(unused)]
pub struct ServerRunner
{
    task: JoinHandle<()>,
    cmd_channel: Sender<ServerCommand>,
}

struct CombinedMessageQueues {
    queues: Vec<MessageQueueConsumer>
}

impl CombinedMessageQueues {
    pub fn new(queues: Vec<MessageQueueConsumer>) -> Self {
        /*let (producer, consumer) = mpsc::channel::<ClusterMessage>(16);

        for mut q in queues {
            let producer = producer.clone();

            tokio::task::spawn_local(async move {
                loop {
                    debug!("CombinedMessageQueue iteration");
                    producer.send(q.recv().await.unwrap()).await.unwrap();
                }
            });
        }*/

        Self { queues }
    }

    pub async fn recv(&mut self) -> AnotherlandResult<ClusterMessage> {
        let mut futures = FuturesUnordered::new();
        for q in self.queues.iter_mut() {
            futures.push(q.recv());
        }

        futures.next().await.unwrap()


        //Ok(self.consumer.recv().await.ok_or(AnotherlandError::from_kind(AnotherlandErrorKind::MessageQueueError))?)
    }
}

impl ServerRunner
    
{
    pub fn new<T>(properties: T::ServerProperties) -> Self
        where T: 'static + ServerInstance + Send
    {
        let (cmd_sender, mut cmd_receiver) = mpsc::channel(10usize);

        Self {
            task: spawn(async move {
                let (mut server, mut cluster) = {
                    let mut startup_retry = time::interval(Duration::from_millis(5000));
                    let server;
                    let cluster;

                    loop {

                        /*let cluster_inst = match MessageQueue::connect().await {
                            Ok(queue) => queue,
                            Err(e) => {
                                error!("[{}] Cluster connection failed: {:#?}", type_name::<T>(), e);
                                startup_retry.tick().await;
                                continue;
                            }
                        };*/

                        match T::init(&properties).await {
                            Err(e) => {
                                error!("[{}] Failed to start: {:#?}", type_name::<T>(), e);
                                startup_retry.tick().await;
                            },
                            Ok(instance) => {
                                // Connect cluster message queues
                                let futures: Vec<_> = instance.get_subscribed_channels().into_iter().map(|v| connect_queue(v)).collect();
                                let channels: Result<Vec<_>, _> = join_all(futures).await.into_iter().collect();

                                let channels: Vec<_> = match channels {
                                    Ok(queue) => queue.into_iter().map(|q| q.1).collect(),
                                    Err(e) => {
                                        error!("[{}] Cluster connection failed: {:#?}", type_name::<T>(), e);
                                        startup_retry.tick().await;
                                        continue;
                                    }
                                };

                                server = instance;
                                cluster = channels;
                                break;
                            },
                        }
                    }

                    (server, CombinedMessageQueues::new(cluster))
                };

                let mut tick = time::interval(T::tickrate()); // Duration::from_millis((1000.0 / 30.0) as u64)
                let has_listener = server.raknet_listener().is_some();

                if has_listener {
                    loop {
                        select! {
                            request = server.raknet_listener().unwrap().next_request() => {
                                match request {
                                    Some(request) => {
                                        if let Err(e) = server.handle_request(request).await {
                                            error!("[{}] Request handler failed: {:#?}", type_name::<T>(), e);
                                        }
                                    },
                                    None => (),/*{
                                        error!("[{}] Request recv failed: {:#?}", type_name::<T>(), e);
                                    }*/
                                }
                            },
                            _ = tick.tick() => {
                                if let Err(e) = server.tick().await {
                                    error!("[{}] Sever tick failed: {:#?}", type_name::<T>(), e);
                                }
                            },
                            Ok(message) = cluster.recv() => {
                                if let Err(e) = server.handle_cluster_message(message).await {
                                    error!("[{}] Cluster message handler failed: {:#?}", type_name::<T>(), e);
                                }
                            }
                            cmd = cmd_receiver.recv() => {
                                match cmd {
                                    None |Some(ServerCommand::Stop) => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    loop {
                        select! {
                            _ = tick.tick() => {
                                if let Err(e) = server.tick().await {
                                    error!("[{}] Sever tick failed: {:#?}", type_name::<T>(), e);
                                }
                            },
                            Ok(message) = cluster.recv() => {
                                if let Err(e) = server.handle_cluster_message(message).await {
                                    error!("[{}] Cluster message handler failed: {:#?}", type_name::<T>(), e);
                                }
                            }
                            cmd = cmd_receiver.recv() => {
                                match cmd {
                                    None |Some(ServerCommand::Stop) => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }


                server.close().await;
            }),
            cmd_channel: cmd_sender,
        }
    }

    pub async fn stop(self) {
    }
}