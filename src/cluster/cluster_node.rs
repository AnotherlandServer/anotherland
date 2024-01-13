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

use std::{marker::PhantomData, any::{Any, type_name}, collections::HashMap, sync::{Arc, Mutex}, net::SocketAddr, cell::OnceCell};
use futures::future::join_all;
use log::{debug, trace, error, info};
use tokio::{sync::{mpsc, oneshot, broadcast, watch, Barrier}, select, net::TcpStream, task::JoinSet};
use tokio_util::{task::task_tracker::TaskTracker, sync::CancellationToken};

use crate::{cluster::frontend::{Frontend}, util::{AnotherlandError, AnotherlandResult}, NODE};

use super::{actor::{Actor, ActorHandler, ActorResult, ActorErr}};

pub struct ClusterNodeData {
    actors: HashMap<String, ActorEntry>,
    remote_actors: HashMap<String, Arc<RemoteActorNode>>,
    //frontend: Vec<mpsc::Sender<()>>,
    state_signal_wait: watch::Receiver<ClusterNodeState>,
    state_signal: watch::Sender<ClusterNodeState>,

    starting_nodes: Vec<oneshot::Receiver<()>>,

    subtasks: OnceCell<TaskTracker>,
}

pub struct ClusterNode(Arc<Mutex<ClusterNodeData>>);

struct ActorEntry {
    pub producer: Box<dyn Any + Send + Sync>,
}

enum RemoteActorNode {
    Remote(SocketAddr),
    Local(ActorEntry),
}

enum RemoteActorConnection<T: 'static + ActorHandler> {
    Remote(()),
    Local(mpsc::Sender<T::MessageType>),
}

impl<T: 'static + ActorHandler> Clone for RemoteActorConnection<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Remote(arg0) => Self::Remote(arg0.clone()),
            Self::Local(arg0) => Self::Local(arg0.clone()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
enum ClusterNodeState {
    PreStart,
    Starting,
    ActorsStarted,
    FrontendsStarted,
    Stopping,
}

impl ClusterNode {
    pub fn new() -> ClusterNode {
        let (state_signal, state_signal_wait) = watch::channel(ClusterNodeState::PreStart);

        Self (Arc::new(Mutex::new(ClusterNodeData {
            actors: HashMap::new(),
            remote_actors: HashMap::new(),
            //frontend: Vec::new(),
            state_signal_wait,
            state_signal,

            starting_nodes: Vec::new(),

            subtasks: (||{
                let cell = OnceCell::new(); 
                let _ = cell.set(TaskTracker::new()); 
                cell
            })(),
        })))
    }

    pub fn add_actor<T>(&self, mut actor: T) -> ActorRef<T::ActorType>
        where T: 'static + Actor<ActorType = T> + ActorHandler + Send + Sync {

        let name = actor.name().map(|v| v.to_owned());

        let (tx, mut rx) = mpsc::channel(10);
        let actor_token = CancellationToken::new();

        let actor_ref = ActorRef { 
            channel: tx.clone(), 
            token: Some(actor_token.clone()),
            phantom: PhantomData::default() 
        };

        let mut state_signal_wait = self.0.lock().unwrap().state_signal_wait.clone();
        let mut data = self.0.lock().unwrap();

        if let Some(name) = name.as_ref() {
            trace!("Starting actor {}", name);
        } else {
            trace!("Starting anonymus actor {}", type_name::<T>());
        }

        let (starting_notify, starting_wait) = oneshot::channel();
        if *data.state_signal_wait.borrow() == ClusterNodeState::PreStart {
            data.starting_nodes.push(starting_wait);
        }

        data.subtasks.get_mut().unwrap().spawn({
            let actor_ref = actor_ref.clone();

            async move {
                let mut stopping = false;

                // Wait for node start
                let _ = state_signal_wait.wait_for(|v| *v >= ClusterNodeState::Starting).await;

                // Run start lifecycle
                actor.starting().await?;

                // Notify cluster we've started
                let _ = starting_notify.send(());

                let _ = state_signal_wait.wait_for(|v| *v >= ClusterNodeState::ActorsStarted).await;

                actor.started(actor_ref.clone()).await?;

                'event_loop: loop {
                    tokio::select! {
                        msg = rx.recv() => { 
                            if let Some(msg) = msg {
                                let _ = actor.handle_message(msg).await; 
                            } else {
                                // all messages are handled, stop actor task
                                break 'event_loop;
                            }
                        },
                        _ = actor.stopping(), if stopping => {
                                // close channel to forbid new messages once Actor::stopping returns
                                //rx.close();

                                // we are now done "stopping"
                                stopping = false;
                        },
                        _ = actor_token.cancelled() => {
                            stopping = true;
                        },
                        _ = state_signal_wait.changed() => {
                            if *state_signal_wait.borrow() == ClusterNodeState::Stopping {
                                stopping = true;
                            }
                        },
                    }
                }

                // Run stop lifecycle
                let _ = actor.stopped().await?;

                if let Some(name) = actor.name() {
                    trace!("Stopped actor {}", name);
                } else {
                    trace!("Stopped anonymus actor {}", type_name::<T>());
                }

                Ok::<_,AnotherlandError>(())
            }
        });
        
        if let Some(name) = name {
            // Register remote actors at local node, so they can be used in
            // standalone server mode too, where we only have a single ClusterNode.
            if T::has_remote_actions() {
                data.remote_actors.insert(name.clone(), Arc::new(
                    RemoteActorNode::Local( ActorEntry { producer: Box::new(tx.clone()) })
                ));
            }

            data.actors.insert(name,  ActorEntry { producer: Box::new(tx) });
        }

        actor_ref
    }

    pub fn add_frontend<T>(&self, mut frontend: T)
        where T: 'static + Frontend {

        let mut state_signal_wait = self.0.lock().unwrap().state_signal_wait.clone();
        let (tx, mut rx) = mpsc::channel(1);

        let mut data = self.0.lock().unwrap();

        let name = frontend.name().to_owned();

        trace!("Starting frontend {}", name);

        data.subtasks.get_mut().unwrap().spawn(async move {
            // wait for the node to start
            let _ = state_signal_wait.wait_for(|v| *v >= ClusterNodeState::Starting).await;

            // register ourselves for 2nd starting phase
            let (starting_notify, starting_wait) = oneshot::channel();

            {
                let mut data = NODE.0.lock().unwrap();
                if *data.state_signal_wait.borrow() < ClusterNodeState::ActorsStarted {
                    data.starting_nodes.push(starting_wait);
                }

                drop(data);
            }

            // wait for 2nd starting phase to start
            let _ = state_signal_wait.wait_for(|v| *v >= ClusterNodeState::ActorsStarted).await;

            let token = CancellationToken::new();
            let task_token = token.clone();

            frontend.starting().await?;

            let _ = starting_notify.send(());

            // wait for 2nd starting phase to finish
            let _ = state_signal_wait.wait_for(|v| *v >= ClusterNodeState::FrontendsStarted).await;

            let mut task = tokio::spawn(async move {
                if let Err(e) = frontend.run(task_token).await {
                    error!(frontend = frontend.name(); "Stopped with error: {:#?}", e);
                }
                
                frontend
            });

            // wait for frontend to stop
            loop {
                select! {
                    _ = rx.recv() => token.cancel(),
                    Ok(mut frontend) = &mut task => {
                        frontend.stopped().await?;

                        trace!("Stopped frontend {}", frontend.name());
                        break;
                    },
                }
            }
    
            Ok::<_,AnotherlandError>(())
        });

        let mut state_signal_wait = data.state_signal_wait.clone();

        tokio::spawn(async move {
            loop {
                let _ = state_signal_wait.changed().await;
                let state = state_signal_wait.borrow().to_owned();
                match state{
                    ClusterNodeState::Stopping => {
                        let _ = tx.send(()).await;
                        break;
                    },
                    _ => (),
                }
            }
        });
    }

    pub fn get_actor<T>(&self, name: &str) -> Option<ActorRef<T>>
        where T: 'static + Actor + ActorHandler + Send + Sync {
        
        if let Some(actor) = self.0.lock().unwrap().actors.get(name) {
            if let Some(producer) = actor.producer.downcast_ref::<mpsc::Sender<T::MessageType>>() {
                Some(ActorRef { 
                    channel: producer.clone(), 
                    token: None,
                    phantom: PhantomData::default() 
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_remote_actor<T>(&self, name: &str) -> Option<RemoteActorRef<T>>
    where T: 'static + Actor + ActorHandler + Send + Sync {
        if T::has_remote_actions() {
            if let Some(actor) = self.0.lock().unwrap().remote_actors.get(name).map(|v| v.clone()) {
                let connection = match actor.as_ref() {
                    RemoteActorNode::Local(actor) => {
                        if let Some(producer) = actor.producer.downcast_ref::<mpsc::Sender<T::MessageType>>() {
                            Some(RemoteActorConnection::Local(producer.clone()))
                        } else {
                            None
                        }
                    },
                    RemoteActorNode::Remote(addr) => {
                        todo!()
                    }
                };

                connection.map(|c| {
                    RemoteActorRef { 
                        node: c,
                        phantom: PhantomData::default(), 
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn start(&self) {
        let starting_actors: Vec<_> = self.0.lock().unwrap().starting_nodes.drain(..).collect();
        let _ = self.0.lock().unwrap().state_signal.send(ClusterNodeState::Starting);
        let _ = join_all(starting_actors).await;

        info!("Actors started, starting frontends...");

        let _ = self.0.lock().unwrap().state_signal.send(ClusterNodeState::ActorsStarted);
        let starting_frontends: Vec<_> = self.0.lock().unwrap().starting_nodes.drain(..).collect();
        let _ = join_all(starting_frontends).await;

        let _ = self.0.lock().unwrap().state_signal.send(ClusterNodeState::FrontendsStarted);

        info!("Cluster node started!");
    }

    pub async fn stop(&self) {
        let _ = self.0.lock().unwrap().state_signal.send(ClusterNodeState::Stopping);
    
        // clear local actor references
        self.0.lock().unwrap().actors.clear();
        self.0.lock().unwrap().remote_actors.clear();

        // Wait for all subtasks to stop
        if let Some(subtasks) =  self.0.lock().unwrap().subtasks.take() {
            subtasks.close();
            subtasks.wait().await;
        }
    }
}

pub struct ActorRef<T: ActorHandler + Send + Sync> {
    channel: mpsc::Sender<T::MessageType>,
    token: Option<CancellationToken>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ActorHandler + Send + Sync> ActorRef<T> {
    pub async fn send_message(&self, msg: T::MessageType) -> ActorResult<()> {
        self.channel.send(msg).await.map_err(|_| ActorErr::SendError)
    }

    pub fn stop(self) {
        if let Some(token) = self.token {
            token.cancel();
        } else {
            panic!("Actor can't be stopped!")
        }
    }
}

impl<T: ActorHandler + Send + Sync> Clone for ActorRef<T> {
    fn clone(&self) -> Self {
        Self { channel: self.channel.clone(), token: self.token.clone(), phantom: self.phantom.clone() }
    }
}

pub struct RemoteActorRef<T: 'static + ActorHandler + Send + Sync> {
    node: RemoteActorConnection<T>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: 'static + ActorHandler + Send + Sync> RemoteActorRef<T> {
    pub async fn send_message(&self, msg: T::MessageType) -> ActorResult<()> {
        match &self.node {
            RemoteActorConnection::Local(channel) => {
                channel.send(msg).await.map_err(|_| ActorErr::SendError)
            },
            RemoteActorConnection::Remote(_) => {
                todo!()
            }
        }
    }
}

impl<T: 'static + ActorHandler + Send + Sync> Clone for RemoteActorRef<T> {
    fn clone(&self) -> Self {
        Self { node: self.node.clone(), phantom: self.phantom.clone() }
    }
}