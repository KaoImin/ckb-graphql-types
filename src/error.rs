use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("Parse Bytes error")]
    ParseBytes,

    #[error("Parse Uint error {0}")]
    ParseUint(ParseIntError),

    #[error("Parse from hex error")]
    FromHex(faster_hex::Error),

    #[error("Invalid hex prefix")]
    HexPrefix,
}

impl From<faster_hex::Error> for Error {
    fn from(err: faster_hex::Error) -> Self {
        Error::FromHex(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseUint(err)
    }
}
