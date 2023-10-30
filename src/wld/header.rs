use bytes::{Buf, Bytes};

use crate::{Decoder, EQFilesError};

#[derive(Debug, Default)]
pub struct WldHeader {
    pub magic_number: u32,
    pub is_old_world: bool,
    pub fragment_count: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub hash_size: u32,
    pub unk3: u32,
}

impl Decoder for WldHeader {
    type Settings = ();

    fn new(input: &mut Bytes, _: Self::Settings) -> Result<Self, EQFilesError>
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
        let unk1 = input.get_u32_le();
        let unk2 = input.get_u32_le();
        let hash_size = input.get_u32_le();
        let unk3 = input.get_u32_le();

        Ok(WldHeader {
            magic_number,
            is_old_world,
            fragment_count,
            unk1,
            unk2,
            hash_size,
            unk3,
        })
    }
}
