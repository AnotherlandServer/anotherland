use std::{io, error::Error, fmt::Display};

use nom::error::VerboseError;

use crate::raknet::{RakNetPeerHandle, RakNetError};

#[derive(Debug)]
pub enum AnotherlandErrorKind {
    RakNetError,
    DBError,
    IOError,
}

impl AnotherlandErrorKind {
    fn as_str(&self) -> &'static str {
        use AnotherlandErrorKind::*;
        match *self {
            RakNetError => "raknet error",
            DBError => "db error",
            IOError => "io error",
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

impl From<surrealdb::Error> for AnotherlandError {
    fn from(value: surrealdb::Error) -> Self {
        Self::new(AnotherlandErrorKind::DBError, value)
    }
}

impl From<io::Error> for AnotherlandError {
    fn from(value: io::Error) -> Self {
        Self::new(AnotherlandErrorKind::IOError, value)
    }
}


pub type AnotherlandResult<T> = Result<T, AnotherlandError>;