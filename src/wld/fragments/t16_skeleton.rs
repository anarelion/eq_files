use bitbybit::bitfield;
use bytes::{Buf, Bytes};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::utils::count;
use crate::wld::names::WldNames;
use crate::Decoder;

#[derive(Clone, Debug)]
pub struct WldSkeleton {
    pub name: Option<String>,
    pub flags: WldSkeletonFlags,
    pub num_dags: u32,
    pub collision_volume_reference: u32,
    pub centre_offset: Option<(u32, u32, u32)>,
    pub bounding_radius: Option<f32>,
    pub dags: Vec<WldSkeletonDag>,
    pub num_attached_skins: Option<u32>,
    pub dm_sprites: Vec<u32>,
    pub link_skin_updates_to_dag_index: Vec<u32>,
}

#[bitfield(u32)]
pub struct WldSkeletonFlags {
    #[bit(0, r)]
    pub has_center_offset: bool, // 0x01
    #[bit(1, r)]
    pub has_bounding_radius: bool, // 0x02
    #[bit(9, r)]
    pub unknown: bool, // 0x200
}

#[derive(Clone, Debug)]
pub struct WldSkeletonDag {
    pub name: Option<String>,
    pub flags: u32,
    pub track_ref: u32,
    pub mesh_or_sprite_ref: u32,
    pub num_sub_dags: u32,
    pub sub_dags: Vec<u32>,
}

impl Decoder for WldSkeleton {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let flags = WldSkeletonFlags::new_with_raw_value(input.get_u32_le());
        let num_dags = input.get_u32_le();
        let collision_volume_reference = input.get_u32_le();
        let centre_offset = if flags.has_center_offset() {
            Some((input.get_u32_le(), input.get_u32_le(), input.get_u32_le()))
        } else {
            None
        };
        let bounding_radius = if flags.has_bounding_radius() {
            Some(input.get_f32_le())
        } else {
            None
        };

        let dags = count(input, num_dags as usize, settings, WldSkeletonDag::new)?;

        let num_attached_skins = if flags.unknown() {
            Some(input.get_u32_le())
        } else {
            None
        };

        let mut dm_sprites = Vec::new();
        let mut link_skin_updates_to_dag_index = Vec::new();
        if num_attached_skins.is_some() {
            for _ in 0..num_attached_skins.unwrap() {
                dm_sprites.push(input.get_u32_le());
            }

            for _ in 0..num_attached_skins.unwrap() {
                link_skin_updates_to_dag_index.push(input.get_u32_le());
            }
        }

        Ok(Self {
            name,
            flags,
            num_dags,
            collision_volume_reference,
            centre_offset,
            bounding_radius,
            dags,
            num_attached_skins,
            dm_sprites,
            link_skin_updates_to_dag_index,
        })
    }
}

impl Decoder for WldSkeletonDag {
    type Settings = Rc<WldNames>;

    fn new(input: &mut Bytes, settings: Self::Settings) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name(input);
        let flags = input.get_u32_le();

        let track_ref = input.get_u32_le();
        let mesh_or_sprite_ref = input.get_u32_le();
        let num_sub_dags = input.get_u32_le();
        let mut sub_dags = Vec::with_capacity(num_sub_dags as usize);
        for _ in 0..num_sub_dags {
            sub_dags.push(input.get_u32_le());
        }

        Ok(Self {
            name,
            flags,
            track_ref,
            mesh_or_sprite_ref,
            num_sub_dags,
            sub_dags,
        })
    }
}

impl Debug for WldSkeletonFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WldSkeletonFlags")
            .field("has_center_offset", &self.has_center_offset())
            .field("has_bounding_radius", &self.has_bounding_radius())
            .field("unknown", &self.unknown())
            .finish()
    }
}
