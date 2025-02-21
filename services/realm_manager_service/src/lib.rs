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

#![feature(let_chains)]
#![feature(hash_extract_if)]
#![feature(exclusive_wrapper)]

use std::sync::OnceLock;

use async_graphql::{EmptySubscription, Schema};
use chat_router::ChatRouter;
use instance_registry::InstanceRegistry;
use node_registry::NodeRegistry;
use schema::{MutationRoot, QueryRoot};

mod db;
mod schema;
mod node_registry;
mod instance_registry;
mod session_manager;
mod chat_router;
mod item_storage_session;
mod equipment_slots;

pub mod error;
pub mod proto;
use session_manager::SessionManager;

// These have to be defined so everyting can be linked.
// But since the code is never actually called, we don't need
// to initialize them.
pub static NODE_REGISTRY: OnceLock<NodeRegistry> = OnceLock::new();
pub static INSTANCE_REGISTRY: OnceLock<InstanceRegistry> = OnceLock::new();
pub static SESSION_MANAGER: OnceLock<SessionManager> = OnceLock::new();
pub static CHAT_ROUTER: OnceLock<ChatRouter> = OnceLock::new();

pub fn get_schema_sdl() -> String {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
        .sdl()
}