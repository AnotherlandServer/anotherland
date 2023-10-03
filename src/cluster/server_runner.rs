use std::{time::Duration, any::type_name};

use log::error;
use tokio::{task::JoinHandle, sync::mpsc, sync::mpsc::Sender, spawn, select, time};

use super::{server_instance::ServerInstance, MessageQueue};

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

impl ServerRunner
    
{
    pub fn new<T>() -> Self
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
                        // Connect cluster message queue
                        let cluster_inst = match MessageQueue::connect().await {
                            Ok(queue) => queue,
                            Err(e) => {
                                error!("[{}] Cluster connection failed: {:#?}", type_name::<T>(), e);
                                startup_retry.tick().await;
                                continue;
                            }
                        };

                        match T::init().await {
                            Err(e) => {
                                error!("[{}] Failed to start: {:#?}", type_name::<T>(), e);
                                startup_retry.tick().await;
                            },
                            Ok(instance) => {
                                server = instance;
                                cluster = cluster_inst;
                                break;
                            },
                        }
                    }

                    (server, cluster)
                };

                let mut tick = time::interval(Duration::from_millis((1000.0 / 30.0) as u64));
                loop {
                    select! {
                        request = server.next_request() => {
                            match request {
                                Ok(request) => {
                                    if let Some(request) = request {
                                        if let Err(e) = server.handle_request(request).await {
                                            error!("[{}] Request handler failed: {:#?}", type_name::<T>(), e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    error!("[{}] Request recv failed: {:#?}", type_name::<T>(), e);
                                }
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

                server.close().await;
            }),
            cmd_channel: cmd_sender,
        }
    }

    pub async fn stop(self) {
    }
}