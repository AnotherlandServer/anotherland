// Copyright (C) 2025 AnotherlandServer
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

use chrono::{DateTime, Utc};
use cluster::Notification;
use core_api::proto::CoreNotification;
use serde::{Deserialize, Serialize};
use toolkit::types::Uuid;

use crate::node_registry::{Node, NodeSocketAddress};

use super::NodeType;

#[derive(Serialize, Deserialize, Debug)]
pub enum RealmNotification {
    ClusterNotification(CoreNotification),
    NodeAdded(Node),
    NodeRemoved(Uuid),
    InstanceRequested {
        transaction_id: Uuid,
        zone: Uuid,
        key: Option<Uuid>,
        valid_until: DateTime<Utc>,
    },
    ItemStorageUpdated { 
        id: Uuid,
        tag: Option<String>, 
    }
}

impl Notification for RealmNotification {
    fn topic_name(&self) -> &'static str {
        match self {
            RealmNotification::ClusterNotification(notification) => notification.topic_name(),
            RealmNotification::NodeAdded(_) => "cluster.node.added",
            RealmNotification::NodeRemoved(_) => "cluster.node.removed",
            RealmNotification::InstanceRequested { .. } => "realm.instance.request",
            RealmNotification::ItemStorageUpdated { .. } => "realm.item_storage.updated",
        }
    }
}