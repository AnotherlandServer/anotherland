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

use async_graphql::{Context, Error, Object, SimpleObject};
use toolkit::types::Uuid;

use super::nodes::Node;

#[derive(Default)]
pub struct InstancesRoot;

#[derive(Default)]
pub struct InstancesMutationRoot;

#[Object]
impl InstancesRoot {
    pub async fn instance(&self, _ctx: &Context<'_>, id: Uuid) -> Result<Option<Instance>, Error> {
        todo!()
    }
}

#[Object]
impl InstancesMutationRoot {
    pub async fn join_instance(&self, _ctx: &Context<'_>, session_id: Uuid, zone_id: Uuid, instance_id: Option<Uuid>) -> Result<Instance, Error> {
        todo!()
    }
}

#[derive(SimpleObject)]
pub struct Instance {
    zone_id: Uuid,
    instance_id: Uuid,
    node: Node,
}