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

mod uuid;
mod avatarid;
mod nativeparam;
mod banner;

pub use nativeparam::*;
pub use macros::*;
pub use banner::*;
pub mod types;
pub mod string_parsers;
pub mod record_pagination;

// reexports
pub use env_logger;
pub use dotenvy;
pub use config;
pub use once_cell;