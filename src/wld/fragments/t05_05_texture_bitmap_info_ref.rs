use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use bitbybit::bitfield;
use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[bitfield(u32)]
pub struct WldTextureBitmapInfoRefFlags {
    #[bit(20, r)]
    pub unknown1: bool, // 0x100000
    #[bit(22, r)]
    pub unknown2: bool, // 0x400000
    #[bits(0..=31, r)]
    pub all: u32,
}

impl Debug for WldTextureBitmapInfoRefFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut x = f.debug_struct("WldTextureListFlags");
        x.field("unknown1", &self.unknown1());
        x.field("unknown2", &self.unknown2());
        if self.all() != 0 {
            x.field("all", &format!("{:#x}", self.all()));
        }
        x.finish()
    }
}

#[derive(Clone, Debug)]
pub struct WldTextureBitmapInfoRef {
    pub name: Option<String>,
    pub flags: WldTextureBitmapInfoRefFlags,
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
        let flags = WldTextureBitmapInfoRefFlags::new_with_raw_value(input.get_u32_le());

        Ok(Self {
            name,
            flags,
            texture_ref,
        })
    }
}
