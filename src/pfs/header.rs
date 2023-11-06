use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::EmptySettings;

#[derive(Clone)]
pub struct PackFileHeader {
    pub(crate) directory_offset: u32,
    pub magic_number: u32,
    pub version: u32,
}

impl crate::Decoder<EmptySettings> for PackFileHeader {
    fn new(input: &mut Bytes, _: Arc<EmptySettings> ) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let directory_offset = input.get_u32_le();
        let magic_number = input.get_u32_le();
        let version = input.get_u32_le();
        Ok(PackFileHeader {
            directory_offset,
            magic_number,
            version,
        })
    }
}
