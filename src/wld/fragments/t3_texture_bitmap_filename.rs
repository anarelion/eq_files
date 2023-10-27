use std::rc::Rc;

use bytes::{Buf, Bytes};

use crate::wld::names::WldNames;
use crate::utils::decode_string;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTextureBitmapFilename {
    pub name: Option<String>,
    pub textures: Vec<String>,
}

impl Decoder for WldTextureBitmapFilename {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let textures = (0..(input.get_i32_le() + 1))
            .map(|_| {
                let name_length = input.get_u16_le();
                decode_string(input, name_length as usize).unwrap()
            })
            .collect();

        Ok(Self { name, textures })
    }
}
