mod t16_skeleton;
mod t17_skeleton_ref;
mod t18_track_def;
mod t19_track;
mod t20_model;
mod t3_texture_name;
mod t45_dm_sprite_ref;
mod t48_material;
mod t49_material_list;
mod t4_texture;
mod t54_mesh;
mod t5_texture_list_wrapper;

use std::sync::Arc;

pub use t16_skeleton::WldSkeleton;
pub use t17_skeleton_ref::WldSkeletonRef;
pub use t18_track_def::WldTrackDef;
pub use t19_track::WldTrack;
pub use t20_model::WldModel;
pub use t3_texture_name::WldTextureFilename;
pub use t45_dm_sprite_ref::WldDmSpriteRef;
pub use t48_material::WldMaterial;
pub use t49_material_list::WldMaterialList;
pub use t4_texture::WldTextureList;
pub use t54_mesh::WldMesh;
pub use t5_texture_list_wrapper::WldTextureRef;

use crate::Decoder;

use super::{header::WldHeader, names::WldNames, raw_fragment::WldRawFragment};

#[derive(Debug)]
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
