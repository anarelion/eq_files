use bitbybit::bitfield;
use bytes::{Buf, Bytes};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::{utils::count, Settings};
use crate::{Decoder, WldFragment};

#[derive(Clone, Debug)]
pub struct WldSkeletonTrackSet {
    pub name: Option<String>,
    pub flags: WldSkeletonFlags,
    pub num_dags: u32,
    pub polygon_animation_reference: u32,
    pub centre_offset: Option<(u32, u32, u32)>,
    pub bounding_radius: Option<f32>,
    pub dags: Vec<WldSkeletonDag>,
    pub mesh_reference_count: Option<u32>,
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
    pub has_mesh_references: bool, // 0x200
}

#[derive(Clone, Debug)]
pub struct WldSkeletonDag {
    pub name: Option<String>,
    pub flags: u32,
    pub track_ref: u32,
    pub mesh_or_sprite_ref: u32,
    pub num_sub_dags: u32,
    pub sub_dags: Vec<u32>,
    pub parent: Option<u32>,
}

impl WldFragment for WldSkeletonTrackSet {
    const TYPE: u32 = 16;
}

impl Decoder<Settings> for WldSkeletonTrackSet {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name().clone();
        let flags = WldSkeletonFlags::new_with_raw_value(input.get_u32_le());
        let num_dags = input.get_u32_le();
        let polygon_animation_reference = input.get_u32_le();
        let centre_offset = flags
            .has_center_offset()
            .then(|| (input.get_u32_le(), input.get_u32_le(), input.get_u32_le()));
        let bounding_radius = flags.has_bounding_radius().then(|| input.get_f32_le());

        let mut dags = count(input, num_dags as usize, settings, WldSkeletonDag::new)?;

        let mesh_reference_count = flags.has_mesh_references().then(|| input.get_u32_le());

        let mut dm_sprites = Vec::new();
        let mut link_skin_updates_to_dag_index = Vec::new();
        if mesh_reference_count.is_some() {
            for _ in 0..mesh_reference_count.unwrap() {
                dm_sprites.push(input.get_u32_le());
            }

            for _ in 0..mesh_reference_count.unwrap() {
                link_skin_updates_to_dag_index.push(input.get_u32_le());
            }
        }

        for index in 0..dags.len() {
            let subs = {
                let dag = dags.get(index).unwrap();
                dag.sub_dags.clone()
            };
            
            for sub in &subs {
                let sb = dags.get_mut(*sub as usize).unwrap();
                sb.parent = Some(index as u32);
            }
        };

        Ok(Self {
            name,
            flags,
            num_dags,
            polygon_animation_reference,
            centre_offset,
            bounding_radius,
            dags,
            mesh_reference_count,
            dm_sprites,
            link_skin_updates_to_dag_index,
        })
    }
}

impl Decoder<Settings> for WldSkeletonDag {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name_ref = input.get_i32_le();
        let name = settings.get_from_name_ref(name_ref);
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
            parent: None,
        })
    }
}

impl Debug for WldSkeletonFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WldSkeletonFlags")
            .field("has_center_offset", &self.has_center_offset())
            .field("has_bounding_radius", &self.has_bounding_radius())
            .field("has_mesh_references", &self.has_mesh_references())
            .finish()
    }
}
