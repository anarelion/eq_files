use bitbybit::bitfield;
use bytes::{Buf, Bytes};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::{Decoder, Settings, WldFragment};

#[derive(Clone, Debug)]
pub struct WldSkeletonPieceTrack {
    pub name: Option<String>,
    pub reference: u32,
    pub flags: WldSkeletonPieceTrackFlags,
    pub sleep: Option<u32>,
}

#[bitfield(u32)]
pub struct WldSkeletonPieceTrackFlags {
    #[bit(0, r)]
    pub has_sleep: bool, // 0x01
    #[bit(1, r)]
    pub reverse: bool, // 0x02
    #[bit(2, r)]
    pub interpolate: bool, // 0x04
}

impl WldFragment for WldSkeletonPieceTrack {
    const TYPE: u32 = 19;
}

impl Decoder<Settings> for WldSkeletonPieceTrack {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name();
        let reference = input.get_u32_le();
        let flags = WldSkeletonPieceTrackFlags::new_with_raw_value(input.get_u32_le());
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

impl Debug for WldSkeletonPieceTrackFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WldSkeletonPieceTrackFlags")
            .field("has_sleep", &self.has_sleep())
            .field("reverse", &self.reverse())
            .field("interpolate", &self.interpolate())
            .finish()
    }
}
