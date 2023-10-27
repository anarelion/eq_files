use bytes::{Buf, Bytes};

#[derive(Clone)]
pub struct PackFileHeader {
    pub(crate) directory_offset: u32,
    pub magic_number: u32,
    pub version: u32,
}

impl crate::Decoder for PackFileHeader {
    type Settings = ();

    fn new(input: &mut Bytes, _: Self::Settings) -> Result<Self, crate::EQFilesError>
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
