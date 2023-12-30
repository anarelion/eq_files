mod pfs;
mod utils;
mod wld;

use std::string::FromUtf8Error;
use std::sync::Arc;

use bytes::Bytes;
use lazy_static::lazy_static;
use thiserror::Error;

pub use crate::pfs::PackFile;
pub use crate::wld::fragments::*;
pub use crate::wld::WldFile;

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

#[derive(Default)]
pub struct EmptySettings;

lazy_static! {
    pub static ref EMPTY_SETTINGS: Arc<EmptySettings> = Arc::new(EmptySettings);
}

pub trait Decoder<S> {
    fn new(input: &mut Bytes, settings: Arc<S>) -> Result<Self, EQFilesError>
    where
        Self: Sized;
}
