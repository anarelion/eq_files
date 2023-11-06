use std::io::Read;
use std::sync::Arc;

use crate::pfs::block::PackFileBlock;
use crate::EmptySettings;
use bytes::{Buf, Bytes};
use compress::zlib;

#[derive(Clone)]
pub struct PackFileEntry {
    pub filename_crc: u32,
    pub pointer: u32,
    pub uncompressed_size: u32,
    pub blocks: Option<Vec<PackFileBlock>>,
}

impl crate::Decoder<EmptySettings> for PackFileEntry {
    fn new(input: &mut Bytes, _: Arc<EmptySettings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let filename_crc = input.get_u32_le();
        let pointer = input.get_u32_le();
        let uncompressed_size = input.get_u32_le();
        Ok(PackFileEntry {
            filename_crc,
            pointer,
            uncompressed_size,
            blocks: None,
        })
    }
}

impl PackFileEntry {
    pub fn decompress(&self) -> Bytes {
        self.blocks
            .as_ref()
            .expect("Failed to decompress block")
            .iter()
            .flat_map(|block| {
                let mut buf = Vec::new();
                zlib::Decoder::new(std::io::Cursor::new(block.data.to_vec()))
                    .read_to_end(&mut buf)
                    .expect("Have enough bytes to decompress");
                buf
            })
            .collect()
    }
}
