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

use std::{error::Error, fmt::Display, io};

use nom::error::VerboseError;

#[derive(Debug)]
pub enum RakNetErrorKind {
    InvalidAddressFormat,
    ParserError,
    IOError,
}

impl RakNetErrorKind {
    fn as_str(&self) -> &'static str {
        use RakNetErrorKind::*;
        match *self {
            InvalidAddressFormat => "invalid address format",
            ParserError => "parse error",
            IOError => "io error",
        }
    }
}

#[derive(Debug)]
pub struct RakNetError {
    kind: RakNetErrorKind,
    error: Option<Box<dyn Error + Send + Sync>>,
}

impl RakNetError {
    pub fn new<E>(kind: RakNetErrorKind, error: E) -> Self 
    where
        E: Into<Box<dyn Error + Send + Sync>>
    {
        Self {
            kind,
            error: Some(error.into()),
        }
    }

    pub fn from_kind(k: RakNetErrorKind) -> Self {
        Self {
            kind: k,
            error: None,
        }
    }
}

impl Display for RakNetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_str())
    }
}

impl Error for RakNetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.as_ref().map(|v| v.as_ref() as &(dyn Error))
    }
}

impl From<nom::Err<VerboseError<&'static [u8]>>> for RakNetError {
    fn from(value: nom::Err<VerboseError<&'static [u8]>>) -> Self {
        Self::new(RakNetErrorKind::ParserError, value)
    }
}

impl From<io::Error> for RakNetError {
    fn from(value: io::Error) -> Self {
        Self::new(RakNetErrorKind::IOError, value)
    }
}

impl From<RakNetErrorKind> for RakNetError {
    fn from(value: RakNetErrorKind) -> Self {
        Self::from_kind(value)
    }
}

pub type RakNetResult<T> = Result<T, RakNetError>;
