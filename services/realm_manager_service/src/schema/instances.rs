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

use async_graphql::{Context, Error, Object, SimpleObject};
use toolkit::types::Uuid;

use crate::{proto::InstanceKey, INSTANCE_REGISTRY};

use super::nodes::{Node, NodesRoot};

#[derive(Default)]
pub struct InstancesRoot;

#[derive(Default)]
pub struct InstancesMutationRoot;

#[Object]
impl InstancesRoot {
    pub async fn instance(&self, ctx: &Context<'_>, zone: Uuid, key: Option<Uuid>) -> Result<Option<Instance>, Error> {
        if let Some(instance) = INSTANCE_REGISTRY.get().unwrap().get_instance(InstanceKey::new(zone, key)).await {
            
            Ok(Some(Instance { 
                zone_id: instance.key.zone(), 
                key: instance.key.instance(), 
                node: NodesRoot
                    .node(ctx, instance.node.into()).await?
                    .ok_or(Error::new("node not found"))?
            }))
        } else {
            Ok(None)
        }
    }
}

#[Object]
impl InstancesMutationRoot {
    pub async fn join_instance(&self, ctx: &Context<'_>, session_id: Uuid, zone_id: Uuid, instance_id: Option<Uuid>) -> Result<Instance, Error> {
        let instance = INSTANCE_REGISTRY.get().unwrap()
            .request_instance(session_id, InstanceKey::new(zone_id, instance_id)).await?;
        Ok(Instance { 
            zone_id: instance.key.zone(), 
            key: instance.key.instance(), 
            node: NodesRoot
                .node(ctx, instance.node.into()).await?
                .ok_or(Error::new("node not found"))?
        })
    }
}

#[derive(SimpleObject)]
pub struct Instance {
    zone_id: Uuid,
    key: Option<Uuid>,
    node: Node
}
