use nom::error::VerboseError;

pub enum Error<'a> {
    InvalidAddressFormat,
    ParserError(nom::Err<VerboseError<&'a[u8]>>),
    TokioError(tokio::io::Error),
}

impl <'a> From<nom::Err<VerboseError<&'a [u8]>>> for Error<'a> {
    fn from(value: nom::Err<VerboseError<&'a [u8]>>) -> Self {
        Error::ParserError(value)
    }
}