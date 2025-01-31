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

use std::{fmt, io};

#[derive(Debug)]
pub enum UPKError {
    IOError(std::io::Error),
    NomErr(String),
}

pub type UPKResult<T> = Result<T, UPKError>;

impl From<io::Error> for UPKError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl <E>From<nom::Err<E>> for UPKError where E: fmt::Debug {
    fn from(value: nom::Err<E>) -> Self {
        Self::NomErr(value.to_string())
    }
}