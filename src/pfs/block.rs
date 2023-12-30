use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::EmptySettings;

#[derive(Clone, PartialEq)]
pub struct PackFileBlock {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub data: Bytes,
}

impl Decoder<EmptySettings> for PackFileBlock {
    fn new(input: &mut Bytes, _: Arc<EmptySettings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let compressed_size = input.get_u32_le();
        let uncompressed_size = input.get_u32_le();
        let data = crate::utils::take(input, compressed_size as usize);
        Ok(PackFileBlock {
            compressed_size,
            uncompressed_size,
            data,
        })
    }
}
