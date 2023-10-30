pub(crate) mod fragments;
mod header;
mod names;
mod raw_fragment;

use std::rc::Rc;

use bytes::Bytes;
use std::collections::{HashMap, HashSet};
use tracing::info;

use crate::utils::*;
use crate::{Decoder, EQFilesError};
use fragments::*;
use header::WldHeader;
use names::WldNames;
use raw_fragment::WldRawFragment;

#[derive(Debug, Default)]
pub struct WldFile {
    pub header: WldHeader,
    pub names: WldNames,
    pub raw_fragments: HashMap<u32, WldRawFragment>,
    pub t3: HashMap<u32, WldTextureFilename>,
    pub t4: HashMap<u32, WldTextureList>,
    pub t5: HashMap<u32, WldTextureRef>,
    pub t18: HashMap<u32, WldTrackDef>,
    pub t19: HashMap<u32, WldTrack>,
    pub t20: HashMap<u32, WldModel>,
    pub t45: HashMap<u32, WldDmSpriteRef>,
    pub t48: HashMap<u32, WldMaterial>,
    pub t49: HashMap<u32, WldMaterialList>,
    pub t54: HashMap<u32, WldMesh>,
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
        let t20 = extract_fragments(&raw_fragments, 20, names.clone());
        let t45 = extract_fragments(&raw_fragments, 45, names.clone());
        let t48 = extract_fragments(&raw_fragments, 48, names.clone());
        let t49 = extract_fragments(&raw_fragments, 49, names.clone());
        let t54 = extract_fragments(&raw_fragments, 54, (header.is_old_world, names.clone()));
        let remaining: HashSet<u32> = raw_fragments
            .iter()
            .map(|(_, frag)| frag.0)
            .filter(|a| ![3, 4, 5, 18, 19, 20, 45, 48, 49, 54].contains(a))
            .collect();
        if !remaining.is_empty() {
            info!("{:?}", remaining);
        }
        Ok(WldFile {
            header,
            names: Rc::into_inner(names).unwrap(),
            raw_fragments,
            t3,
            t4,
            t5,
            t18,
            t19,
            t20,
            t45,
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

impl WldFile {
    pub fn materials(&self) -> Vec<WldMaterial> {
        self.t48.clone().into_values().collect()
    }

    pub fn meshes(&self) -> Vec<WldMesh> {
        self.t54.clone().into_values().collect()
    }

    pub fn models(&self) -> Vec<WldModel> {
        self.t20.clone().into_values().collect()
    }

    pub fn get_texture_filename(&self, index: u32) -> WldTextureFilename {
        self.t3.get(&index).unwrap().clone()
    }

    pub fn get_texture_list(&self, index: u32) -> WldTextureList {
        self.t4.get(&index).unwrap().clone()
    }

    pub fn get_texture_list_ref(&self, index: u32) -> WldTextureRef {
        self.t5.get(&index).unwrap().clone()
    }
}
