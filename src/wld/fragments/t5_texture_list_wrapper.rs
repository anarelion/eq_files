use std::rc::Rc;

use bytes::{Buf, Bytes};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTextureRef {
    pub name: Option<String>,
    pub flags: u32,
    pub texture_ref: u16,
}

impl Decoder for WldTextureRef {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let texture_ref = input.get_u16_le();
        let flags = input.get_u32_le();

        Ok(Self {
            name,
            flags,
            texture_ref,
        })
    }
}
