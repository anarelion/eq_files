use std::rc::Rc;

use bytes::{Bytes, Buf};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldSkeletonRef {
    pub name: Option<String>,
    pub reference : u32,
    pub params1: u32,
}

impl Decoder for WldSkeletonRef {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let reference = input.get_u32_le();
        let params1 = input.get_u32_le();

        Ok(Self {
            name,
            reference,
            params1,
        })
    }
}
