use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldFragment23 {
    pub name: Option<String>,
}

impl WldFragment for WldFragment23 {
    const TYPE: u32 = 23;
}

impl Decoder<Settings> for WldFragment23 {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        Ok(Self {
            name,
        })
    }
}
