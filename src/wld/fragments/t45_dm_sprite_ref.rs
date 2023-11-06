use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::{Decoder, Settings};

#[derive(Clone, Debug)]
pub struct WldDmSpriteRef {
    pub name: Option<String>,
    pub reference: u32,
    pub params: u32,
}

impl Decoder<Settings> for WldDmSpriteRef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        Ok(Self {
            name,
            reference: input.get_u32_le(),
            params: input.get_u32_le(),
        })
    }
}
