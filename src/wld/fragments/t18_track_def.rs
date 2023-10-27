use std::rc::Rc;

use bytes::Bytes;

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTrackDef {
    pub name: Option<String>,
    pub remainder: Bytes,
}

impl Decoder for WldTrackDef {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);

        Ok(Self {
            name,
            remainder: input.clone(),
        })
    }
}
