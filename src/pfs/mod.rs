mod block;
mod entry;
mod header;

use std::path::PathBuf;
use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::{utils::*, EMPTY_SETTINGS, EmptySettings};
use crate::{Decoder, EQFilesError};

#[derive(Clone)]
pub struct PackFile {
    pub path: PathBuf,
    pub header: header::PackFileHeader,
    pub entry_count: usize,
    pub entries: Vec<entry::PackFileEntry>,
    // footer: PackFileFooter,
}

impl Decoder<PathBuf> for PackFile {
    fn new(input: &mut Bytes, settings: Arc<PathBuf>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let header = header::PackFileHeader::new(input, EMPTY_SETTINGS.clone())?;
        let block_contents = take(input, header.directory_offset as usize - 12);
        let entry_count = input.get_u32_le() as usize;
        let mut entries = count(
            input,
            entry_count,
            EMPTY_SETTINGS.clone(),
            entry::PackFileEntry::new,
        )?;
        // let footer = footer(input)?;

        entries.sort_by_key(|a| a.pointer);

        let entries = entries
            .into_iter()
            .map(|mut e| {
                let mut offset = (e.pointer - 12) as usize;
                let mut bytes_remaining = e.uncompressed_size;
                let mut blocks = Vec::new();

                while bytes_remaining > 0 {
                    let mut temp_block = block_contents.clone();
                    temp_block.advance(offset);
                    let block =
                        block::PackFileBlock::new(&mut temp_block, EMPTY_SETTINGS.clone()).unwrap();
                    offset += (block.compressed_size + 8) as usize;
                    bytes_remaining -= block.uncompressed_size;
                    blocks.push(block);
                }
                e.blocks = Some(blocks);
                e
            })
            .collect();
        Ok(PackFile {
            path: settings.clone().to_path_buf(),
            header,
            entry_count,
            entries,
            // footer: PackFileFooter {
            //     footer_string: Vec::new(),
            //     timestamp: 0,
            // },
        })
    }
}

fn directory_string(input: &mut Bytes, _: Arc<EmptySettings>) -> Result<String, EQFilesError> {
    let length = input.get_u32_le();
    match string(input, length as usize) {
        Ok(s) => Ok(s),
        Err(e) => Err(EQFilesError::ErrorDecodingString(e)),
    }
}

pub fn directory(input: &mut Bytes) -> Result<Vec<String>, EQFilesError> {
    let file_count = input.get_u32_le();
    Ok(count(input, file_count as usize, EMPTY_SETTINGS.clone(), directory_string)?)
}

impl PackFile {
    pub fn filenames(&self) -> Vec<String> {
        let directory_entry = self.entries.last().expect("Directory block does not exist");
        let mut uncompressed_blocks = directory_entry.decompress();
        let filenames =
            directory(&mut uncompressed_blocks).expect("Failed to parse directory block");
        filenames
    }

    pub fn get(&self, filename: &str) -> Option<Bytes> {
        self.filenames()
            .iter()
            .position(|f| f.eq_ignore_ascii_case(filename))
            .and_then(|position| self.entries.get(position).map(|entry| entry.decompress()))
    }

    pub fn files(self) -> impl Iterator<Item = (String, Bytes)> {
        self.filenames()
            .into_iter()
            .zip(self.entries.into_iter().map(|entry| entry.decompress()))
    }
}

// pub struct PackFileFooter {
//     footer_string: Vec<u8>,
//     timestamp: u32,
// }

// fn footer(input: &mut Bytes) -> Result<PackFileFooter, PackFileError> {
//     let footer_string = take(input, 5);
//     let timestamp = input.get_u32_le();
//     Ok(PackFileFooter {
//         footer_string: Vec::from(footer_string),
//         timestamp,
//     })
// }
