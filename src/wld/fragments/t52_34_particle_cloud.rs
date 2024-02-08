use std::sync::Arc;

use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldParticleCloud {
    pub name: Option<String>,
}

impl WldFragment for WldParticleCloud {
    const TYPE: u32 = 52;
}

impl Decoder<Settings> for WldParticleCloud {
    fn new(_input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();

        Ok(Self { name })
    }
}
