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

#![feature(ascii_char)]

mod error;
mod server;
mod client;
mod state;
mod identifier;
mod message;

pub mod notification;

pub use error::*;
pub use server::*;
pub use client::*;
pub use message::*;
pub use notification::*;

pub use zeromq::{self, Endpoint, util::PeerIdentity, Host};
