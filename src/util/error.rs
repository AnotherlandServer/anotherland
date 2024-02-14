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

#![allow(dead_code)]

use std::{io, error::Error, fmt::Display};

use bcrypt::BcryptError;
use nom::error::VerboseError;

use atlas::{raknet::RakNetError, ParamError};

#[derive(Debug, Clone, Copy)]
pub enum AnotherlandErrorKind {
    // Module errors
    RakNet,
    DB,
    IO,
    Bcrypt,
    Parse,
    Thread,
    MessageQueue,
    Param,
    QuinnConnect,
    QuinnConnection,
    QuinnRead,
    QuinnWrite,

    // Application errors
    Application,
}

impl AnotherlandErrorKind {
    fn as_str(&self) -> &'static str {
        use AnotherlandErrorKind::*;
        match *self {
            RakNet => "raknet error",
            DB => "db error",
            IO => "io error",
            Bcrypt => "bcrypt error",
            Parse => "parse error",
            Thread => "thread error",
            MessageQueue => "message queue error",
            Param => "param error",
            Application => "application error",
            QuinnConnect => "quinn connect error",
            QuinnConnection => "quinn connection error",
            QuinnRead => "quinn read error",
            QuinnWrite => "quinn write error",
        }
    }
}

impl AnotherlandError {
    pub fn new<E>(kind: AnotherlandErrorKind, error: E) -> Self 
    where
        E: Into<Box<dyn Error + Send + Sync>>
    {
        Self {
            kind,
            error: Some(error.into()),
        }
    }

    pub fn app_err(msg: &str) -> Self {
        Self {
            kind: AnotherlandErrorKind::Application,
            error: Some(msg.into())
        }
    }

    pub fn from_kind(k: AnotherlandErrorKind) -> Self {
        Self {
            kind: k,
            error: None,
        }
    }
}

#[derive(Debug)]
pub struct AnotherlandError {
    kind: AnotherlandErrorKind,
    error: Option<Box<dyn Error + Send + Sync>>,
}

impl AnotherlandError {
    pub fn kind(&self) -> AnotherlandErrorKind { self.kind }
}

impl Display for AnotherlandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_str())
    }
}

impl Error for AnotherlandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.as_ref().map(|v| v.as_ref() as &(dyn Error))
    }
}

impl From<AnotherlandErrorKind> for AnotherlandError {
    fn from(kind: AnotherlandErrorKind) -> Self {
        Self::new(kind, "")
    }
}

impl From<RakNetError> for AnotherlandError {
    fn from(value: RakNetError) -> Self {
        Self::new(AnotherlandErrorKind::RakNet, value)
    }
}

impl From<mongodb::error::Error> for AnotherlandError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::new(AnotherlandErrorKind::DB, value)
    }
}

impl From<io::Error> for AnotherlandError {
    fn from(value: io::Error) -> Self {
        Self::new(AnotherlandErrorKind::IO, value)
    }
}

impl From<BcryptError> for AnotherlandError {
    fn from(value: BcryptError) -> Self {
        Self::new(AnotherlandErrorKind::Bcrypt, value)
    }
}

impl <'a> From<nom::Err<VerboseError<&'a [u8]>>> for AnotherlandError {
    fn from(value: nom::Err<VerboseError<&'a [u8]>>) -> Self {
        Self::new(AnotherlandErrorKind::Parse, value.to_string())
    }
}

impl From<tokio::task::JoinError> for AnotherlandError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::new(AnotherlandErrorKind::Parse, value)
    }
}

impl From<sqlite::Error> for AnotherlandError {
    fn from(value: sqlite::Error) -> Self {
        Self::new(AnotherlandErrorKind::DB, value)
    }
}

impl From<rabbitmq_stream_client::error::ClientError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ClientError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerCloseError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerCloseError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerDeliveryError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerDeliveryError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ProducerCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProducerCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ProducerPublishError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProducerPublishError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::ProtocolError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProtocolError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::StreamCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::StreamCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<rabbitmq_stream_client::error::StreamDeleteError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::StreamDeleteError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<tokio::sync::broadcast::error::RecvError> for AnotherlandError {
    fn from(value: tokio::sync::broadcast::error::RecvError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueue, value)
    }
}

impl From<serde_json::Error> for AnotherlandError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(AnotherlandErrorKind::Parse, value)
    }
}

impl From<ParamError> for AnotherlandError {
    fn from(value: ParamError) -> Self {
        Self::new(AnotherlandErrorKind::Param, value)
    }
}

impl From<quinn::ConnectError> for AnotherlandError {
    fn from(value: quinn::ConnectError) -> Self {
        Self::new(AnotherlandErrorKind::QuinnConnect, value)
    }
}


impl From<quinn::ConnectionError> for AnotherlandError {
    fn from(value: quinn::ConnectionError) -> Self {
        Self::new(AnotherlandErrorKind::QuinnConnection, value)
    }
}

impl From<quinn::ReadError> for AnotherlandError {
    fn from(value: quinn::ReadError) -> Self {
        Self::new(AnotherlandErrorKind::QuinnRead, value)
    }
}

impl From<quinn::WriteError> for AnotherlandError {
    fn from(value: quinn::WriteError) -> Self {
        Self::new(AnotherlandErrorKind::QuinnWrite, value)
    }
}

pub type AnotherlandResult<T> = Result<T, AnotherlandError>;