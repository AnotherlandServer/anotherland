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

use std::time::Duration;

use raknet::RakNetListener;
use tokio::time::sleep;
use log::debug;

#[toolkit::service_main]
async fn main() {
    let mut listener = RakNetListener::bind("0.0.0.0:6112").await.unwrap();
    listener.listen(100).await;

    loop {
        let socket = listener.accept().await;
        debug!("Got new connection!");
        sleep(Duration::from_millis(10)).await;
    }
}
