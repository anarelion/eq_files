use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::{Decoder, Settings, WldFragment};

#[derive(Clone, Debug)]
pub struct WldTrackDef {
    pub name_ref: i32,
    pub name: Option<String>,
    pub flags: u32,
    pub remainder: Bytes,
}

impl WldFragment for WldTrackDef {
    const TYPE: u32 = 18;
}

impl Decoder<Settings> for WldTrackDef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name_ref = settings.get_name_ref();
        let name = settings.get_name();
        let flags = input.get_u32_le();

        Ok(Self {
            name_ref,
            name,
            flags,
            remainder: input.clone(),
        })
    }
}
