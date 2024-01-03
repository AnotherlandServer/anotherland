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

use async_trait::async_trait;
use tokio::{sync::{oneshot, mpsc}, select};
use tokio_util::sync::CancellationToken;

use crate::util::{AnotherlandResult, AnotherlandError};

#[async_trait]
pub trait Frontend: Send + Sync {
    fn name(&self) -> &str;
    async fn starting(&mut self) -> AnotherlandResult<()> { Ok(()) }
    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> { Ok(()) }
    async fn stopped(&mut self) -> AnotherlandResult<()> { Ok(()) }
}
