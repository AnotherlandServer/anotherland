use std::{io, error::Error, fmt::Display};

use bcrypt::BcryptError;
use nom::error::VerboseError;

use atlas::{raknet::RakNetError, ParamError};

#[derive(Debug, Clone, Copy)]
pub enum AnotherlandErrorKind {
    // Module errors
    RakNetError,
    DBError,
    IOError,
    BcryptError,
    ParseError,
    ThreadError,
    MessageQueueError,
    ParamError,

    // Application errors
    ApplicationError,
}

impl AnotherlandErrorKind {
    fn as_str(&self) -> &'static str {
        use AnotherlandErrorKind::*;
        match *self {
            RakNetError => "raknet error",
            DBError => "db error",
            IOError => "io error",
            BcryptError => "bcrypt error",
            ParseError => "parse error",
            ThreadError => "thread error",
            MessageQueueError => "message queue error",
            ParamError => "param error",
            ApplicationError => "application error",
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
            kind: AnotherlandErrorKind::ApplicationError,
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

impl From<RakNetError> for AnotherlandError {
    fn from(value: RakNetError) -> Self {
        Self::new(AnotherlandErrorKind::RakNetError, value)
    }
}

impl From<mongodb::error::Error> for AnotherlandError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::new(AnotherlandErrorKind::DBError, value)
    }
}

impl From<io::Error> for AnotherlandError {
    fn from(value: io::Error) -> Self {
        Self::new(AnotherlandErrorKind::IOError, value)
    }
}

impl From<BcryptError> for AnotherlandError {
    fn from(value: BcryptError) -> Self {
        Self::new(AnotherlandErrorKind::BcryptError, value)
    }
}

impl From<nom::Err<VerboseError<&'static [u8]>>> for AnotherlandError {
    fn from(value: nom::Err<VerboseError<&'static [u8]>>) -> Self {
        Self::new(AnotherlandErrorKind::ParseError, value)
    }
}

impl From<tokio::task::JoinError> for AnotherlandError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::new(AnotherlandErrorKind::ParseError, value)
    }
}

impl From<sqlite::Error> for AnotherlandError {
    fn from(value: sqlite::Error) -> Self {
        Self::new(AnotherlandErrorKind::DBError, value)
    }
}

impl From<rabbitmq_stream_client::error::ClientError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ClientError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerCloseError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerCloseError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ConsumerDeliveryError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ConsumerDeliveryError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ProducerCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProducerCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ProducerPublishError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProducerPublishError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::ProtocolError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::ProtocolError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::StreamCreateError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::StreamCreateError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<rabbitmq_stream_client::error::StreamDeleteError> for AnotherlandError {
    fn from(value: rabbitmq_stream_client::error::StreamDeleteError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<tokio::sync::broadcast::error::RecvError> for AnotherlandError {
    fn from(value: tokio::sync::broadcast::error::RecvError) -> Self {
        Self::new(AnotherlandErrorKind::MessageQueueError, value)
    }
}

impl From<serde_json::Error> for AnotherlandError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(AnotherlandErrorKind::ParseError, value)
    }
}

impl From<ParamError> for AnotherlandError {
    fn from(value: ParamError) -> Self {
        Self::new(AnotherlandErrorKind::ParamError, value)
    }
}

pub type AnotherlandResult<T> = Result<T, AnotherlandError>;