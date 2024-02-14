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

use async_trait::async_trait;
use tokio::{net::TcpListener, io::AsyncWriteExt, select};
use tokio_util::{task::TaskTracker, sync::CancellationToken};

use crate::{cluster::frontend::Frontend, util::AnotherlandResult, CONF};

pub struct LoginQueueFrontend {
    tasks: TaskTracker,
}

impl LoginQueueFrontend {
    pub async fn initialize() -> AnotherlandResult<Self> {
        Ok(Self {
            tasks: TaskTracker::new(),
        })
    }
}

#[async_trait]
impl Frontend for LoginQueueFrontend {
    fn name(&self) -> &str { "login_queue" }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(())
    }

    async fn run(&mut self, token: CancellationToken) -> AnotherlandResult<()> {
        let listener = TcpListener::bind(CONF.login_server.queue_listen_address).await?;

        loop {
            select! { 
                res = listener.accept() => {
                    let (mut connection, _address) = res?;

                    self.tasks.spawn(async move {
                        // These two messages where captured from a real server back in the days.
                        // So far it's only known, that the first word does specify the message length,
                        // and the following some kind of message type.
                        // 0000   0c 00 02 00 13 50 bb 07 01 00 10 01
                        // 0000   0c 00 05 00 13 50 bb 07 01 00 10 01
        
                        // 3rd word - 20499
                        // 4th word - 1979
                        // 5th word - 1
                        // 6th word - 257
         
                        let _ = connection.shutdown().await;
                    });
                },
                _ = token.cancelled() => break Ok(()),
            }
        }         
    }

    async fn stopped(&mut self) -> AnotherlandResult<()> { 
        self.tasks.close();
        self.tasks.wait().await;

        Ok(())
    }
}