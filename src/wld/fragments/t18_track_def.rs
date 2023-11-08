use std::sync::Arc;

use bytes::Bytes;

use crate::{Decoder, Settings};

#[derive(Clone, Debug)]
pub struct WldTrackDef {
    pub name: Option<String>,
    pub remainder: Bytes,
}

impl Decoder<Settings> for WldTrackDef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        Ok(Self {
            name,
            remainder: input.clone(),
        })
    }
}
