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

use async_graphql::{EmptySubscription, Schema};
use schema::{MutationRoot, QueryRoot};

mod db;
mod schema;
pub mod proto;
mod realm_status_registry;

pub fn get_schema_sdl() -> String {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
        .sdl()
}