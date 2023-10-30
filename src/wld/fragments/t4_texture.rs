use std::rc::Rc;

use bytes::{Buf, Bytes};

use crate::wld::names::WldNames;
use crate::Decoder;
use tracing::info;

#[derive(Clone, Debug)]
pub struct WldTextureList {
    pub name: Option<String>,
    pub flags: u32,
    pub sleep: u32,
    pub texture_current: u32,
    pub texture_list: Vec<u32>,
}

impl Decoder for WldTextureList {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let flags = input.get_u32_le();

        let texture_count = input.get_i32_le();
        let texture_current = match flags & 0x20 {
            0x20 => input.get_u32_le(),
            _ => 0,
        };
        let sleep = if (flags & 0x10 != 0) && (flags & 0x08 != 0) {
            input.get_u32_le()
        } else {
            0
        };
        let mut texture_list = Vec::new();
        for _ in 0..texture_count {
            texture_list.push(input.get_u32_le());
        }

        if texture_count != 1 {
            info!("weird texture : {:?}", name);
        }

        Ok(Self {
            name,
            flags,
            sleep,
            texture_current,
            texture_list,
        })
    }
}
