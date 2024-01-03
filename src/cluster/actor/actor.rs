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

use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use futures::Future;
use tokio::sync::mpsc;

use crate::util::AnotherlandResult;

use super::ActorErr;

#[async_trait]
pub trait Actor: Send {
    fn name(&self) -> &str;
    async fn starting(&mut self) -> AnotherlandResult<()> { Ok(()) }
    async fn started(&mut self) -> AnotherlandResult<()> { Ok(()) }

    /// Stopping MUST be cancel safe to avoid blocking the main event loop
    async fn stopping(&mut self) -> AnotherlandResult<()> { Ok(()) }
    async fn stopped(&mut self) -> AnotherlandResult<()> { Ok(()) }
}

#[async_trait]
pub trait ActorHandler {
    type MessageType: Send;
    type RemoteActorHandler;

    async fn handle_message(&mut self, message: Self::MessageType);
    fn has_remote_actions() -> bool;
}
