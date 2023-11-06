use bitbybit::bitfield;
use bytes::{Buf, Bytes};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::{Decoder, Settings};

#[derive(Clone, Debug, )]
pub struct WldTrack {
    pub name: Option<String>,
    pub reference: u32,
    pub flags: WldTrackFlags,
    pub sleep: Option<u32>,
}

#[bitfield(u32)]
#[derive()]
pub struct WldTrackFlags {
    #[bit(0, r)]
    pub has_sleep: bool, // 0x01
    #[bit(1, r)]
    pub reverse: bool, // 0x02
    #[bit(2, r)]
    pub interpolate: bool, // 0x04
}

impl Decoder<Settings> for WldTrack {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
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
