use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::{Decoder, Settings};

#[derive(Clone, Debug)]
pub struct WldMaterialList {
    pub name: Option<String>,
    pub flags: u32,
    pub material_refs: Vec<u32>,
}

impl Decoder<Settings> for WldMaterialList {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let flags = input.get_u32_le();
        let count = input.get_u32_le();
        let mut refs = Vec::new();
        for _ in 0..count {
            refs.push(input.get_u32_le());
        }

        Ok(Self {
            name,
            flags,
            material_refs: refs,
        })
    }
}
