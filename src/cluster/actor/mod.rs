// Copyright (C) 2023 AnotherlandServer
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

mod actor;
mod error;
mod cluster_node;

pub use actor_macros::*;
pub use actor::*;
pub use error::*;
pub use cluster_node::*;

pub mod common_imports {
    pub use super::ActorRef;
    pub use super::RemoteActorRef;
    pub use super::actor::Actor;
    pub use super::actor::ActorHandler;
    pub use tokio::sync::oneshot;
    pub use poem::async_trait;
    pub use std::marker::PhantomData;
}
