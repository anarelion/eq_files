mod t03_03_texture_bitmap_name;
mod t04_04_texture_bitmap_info;
mod t05_05_texture_bitmap_info_ref;
mod t16_10_skeleton_track_set;
mod t17_11_skeleton_track_set_ref;
mod t18_12_skeleton_piece_track_def;
mod t19_13_skeleton_piece_track;
mod t20_14_model;
mod t38_26_particle_sprite;
mod t45_2d_mesh_ref;
mod t48_30_material;
mod t49_31_material_list;
mod t52_34_particle_cloud;
mod t54_36_mesh;

use std::sync::Arc;

pub use t03_03_texture_bitmap_name::WldTextureBitmapName;
pub use t04_04_texture_bitmap_info::WldTextureBitmapInfo;
pub use t05_05_texture_bitmap_info_ref::WldTextureBitmapInfoRef;
pub use t16_10_skeleton_track_set::WldSkeletonTrackSet;
pub use t17_11_skeleton_track_set_ref::WldSkeletonTrackSetRef;
pub use t18_12_skeleton_piece_track_def::WldSkeletonPieceTrackDef;
pub use t19_13_skeleton_piece_track::WldSkeletonPieceTrack;
pub use t20_14_model::WldModel;
pub use t38_26_particle_sprite::WldParticleSprite;
pub use t45_2d_mesh_ref::WldMeshRef;
pub use t48_30_material::WldMaterial;
pub use t49_31_material_list::WldMaterialList;
pub use t52_34_particle_cloud::WldParticleCloud;
pub use t54_36_mesh::WldMesh;

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
