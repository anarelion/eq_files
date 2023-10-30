use std::rc::Rc;

use bytes::{Bytes, Buf};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldDmSpriteRef {
    pub name: Option<String>,
    pub reference: u32,
    pub params: u32,
}

impl Decoder for WldDmSpriteRef {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);

        Ok(Self {
            name,
            reference: input.get_u32_le(),
            params: input.get_u32_le(),
        })
    }
}
