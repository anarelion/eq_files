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
    pub raw_fragments: BTreeMap<u32, Arc<WldRawFragment>>,
    base_settings: BaseSettings,
}

impl Decoder<EmptySettings> for WldFile {
    fn new(input: &mut Bytes, settings: Arc<EmptySettings>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let header = Arc::new(WldHeader::new(input, settings.clone())?);
        let names = Arc::new(WldNames::new(input, Arc::new(header.hash_size))?);

        let raw_fragments: BTreeMap<u32, Arc<WldRawFragment>> = count(
            input,
            header.fragment_count as usize,
            names.clone(),
            WldRawFragment::new,
        )?
        .iter()
        .enumerate()
        .map(|(i, v)| ((i + 1) as u32, Arc::new(v.clone())))
        .collect();

        Ok(WldFile {
            header: header.clone(),
            names: names.clone(),
            raw_fragments,
            base_settings: BaseSettings::new(header, names),
        })
    }
}

impl WldFile {
    pub fn fragment_by_index<T>(&self, index: u32) -> Option<T>
    where
        T: WldFragment,
    {
        let fragment = self.raw_fragments.get(&index)?.clone();
        assert_eq!(fragment.fragment_type, T::TYPE);
        let mut raw = fragment.contents.clone();

        T::new(
            &mut raw,
            self.base_settings.make_settings(fragment),
        )
        .ok()
    }
    // pub fn models(&self) -> Vec<WldModel> {
    //     self.t20.clone().into_values().collect()
    // }

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
