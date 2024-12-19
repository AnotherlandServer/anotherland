// Copyright (C) 2024 AnotherlandServer
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

use async_graphql::{Context, Error, Object, SimpleObject, ID};
use toolkit::types::Uuid;

use crate::{instance_registry::InstanceRegistry, proto::InstanceKey};

use super::{nodes::{Node, NodesRoot}, QueryRoot};

#[derive(Default)]
pub struct InstancesRoot;

#[derive(Default)]
pub struct InstancesMutationRoot;

#[Object]
impl InstancesRoot {
    pub async fn instance(&self, ctx: &Context<'_>, zone: Uuid, key: Option<Uuid>) -> Result<Option<Instance>, Error> {
        let instances = ctx.data::<InstanceRegistry>()?;
        if let Some(instance) = instances.get_instance(InstanceKey::new(zone, key)).await {
            
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
    pub async fn join_instance(&self, ctx: &Context<'_>, _session_id: Uuid, zone_id: Uuid, instance_id: Option<Uuid>) -> Result<Instance, Error> {
        let instances = ctx.data::<InstanceRegistry>()?;
        let instance = instances.request_instance(InstanceKey::new(zone_id, instance_id)).await?;
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