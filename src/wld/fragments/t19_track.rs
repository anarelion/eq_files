use std::rc::Rc;
use std::fmt::{Debug, Formatter};
use bitbybit::bitfield;
use bytes::{Bytes, Buf};

use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldTrack {
    pub name: Option<String>,
    pub reference: u32,
    pub flags: WldTrackFlags,
    pub sleep: Option<u32>,
}


#[bitfield(u32)]
pub struct WldTrackFlags {
    #[bit(0, r)]
    pub has_sleep: bool, // 0x01
    #[bit(1, r)]
    pub reverse: bool, // 0x02
    #[bit(2, r)]
    pub interpolate: bool, // 0x04
}

impl Decoder for WldTrack {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let reference = input.get_u32_le();
        let flags = WldTrackFlags::new_with_raw_value(input.get_u32_le());
        let sleep = if flags.has_sleep() {
            Some(input.get_u32_le())
        } else {
            None
        };

        Ok(Self {
            name,
            reference,
            flags,
            sleep,
        })
    }
}


impl Debug for WldTrackFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WldSkeletonFlags")
            .field("has_sleep", &self.has_sleep())
            .field("reverse", &self.reverse())
            .field("interpolate", &self.interpolate())
            .finish()
    }
}