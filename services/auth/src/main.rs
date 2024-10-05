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

#![feature(let_chains)]

mod auth_session;
mod error;

use auth_session::AuthSessionContext;
use raknet::RakNetListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut listener = RakNetListener::bind("0.0.0.0:6112").await.unwrap();
    listener.generate_random_rsa_key();
    listener.listen(100).await;

    loop {
        let socket = listener.accept().await.unwrap();
        AuthSessionContext::start_auth_session(socket);
    }
}
