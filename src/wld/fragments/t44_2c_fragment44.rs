use std::sync::Arc;

use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldFragment44 {
    pub name: Option<String>,
}

impl WldFragment for WldFragment44 {
    const TYPE: u32 = 44;
}

impl Decoder<Settings> for WldFragment44 {
    fn new(_input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        Ok(Self { name })
    }
}
