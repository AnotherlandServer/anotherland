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
