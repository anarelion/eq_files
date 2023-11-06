use std::sync::Arc;

use bytes::{Buf, Bytes};
use tracing::info;

use crate::Decoder;
use crate::Settings;

#[derive(Clone, Debug)]
pub struct WldSkeletonRef {
    pub name_ref: i32,
    pub name: Option<String>,
    pub reference: u32,
    pub params1: u32,
}

impl Decoder<Settings> for WldSkeletonRef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let reference = input.get_u32_le();
        let params1 = input.get_u32_le();

        info!("Remaining t17: {:?}", input);

        Ok(Self {
            name_ref: settings.get_name_ref(),
            name: settings.get_name(),
            reference,
            params1,
        })
    }
}
