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

use cynic::{http::ReqwestExt, MutationBuilder};
use instance_graphql::{JoinInstance, JoinInstanceVariables};
use toolkit::types::Uuid;

use crate::{ClusterNode, RealmApi, RealmApiError, RealmApiResult};

pub struct Instance {
    pub key: Option<Uuid>,
    pub zone: Uuid,
    pub node: ClusterNode,
}

impl Instance {
    fn from_graphql(other: instance_graphql::Instance) -> RealmApiResult<Self> {
        Ok(Instance {
            key: other.key,
            node: ClusterNode::from_graphql(other.node)?,
            zone: other.zone_id,
        })
    }
}

impl RealmApi {
    pub async fn join_instance(&self, session: Uuid, zone: Uuid, instance_key: Option<Uuid>) -> RealmApiResult<Instance> {
        let response = self.0.client
            .post(self.0.base_url.clone())
            .run_graphql(JoinInstance::build(JoinInstanceVariables {
                session_id: session,
                zone_id: zone,
                instance_id: instance_key,
            })).await?;

        if let Some(JoinInstance { join_instance }) = response.data {
            Ok(Instance::from_graphql(join_instance)?)
        } else if let Some(errors) = response.errors {
            Err(RealmApiError::GraphQl(errors))
        } else {
            unreachable!()
        }
    }
}

pub(crate) mod instance_graphql {
    use crate::schema::*;

    use toolkit::types::Uuid;
    use crate::node_graphql::Node;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct JoinInstanceVariables {
        pub instance_id: Option<Uuid>,
        pub session_id: Uuid,
        pub zone_id: Uuid,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct PublicAddress {
        pub __typename: String,
        pub ip: String,
        pub port: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service", graphql_type = "MutationRoot", variables = "JoinInstanceVariables")]
    pub struct JoinInstance {
        #[arguments(sessionId: $session_id, zoneId: $zone_id, instanceId: $instance_id)]
        pub join_instance: Instance,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct InternalAddress {
        pub __typename: String,
        pub ip: String,
        pub port: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(schema = "realm_manager_service")]
    pub struct Instance {
        pub key: Option<Uuid>,
        pub node: Node,
        pub zone_id: Uuid,
    }
}