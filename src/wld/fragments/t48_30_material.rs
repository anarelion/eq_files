use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldMaterial {
    pub name: Option<String>,
    pub flags: u32,
    pub render_method: u32,
    pub rgb_pen: u32,
    pub brightness: f32,
    pub scaled_ambient: f32,
    pub texture_list_ref: u32,
    pub pairs: Option<(u32, u32)>,
}

impl WldFragment for WldMaterial {
    const TYPE: u32 = 48;
}

impl Decoder<Settings> for WldMaterial {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        let flags = input.get_u32_le();
        let render_method = input.get_u32_le() & 0x7FFFFFFF;
        let rgb_pen = input.get_u32_le();
        let brightness = input.get_f32_le();
        let scaled_ambient = input.get_f32_le();
        let texture_list_ref = input.get_u32_le();

        Ok(Self {
            name,
            flags,
            render_method,
            rgb_pen,
            brightness,
            scaled_ambient,
            texture_list_ref,
            pairs: if flags & 0x01 != 0 {
                Some((input.get_u32_le(), input.get_u32_le()))
            } else {
                None
            },
        })
    }
}
