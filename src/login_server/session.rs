use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::atlas::Uuid;

static STORAGE: Lazy<RwLock<HashMap<Uuid, Arc<RwLock<Session>>>>> = Lazy::new(|| RwLock::new(HashMap::new()));

type SessionHandle = Arc<RwLock<Session>>;

pub struct Session {
    id: Uuid,
    account_id: Uuid,
    active: bool,
}

impl Session {
    pub async fn create(account_id: &Uuid) -> SessionHandle {
        let id = Uuid::new_v4();
        let instance = Arc::new(RwLock::new(Self {
            id: id.clone(),
            account_id: account_id.clone(),
            active: true,
        }));

        STORAGE.write().await.insert(id, instance.clone());
        instance
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn acction_id(&self) -> &Uuid {
        &self.account_id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub async fn end(&mut self) {
        self.active = false;
        STORAGE.write().await.remove(&self.id);
    }
}