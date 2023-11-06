use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::utils::decode_string;
use crate::{Decoder, Settings};

#[derive(Clone, Debug)]
pub struct WldTextureFilename {
    pub name: Option<String>,
    pub textures: Vec<String>,
}

impl Decoder<Settings> for WldTextureFilename {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let textures = (0..(input.get_i32_le() + 1))
            .map(|_| {
                let name_length = input.get_u16_le();
                decode_string(input, name_length as usize).unwrap()
            })
            .collect();

        Ok(Self { name, textures })
    }
}
