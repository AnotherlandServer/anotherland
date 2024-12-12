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

use async_graphql::{Context, Enum, Error, Object, OneofObject, SimpleObject, Union, ID};
use toolkit::types::Uuid;

use crate::{node_registry::{self, NodeRegistry, NodeSocketAddress}, proto::{self, NodeType}};

#[derive(Default)]
pub struct NodesRoot;

#[Object]
impl NodesRoot {
    pub async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Node>, Error> {
        let registry = ctx.data::<NodeRegistry>()?;
        Ok(registry.node(id.parse()?).await
            .map(|node| node.into()))
    }

    pub async fn nodes(&self, ctx: &Context<'_>) -> Result<Vec<Node>, Error> {
        let registry = ctx.data::<NodeRegistry>()?;
        Ok(
            registry.nodes().await
                .into_iter()
                .map(|node| node.into())
                .collect()
        )
    }
}

#[derive(SimpleObject)]
pub struct PublicAddress {
    ip: String,
    port: u16,
}

#[derive(SimpleObject)]
pub struct InternalAddress {
    ip: String,
    port: u16,
}

#[derive(Union)]
pub enum NodeAddress {
    PublicAddress(PublicAddress),
    InternalAddress(InternalAddress),
}

#[derive(SimpleObject)]
pub struct Node {
    id: Uuid,
    ty: NodeType,
    addr: NodeAddress,
}

impl From<node_registry::Node> for Node {
    fn from(value: node_registry::Node) -> Self {
        Node {
            id: value.id,
            ty: value.ty,
            addr: match value.addr {
                NodeSocketAddress::Public(addr) => NodeAddress::PublicAddress(PublicAddress { 
                    ip: addr.ip().to_string(), 
                    port: addr.port(), 
                }),
                NodeSocketAddress::Internal(addr) => NodeAddress::InternalAddress(InternalAddress { 
                    ip: addr.ip().to_string(), 
                    port: addr.port(), 
                }),
            },
        }
    }
}
