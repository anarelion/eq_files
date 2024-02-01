use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldParticleSprite {
    pub name: Option<String>,
    flags: u32,
    bitmap_ref: u32,
    unk: u32,
}

impl WldFragment for WldParticleSprite {
    const TYPE: u32 = 38;
}

impl Decoder<Settings> for WldParticleSprite {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let flags = input.get_u32_le();
        let bitmap_ref = input.get_u32_le();
        let unk = input.get_u32_le();

        Ok(Self {
            name,
            flags,
            bitmap_ref,
            unk,
        })
    }
}
