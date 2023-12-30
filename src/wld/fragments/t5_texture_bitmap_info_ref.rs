use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldTextureBitmapInfoRef {
    pub name: Option<String>,
    pub flags: u32,
    pub texture_ref: u16,
}

impl WldFragment for WldTextureBitmapInfoRef {
    const TYPE: u32 = 5;
}

impl Decoder<Settings> for WldTextureBitmapInfoRef {
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
