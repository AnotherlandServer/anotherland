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

use std::{marker::PhantomData, any::Any, collections::HashMap, sync::Arc};
use futures::channel::mpsc::Sender;
use tokio::{sync::{mpsc, Mutex}, select, net::{unix::SocketAddr, TcpStream}};

use crate::{cluster::frontend::{Frontend}, util::AnotherlandError};

use super::{actor::{Actor, ActorHandler}, ActorResult, ActorErr};

pub struct ClusterNodeData {
    actors: HashMap<String, ActorEntry>,
    remote_actors: HashMap<String, Arc<RemoteActorNode>>,
    frontend: Vec<mpsc::Sender<()>>,
}

pub struct ClusterNode(Arc<Mutex<ClusterNodeData>>);

struct ActorEntry {
    pub producer: Box<dyn Any + Send + Sync>,
}

enum RemoteActorNode {
    Remote(SocketAddr),
    Local(ActorEntry),
}

impl ClusterNode {
    pub fn new() -> ClusterNode {
        Self (Arc::new(Mutex::new(ClusterNodeData {
            actors: HashMap::new(),
            remote_actors: HashMap::new(),
            frontend: Vec::new(),
        })))
    }

    pub fn add_actor<T>(&self, mut actor: T) -> ActorRef<T>
        where T: 'static + Actor + ActorHandler {

        let name = actor.name().to_owned();

        let (tx, mut rx) = mpsc::channel(10);
        tokio::spawn(async move {
            actor.pre_start().await.expect("actor pre_start failed");
            actor.started().await.expect("actor started failed");

            while let Some(msg) = rx.recv().await {
                let _ = actor.handle_message(msg).await;
            }

            let _ = actor.stopped().await.expect("actor stopped failed");
        });

        let mut data = self.0.blocking_lock();

        // Register remote actors at local node, so they can be used in
        // standalone server mode too, where we only have a single ClusterNode.
        if T::has_remote_actions() {
            data.remote_actors.insert(name.clone(), Arc::new(
                RemoteActorNode::Local( ActorEntry { producer: Box::new(tx.clone()) })
            ));
        }

        data.actors.insert(name,  ActorEntry { producer: Box::new(tx.clone()) });

        ActorRef { 
            channel: tx, 
            phantom: PhantomData::default() 
        }
    }

    pub fn add_frontend<T>(&self, name: &str, mut frontend: T)
        where T: 'static + Frontend {

        let stop_channel = run_frontend(frontend);

        self.0.blocking_lock().frontend.push(stop_channel);
    }

    pub fn get_actor<T>(&self, name: &str) -> Option<ActorRef<T>>
        where T: 'static + Actor + ActorHandler {
        
        if let Some(actor) = self.0.blocking_lock().actors.get(name) {
            if let Some(producer) = actor.producer.downcast_ref::<mpsc::Sender<T::MessageType>>() {
                Some(ActorRef { 
                    channel: producer.clone(), 
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
    where T: 'static + Actor + ActorHandler {
        if T::has_remote_actions() {
            if let Some(actor) = self.0.blocking_lock().remote_actors.get(name).map(|v| v.clone()) {
                Some(RemoteActorRef { 
                    node: actor, 
                    stream: None,
                    phantom: PhantomData::default(), 
                })
            } else {
                None
            }
        } else {
            None
        }
}
}

#[derive(Clone)]
pub struct ActorRef<T: ActorHandler> {
    channel: mpsc::Sender<T::MessageType>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ActorHandler> ActorRef<T> {
    pub async fn send_message(&self, msg: T::MessageType) -> ActorResult<()> {
        self.channel.send(msg).await.map_err(|_| ActorErr::SendError)
    }
}

pub fn run_frontend<T>(mut frontend: T) -> mpsc::Sender<()>
    where T: 'static + Frontend {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            frontend.pre_start().await?;

            select! {
                _ = rx.recv() => { break; },
                _ = frontend.run() => {},
            }

            frontend.stopped().await?;
        }

        Ok::<_,AnotherlandError>(())
    });

    tx
}

pub struct RemoteActorRef<T: ActorHandler> {
    node: Arc<RemoteActorNode>,
    stream: Option<TcpStream>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: 'static + ActorHandler> RemoteActorRef<T> {
    pub async fn send_message(&self, msg: T::MessageType) -> ActorResult<()> {
        match self.node.as_ref() {
            RemoteActorNode::Local(actor_ref) => {
                let channel = actor_ref.producer.downcast_ref::<mpsc::Sender<T::MessageType>>().unwrap();
                channel.send(msg).await.map_err(|_| ActorErr::SendError)
            },
            RemoteActorNode::Remote(addr) => {
                todo!()
            },
        }
        //self.channel.send(msg).await.map_err(|_| ActorErr::SendError)
    }
}