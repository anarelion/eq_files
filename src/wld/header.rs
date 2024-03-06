use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;
use tracing::info;

use crate::Decoder;
use crate::EQFilesError;
use crate::EmptySettings;

#[derive(Clone, Debug, Default)]
pub struct WldHeader {
    pub magic_number: u32,
    pub is_old_world: bool,
    pub fragment_count: u32,
    pub region_count: u32,
    pub max_object_bytes: u32,
    pub string_hash_size: u32,
    pub string_count: u32,
}

impl Decoder<EmptySettings> for WldHeader {
    fn new(input: &mut Bytes, _settings: Arc<EmptySettings>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let magic_number = input.get_u32_le();
        if magic_number != 0x54503D02 {
            return Err(EQFilesError::InvalidMagicNumber(magic_number));
        }

        let is_old_world = match input.get_u32_le() {
            0x00015500 => true,
            0x1000C800 => false,
            version => {
                return Err(EQFilesError::InvalidVersionNumber(version));
            }
        };

        let fragment_count = input.get_u32_le();
        let region_count = input.get_u32_le();
        let max_object_bytes = input.get_u32_le();
        let string_hash_size = input.get_u32_le();
        let string_count = input.get_u32_le();

        let result = WldHeader {
            magic_number,
            is_old_world,
            fragment_count,
            region_count,
            max_object_bytes,
            string_hash_size,
            string_count,
        };

        // info!("{:?}", result);

        Ok(result)
    }
}
