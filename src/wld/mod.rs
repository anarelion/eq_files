pub(crate) mod fragments;
mod header;
mod names;
mod raw_fragment;

use std::rc::Rc;

use bytes::Bytes;
use std::collections::{HashMap, HashSet};
use tracing::info;

use self::header::WldHeader;
use self::names::WldNames;
use self::raw_fragment::WldRawFragment;
use crate::utils::*;
use crate::{Decoder, EQFilesError};

#[derive(Debug)]
pub struct WldFile {
    pub header: WldHeader,
    pub names: WldNames,
    // raw_fragments: HashMap<u32, WldRawFragment>,
    pub t3: HashMap<u32, fragments::WldTextureBitmapFilename>,
    pub t4: HashMap<u32, fragments::WldTextureBitmap>,
    pub t5: HashMap<u32, fragments::WldTextureBitmapRef>,
    pub t18: HashMap<u32, fragments::WldTrackDef>,
    pub t19: HashMap<u32, fragments::WldTrack>,
    pub t48: HashMap<u32, fragments::WldTextureMaterial>,
    pub t49: HashMap<u32, fragments::WldTextureMaterialList>,
    pub t54: HashMap<u32, fragments::WldMesh>,
}

impl Decoder for WldFile {
    type Settings = ();

    fn new(input: &mut Bytes, _settings: Self::Settings) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let header = WldHeader::new(input, ())?;
        let names = WldNames::new(input, header.hash_size)?;
        let raw_fragments: HashMap<u32, WldRawFragment> = count(
            input,
            header.fragment_count as usize,
            (),
            WldRawFragment::new,
        )?
        .iter()
        .enumerate()
        .map(|(i, v)| ((i + 1) as u32, v.clone()))
        .collect();

        let names = Rc::new(names);

        let t3 = extract_fragments(&raw_fragments, 3, names.clone());
        let t4 = extract_fragments(&raw_fragments, 4, names.clone());
        let t5 = extract_fragments(&raw_fragments, 5, names.clone());
        let t18 = extract_fragments(&raw_fragments, 18, names.clone());
        let t19 = extract_fragments(&raw_fragments, 19, names.clone());
        let t48 = extract_fragments(&raw_fragments, 48, names.clone());
        let t49 = extract_fragments(&raw_fragments, 49, names.clone());
        let t54 = extract_fragments(&raw_fragments, 54, (header.is_old_world, names.clone()));
        let remaining: HashSet<u32> = raw_fragments
            .iter()
            .map(|(_, frag)| frag.0)
            .filter(|a| ![3, 4, 5, 18, 19, 48, 49, 54].contains(a))
            .collect();
        if !remaining.is_empty() {
            info!("{:?}", remaining);
        }
        Ok(WldFile {
            header,
            names: Rc::into_inner(names).unwrap(),
            // raw_fragments,
            t3,
            t4,
            t5,
            t18,
            t19,
            t48,
            t49,
            t54,
        })
    }
}

fn extract_fragments<S, T: Decoder<Settings = S>>(
    raw_fragments: &HashMap<u32, WldRawFragment>,
    code: u32,
    settings: S,
) -> HashMap<u32, T>
where
    S: Clone,
{
    raw_fragments
        .iter()
        .filter(|f| f.1 .0 == code)
        .map(|(k, v)| (*k, T::new(&mut v.1.clone(), settings.clone()).unwrap()))
        .collect()
}
