mod t16_skeleton_track_set;
mod t17_skeleton_track_set_ref;
mod t18_skeleton_piece_track_def;
mod t19_skeleton_piece_track;
mod t20_model;
mod t3_texture_bitmap_name;
mod t45_mesh_ref;
mod t48_texture;
mod t49_texture_list;
mod t4_texture_bitmap_info;
mod t54_mesh;
mod t5_texture_bitmap_info_ref;

use std::sync::Arc;

pub use t16_skeleton_track_set::WldSkeletonTrackSet;
pub use t17_skeleton_track_set_ref::WldSkeletonTrackSetRef;
pub use t18_skeleton_piece_track_def::WldSkeletonPieceTrackDef;
pub use t19_skeleton_piece_track::WldSkeletonPieceTrack;
pub use t20_model::WldModel;
pub use t3_texture_bitmap_name::WldTextureBitmapName;
pub use t45_mesh_ref::WldMeshRef;
pub use t48_texture::WldTexture;
pub use t49_texture_list::WldMaterialList;
pub use t4_texture_bitmap_info::WldTextureBitmapInfo;
pub use t54_mesh::WldMesh;
pub use t5_texture_bitmap_info_ref::WldTextureBitmapInfoRef;

use super::header::WldHeader;
use super::names::WldNames;
use super::raw_fragment::WldRawFragment;
use crate::Decoder;

#[derive(Debug, Default)]
pub struct BaseSettings {
    header: Arc<WldHeader>,
    names: Arc<WldNames>,
}

impl BaseSettings {
    pub fn new(header: Arc<WldHeader>, names: Arc<WldNames>) -> Self {
        Self { header, names }
    }

    pub fn make_settings(&self, fragment: Arc<WldRawFragment>) -> Arc<Settings> {
        Arc::new(Settings {
            header: self.header.clone(),
            names: self.names.clone(),
            fragment,
        })
    }
}

#[derive(Debug)]
pub struct Settings {
    header: Arc<WldHeader>,
    names: Arc<WldNames>,
    fragment: Arc<WldRawFragment>,
}

impl Settings {
    pub fn get_name_ref(&self) -> i32 {
        self.fragment.name_ref
    }

    pub fn get_name(&self) -> Option<String> {
        self.fragment.name.clone()
    }

    pub fn get_from_name_ref(&self, index: i32) -> Option<String> {
        self.names.get_name(index)
    }

    pub fn is_old_world(&self) -> bool {
        self.header.is_old_world
    }
}

pub trait WldFragment: Decoder<Settings> {
    const TYPE: u32;
}
