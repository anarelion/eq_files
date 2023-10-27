use bytes::{Buf, Bytes};

use crate::utils::take;
use crate::{Decoder, EQFilesError};

#[derive(Clone, Debug)]
pub struct WldRawFragment(pub u32, pub Bytes);

impl Decoder for WldRawFragment {
    type Settings = ();

    fn new(input: &mut Bytes, _: Self::Settings) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let fragment_size = input.get_u32_le();
        let fragment_type = input.get_u32_le();
        Ok(WldRawFragment(
            fragment_type,
            take(input, fragment_size as usize),
        ))
    }
}
