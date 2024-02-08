use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use bitbybit::bitfield;
use bytes::Buf;
use bytes::Bytes;
use tracing::info;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[bitfield(u32)]
pub struct WldTextureBitmapInfoFlags {
    #[bit(3, r)]
    pub animated: bool, // 0x08
    #[bit(5, r)]
    pub skip_frames: bool, // 0x20
}

impl Debug for WldTextureBitmapInfoFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let skip_frames = self.skip_frames() && self.animated();
        f.debug_struct("WldTextureListFlags")
            .field("animated", &self.animated())
            .field("skip_frames", &skip_frames)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct WldTextureBitmapInfo {
    pub name: Option<String>,
    pub flags: WldTextureBitmapInfoFlags,
    pub sleep: u32,
    pub frame_count: u32,
    pub texture_current: u32,
    pub texture_list: Vec<u32>,
}

impl WldFragment for WldTextureBitmapInfo {
    const TYPE: u32 = 4;
}

impl Decoder<Settings> for WldTextureBitmapInfo {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let flags = WldTextureBitmapInfoFlags::new_with_raw_value(input.get_u32_le());

        let frame_count = input.get_u32_le();
        let texture_current = if flags.animated() && flags.skip_frames() {
            input.get_u32_le()
        } else {
            0
        };
        let sleep = if flags.animated() {
            input.get_u32_le()
        } else {
            0
        };
        let mut texture_list = Vec::new();
        for _ in 0..frame_count {
            texture_list.push(input.get_u32_le());
        }

        Ok(Self {
            name,
            flags,
            frame_count,
            sleep,
            texture_current,
            texture_list,
        })
    }
}
