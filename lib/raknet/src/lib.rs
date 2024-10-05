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

// Otherland uses a modified version of RakNet (3.3)

// Enable unstable features
#![feature(let_chains)]
#![feature(duration_millis_float)]
#![feature(integer_atomics)]

mod definitions;
mod listener;
mod socket;
mod error;
mod packet;
mod buffer;
mod reliability;
mod frame;
mod fragment;
mod encryption;
mod util;

pub use definitions::*;
pub use listener::*;
pub use socket::*;
pub use error::*;
