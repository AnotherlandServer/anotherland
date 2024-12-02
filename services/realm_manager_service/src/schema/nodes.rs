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

use async_graphql::{Context, Enum, Error, Object, SimpleObject};
use toolkit::types::Uuid;

#[derive(Default)]
pub struct NodesRoot;

#[Object]
impl NodesRoot {
    async fn nodes(&self, ctx: &Context<'_>, id: Uuid) -> Result<Vec<Node>, Error> {
        
        todo!()
    }
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Frontend,
    Cluster,
    World,
    Dungeon,
}

#[derive(SimpleObject)]
pub struct Node {
    id: Uuid,
    r#type: NodeType,
    endpoint: String,
}