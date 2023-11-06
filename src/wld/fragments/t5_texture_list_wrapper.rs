use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::Decoder;
use crate::Settings;

#[derive(Clone, Debug)]
pub struct WldTextureRef {
    pub name: Option<String>,
    pub flags: u32,
    pub texture_ref: u16,
}

impl Decoder<Settings> for WldTextureRef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let texture_ref = input.get_u16_le();
        let flags = input.get_u32_le();

        Ok(Self {
            name,
            flags,
            texture_ref,
        })
    }
}
