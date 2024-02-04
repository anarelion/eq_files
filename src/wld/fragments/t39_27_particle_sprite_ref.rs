use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldParticleSpriteRef {
    pub name: Option<String>,
    reference: u32,
    unknown: u32,
}

impl WldFragment for WldParticleSpriteRef {
    const TYPE: u32 = 39;
}

impl Decoder<Settings> for WldParticleSpriteRef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let reference = input.get_u32_le();
        let unknown = input.get_u32_le();

        Ok(Self {
            name,
            reference,
            unknown,
        })
    }
}
