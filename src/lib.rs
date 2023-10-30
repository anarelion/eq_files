mod pfs;
mod utils;
mod wld;

use std::string::FromUtf8Error;

pub use crate::pfs::PackFile;
pub use crate::wld::fragments::*;
pub use crate::wld::WldFile;
use bytes::Bytes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EQFilesError {
    #[error("unknown error")]
    UnknownError,
    #[error("invalid magic number {0:x}")]
    InvalidMagicNumber(u32),
    #[error("Invalid WLD version: {0:x}")]
    InvalidVersionNumber(u32),
    #[error("error decoding string")]
    ErrorDecodingString(#[from] FromUtf8Error),
}

pub trait Decoder {
    type Settings;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, EQFilesError>
    where
        Self: Sized;
}
