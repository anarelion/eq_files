pub(crate) mod fragments;
mod header;
mod names;
mod raw_fragment;

use std::sync::Arc;

use bytes::Bytes;
use std::collections::BTreeMap;

use crate::{utils::*, EmptySettings};
use crate::{Decoder, EQFilesError};
use fragments::*;
use header::WldHeader;
use names::WldNames;
use raw_fragment::WldRawFragment;

#[derive(Debug)]
pub struct WldFile {
    pub header: Arc<WldHeader>,
    pub names: Arc<WldNames>,
    pub fragments_by_index: BTreeMap<u32, Arc<WldRawFragment>>,
    pub fragments_by_name: BTreeMap<String, Arc<WldRawFragment>>,
    base_settings: BaseSettings,
}

impl Decoder<EmptySettings> for WldFile {
    fn new(input: &mut Bytes, settings: Arc<EmptySettings>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let header = Arc::new(WldHeader::new(input, settings.clone())?);
        let names = Arc::new(WldNames::new(input, Arc::new(header.hash_size))?);

        let fragments_by_index: BTreeMap<u32, Arc<WldRawFragment>> = count(
            input,
            header.fragment_count as usize,
            names.clone(),
            WldRawFragment::new,
        )?
        .iter()
        .enumerate()
        .map(|(i, v)| ((i + 1) as u32, Arc::new(v.clone())))
        .collect();

        let fragments_by_name = fragments_by_index
            .iter()
            .filter_map(|(_, v)| v.clone().name.clone().map(|n| (n, v.clone())))
            .collect();

        Ok(WldFile {
            header: header.clone(),
            names: names.clone(),
            fragments_by_index,
            fragments_by_name,
            base_settings: BaseSettings::new(header, names),
        })
    }
}

impl WldFile {
    pub fn fragment_by_index<T>(&self, index: u32) -> Option<T>
    where
        T: WldFragment,
    {
        let fragment = self.fragments_by_index.get(&index)?.clone();
        assert_eq!(fragment.fragment_type, T::TYPE);
        let mut raw = fragment.contents.clone();

        T::new(&mut raw, self.base_settings.make_settings(fragment)).ok()
    }
    
    pub fn models(&self) -> Vec<WldModel> {
        self.fragments_by_index
            .clone()
            .into_iter()
            .filter_map(|(t, f)| {
                let f = f.clone();
                if t == 20 {
                    let mut input = f.contents.clone();
                    WldModel::new(&mut input, self.base_settings.make_settings(f)).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    // pub fn materials(&self) -> Vec<WldMaterial> {
    //     self.t48.clone().into_values().collect()
    // }

    // pub fn meshes(&self) -> Vec<WldMesh> {
    //     self.t54.clone().into_values().collect()
    // }

    // pub fn get_texture_filename(&self, index: u32) -> WldTextureFilename {
    //     self.t3.get(&index).unwrap().clone()
    // }

    // pub fn get_texture_list(&self, index: u32) -> WldTextureList {
    //     self.t4.get(&index).unwrap().clone()
    // }

    // pub fn get_texture_list_ref(&self, index: u32) -> WldTextureRef {
    //     self.t5.get(&index).unwrap().clone()
    // }

    // pub fn get_skeleton(&self, index: u32) -> WldSkeleton {
    //     self.t16.get(&index).unwrap().clone()
    // }

    // pub fn get_skeleton_ref(&self, index: u32) -> WldSkeletonRef {
    //     self.t17.get(&index).unwrap().clone()
    // }

    // pub fn get_track(&self, index: u32) -> WldTrack {
    //     self.t19.get(&index).unwrap().clone()
    // }

    // pub fn get_dm_sprite_ref(&self, index: u32) -> WldDmSpriteRef {
    //     self.t45.get(&index).unwrap().clone()
    // }

    // pub fn get_mesh(&self, index: u32) -> WldMesh {
    //     self.t54.get(&index).unwrap().clone()
    // }

    // pub fn get_fragment_type(&self, index: u32) -> u32 {
    //     self.raw_fragments.get(&index).unwrap().0
    // }
}
