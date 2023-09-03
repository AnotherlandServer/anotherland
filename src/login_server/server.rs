use std::sync::{Arc};

use log::{info, debug, trace};
use tokio::{sync::RwLock, task::JoinHandle};

use crate::{raknet::{RakNetListener, Message}, util::AnotherlandResult, CONF, atlas::{CPktLogin, CPkt}};

pub struct LoginServer {
    internal: Arc<RwLock<LoginServerInternal>>
}

impl LoginServer {
    pub async fn init() -> AnotherlandResult<Self> {
        Ok(LoginServer {
            internal: LoginServerInternal::init().await?
        })
    }
}

pub struct LoginServerInternal {
    listener: RakNetListener,
    task: Option<JoinHandle<()>>,
}

impl LoginServerInternal {
    pub async fn init() -> AnotherlandResult<Arc<RwLock<Self>>> {
        info!("Starting login server...");

        let mut inst = Arc::new(RwLock::new(Self {
            listener: RakNetListener::new(),
            task: None,
        }));

        inst.write().await.listener.listen(&CONF.login_server.listen_address).await?;

        let task_handle = {
            let inst = inst.clone();

            Some(tokio::spawn(async move {
                while let Some(request) = inst.read().await.listener.next_request().await {
                    use Message::*;
                    match request.message() {
                        AtlasPkt(CPkt::CPktLogin(pkt)) => {
                            debug!("Login request for {}", pkt.username);
                        },
                        _ => trace!("Unhandled request: {:#?}", request.message()),
                    }
                }

                trace!("Stopping login server loop...");
            }))
        };

        inst.write().await.task = task_handle;

        Ok(inst)
    }
}