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

use std::net::SocketAddr;

use cynic::{http::ReqwestExt, QueryBuilder};
use node_graphql::{NodeAddress, NodeQuery, NodeVariables, NodesQuery};
use toolkit::types::Uuid;

pub use node_graphql::NodeType;

use crate::{RealmApi, RealmApiError, RealmApiResult};

pub struct ClusterNode {
    pub id: Uuid,
    pub ty: NodeType,
    pub addr: ClusterAddress,
}

impl ClusterNode {
    pub(crate) fn from_graphql(other: node_graphql::Node) -> RealmApiResult<Self> {
        Ok(ClusterNode {
            id: other.id,
            ty: other.ty,
            addr: match other.addr {
                NodeAddress::PublicAddress(public_address) => {
                    ClusterAddress::Public(SocketAddr::new(
                        public_address.ip.parse()?, 
                        public_address.port as u16
                    ))
                },
                NodeAddress::InternalAddress(internal_address) => {
                    ClusterAddress::Internal(SocketAddr::new(
                        internal_address.ip.parse()?, 
                        internal_address.port as u16
                    ))
                },
                NodeAddress::Unknown => {
                    unimplemented!()
                },
            }
        })
    }
}

pub enum ClusterAddress {
    Public(SocketAddr),
    Internal(SocketAddr),
}

impl RealmApi {
    pub async fn get_cluster_node(&self, id: &Uuid) -> RealmApiResult<Option<ClusterNode>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(NodeQuery::build(NodeVariables {
                id: &cynic::Id::new(id.to_string())
            })).await?;

        if let Some(NodeQuery { node }) = response.data {
            if let Some(node) = node {
                Ok(Some(ClusterNode::from_graphql(node)?))
            } else {
                Ok(None)
            }
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }

    pub async fn get_cluster_nodes(&self) -> RealmApiResult<Vec<ClusterNode>> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(NodesQuery::build(())).await?;

        if let Some(NodesQuery { nodes }) = response.data {
            Ok(
                nodes.into_iter()
                    .map(ClusterNode::from_graphql)
                    .collect::<RealmApiResult<Vec<ClusterNode>>>()?
            )
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod node_graphql {
    use toolkit::types::Uuid;
    use crate::schema::*;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct NodeVariables<'a> {
        pub id: &'a cynic::Id,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot", variables = "NodeVariables")]
    pub struct NodeQuery {
        #[arguments(id: $id)]
        pub node: Option<Node>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "QueryRoot")]
    pub struct NodesQuery {
        pub nodes: Vec<Node>,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PublicAddress {
        pub __typename: String,
        pub ip: String,
        pub port: i32,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "Node")]
    pub struct Node {
        pub addr: NodeAddress,
        pub id: Uuid,
        pub ty: NodeType,
    }
    
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct InternalAddress {
        pub __typename: String,
        pub ip: String,
        pub port: i32,
    }
    
    #[derive(cynic::InlineFragments, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum NodeAddress {
        PublicAddress(PublicAddress),
        InternalAddress(InternalAddress),
        #[cynic(fallback)]
        Unknown
    }
    
    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub enum NodeType {
        Frontend,
        Cluster,
        World,
    }
}