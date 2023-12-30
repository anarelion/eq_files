use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldSkeletonTrackSetRef {
    pub name_ref: i32,
    pub name: Option<String>,
    pub reference: u32,
    pub params1: u32,
}

impl WldFragment for WldSkeletonTrackSetRef {
    const TYPE: u32 = 17;
}

impl Decoder<Settings> for WldSkeletonTrackSetRef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let reference = input.get_u32_le();
        let params1 = input.get_u32_le();

        Ok(Self {
            name_ref: settings.get_name_ref(),
            name: settings.get_name(),
            reference,
            params1,
        })
    }
}
