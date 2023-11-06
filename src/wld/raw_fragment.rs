use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::utils::take;
use crate::{Decoder, EQFilesError};

use super::names::WldNames;

#[derive(Clone, Debug)]
pub struct WldRawFragment {
    pub fragment_size: u32,
    pub fragment_type: u32,
    pub name_ref: i32,
    pub name: Option<String>,
    pub contents: Bytes,
}

impl Decoder<WldNames> for WldRawFragment {
    fn new(input: &mut Bytes, names: Arc<WldNames>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let fragment_size = input.get_u32_le();
        let fragment_type = input.get_u32_le();
        let name_ref = input.get_i32_le();
        let name = names.get_name(name_ref);
        Ok(WldRawFragment {
            fragment_size,
            fragment_type,
            name_ref,
            name,
            contents: take(input, (fragment_size - 4) as usize),
        })
    }
}
