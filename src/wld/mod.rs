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
use tracing::info;

type FragmentIndex = u32;

#[derive(Debug)]
pub struct WldFile {
    pub header: Arc<WldHeader>,
    pub names: Arc<WldNames>,
    pub fragments_by_index: BTreeMap<FragmentIndex, Arc<WldRawFragment>>,
    pub fragments_by_name: BTreeMap<String, Arc<WldRawFragment>>,
    base_settings: BaseSettings,
}

impl Decoder<EmptySettings> for WldFile {
    fn new(input: &mut Bytes, settings: Arc<EmptySettings>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let header = Arc::new(WldHeader::new(input, settings.clone())?);
        let names = Arc::new(WldNames::new(input, Arc::new(header.string_hash_size))?);

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

        info!("fragments by index: {}", fragments_by_index.len());

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
    pub fn fragment_by_index<T>(&self, index: FragmentIndex) -> Option<T>
    where
        T: WldFragment,
    {
        let fragment = self.fragments_by_index.get(&index)?.clone();
        assert_eq!(fragment.fragment_type, T::TYPE);
        let mut raw = fragment.contents.clone();

        T::new(&mut raw, self.base_settings.make_settings(fragment)).ok()
    }

    pub fn fragment_by_name<T>(&self, name: String) -> Option<T>
    where
        T: WldFragment,
    {
        self
            .fragments_by_name
            .iter()
            .filter(|(fragment_name, fragment)| {
                **fragment_name == name && fragment.fragment_type == T::TYPE
            })
            .filter_map(|(_, fragment)| {
                let mut cont = fragment.contents.clone();
                T::new(
                    &mut cont,
                    self.base_settings.make_settings(fragment.clone()),
                )
                .ok()
            })
            .collect::<Vec<T>>()
            .into_iter()
            .next()
    }

    pub fn fragments_containing_name<T>(&self, contents: String) -> Vec<T>
    where
        T: WldFragment,
    {
        self.fragments_by_name
            .iter()
            .filter(|(name, fragment)| {
                name.contains(&contents) && fragment.fragment_type == T::TYPE
            })
            .filter_map(|(_, fragment)| {
                let mut cont = fragment.contents.clone();
                T::new(
                    &mut cont,
                    self.base_settings.make_settings(fragment.clone()),
                )
                .ok()
            })
            .collect()
    }

    pub fn fragments_by_type<T>(&self, typ: u32) -> Vec<T>
    where
        T: WldFragment,
    {
        self.fragments_by_name
            .iter()
            .filter(|(_, fragment)| {
                fragment.fragment_type == typ && fragment.fragment_type == T::TYPE
            })
            .filter_map(|(_, fragment)| {
                let mut cont = fragment.contents.clone();
                T::new(
                    &mut cont,
                    self.base_settings.make_settings(fragment.clone()),
                )
                .ok()
            })
            .collect()
    }

    pub fn type_by_index(&self, index: u32) -> u32 {
        let fragment = self.fragments_by_index.get(&index).unwrap().clone();
        fragment.fragment_type
    }

    pub fn materials(&self) -> Vec<WldTexture> {
        self.fragments_by_type(WldTexture::TYPE)
    }

    pub fn models(&self) -> Vec<WldModel> {
        self.fragments_by_type(WldModel::TYPE)
    }
}
