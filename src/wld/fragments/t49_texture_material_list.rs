use std::rc::Rc;

use bytes::{Buf, Bytes};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTextureMaterialList {
    pub name: Option<String>,
    pub flags: u32,
    pub material_refs: Vec<u32>,
}

impl Decoder for WldTextureMaterialList {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
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
