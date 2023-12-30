use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::utils::decode_string;
use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldTextureBitmapName {
    pub name: Option<String>,
    pub textures: Vec<String>,
}

impl WldFragment for WldTextureBitmapName {
    const TYPE: u32 = 3;
}

impl Decoder<Settings> for WldTextureBitmapName {
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
