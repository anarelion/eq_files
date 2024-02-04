use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldFragment24 {
    pub name: Option<String>,
}

impl WldFragment for WldFragment24 {
    const TYPE: u32 = 24;
}

impl Decoder<Settings> for WldFragment24 {
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
