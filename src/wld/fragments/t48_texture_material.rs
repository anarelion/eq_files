use std::rc::Rc;

use bytes::{Buf, Bytes};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTextureMaterial {
    pub name: Option<String>,
    pub flags: u32,
    pub render_method: u32,
    pub rgb_pen: u32,
    pub brightness: f32,
    pub scaled_ambient: f32,
    pub bitmap_ref: u32,
    pub pairs: Option<(u32, u32)>,
}

impl Decoder for WldTextureMaterial {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);

        let flags = input.get_u32_le();
        let render_method = input.get_u32_le();
        let rgb_pen = input.get_u32_le();
        let brightness = input.get_f32_le();
        let scaled_ambient = input.get_f32_le();
        let texture_ref = input.get_u32_le();

        Ok(Self {
            name,
            flags,
            render_method,
            rgb_pen,
            brightness,
            scaled_ambient,
            bitmap_ref: texture_ref,
            pairs: if flags & 0x01 != 0 {
                Some((input.get_u32_le(), input.get_u32_le()))
            } else {
                None
            },
        })
    }
}
