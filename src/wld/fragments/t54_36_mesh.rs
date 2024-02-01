use std::collections::BTreeMap;
use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldMesh {
    pub name: Option<String>,

    pub flags: u32,
    pub animation_ref: u32,
    pub centre: (f32, f32, f32),
    pub color_count: u16,
    pub scale: f32,
    pub material_group_count: u16,
    pub material_list_ref: u32,
    pub max_distance: f32,
    pub max: (f32, f32, f32),
    pub mesh_animated_bone_count: u16,
    pub min: (f32, f32, f32),
    pub normal_count: u16,
    pub params2: (u32, u32, u32),
    pub triangle_count: u16,
    pub unk1_frag_ref: u32,
    pub unk2_frag_ref: u32,
    pub uv_count: u16,
    pub vertex_count: u16,
    pub vertex_piece_count: u16,
    pub vertex_texture_count: u16,

    pub color: Vec<[u8; 4]>,
    pub material_group: Vec<(u16, u16)>,
    pub mesh_animated_bone: Vec<[i16; 3]>,
    pub normal: Vec<[f32; 3]>,
    pub position: Vec<[f32; 3]>,
    pub triangle: Vec<[u16; 4]>,
    pub uv: Vec<[f32; 2]>,
    pub vertex_piece: BTreeMap<u16, (u16, u16)>,
    pub vertex_texture: Vec<[u16; 2]>,
}

impl WldFragment for WldMesh {
    const TYPE: u32 = 54;
}

impl Decoder<Settings> for WldMesh {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name = settings.get_name().clone();
        let flags = input.get_u32_le();
        let material_list_ref = input.get_u32_le();
        let animation_ref = input.get_u32_le();
        let unk1_frag_ref = input.get_u32_le();
        let unk2_frag_ref = input.get_u32_le();
        let centre = (input.get_f32_le(), input.get_f32_le(), input.get_f32_le());
        let params2 = (input.get_u32_le(), input.get_u32_le(), input.get_u32_le());

        let max_distance = input.get_f32_le();
        let min = (input.get_f32_le(), input.get_f32_le(), input.get_f32_le());
        let max = (input.get_f32_le(), input.get_f32_le(), input.get_f32_le());
        let vertex_count = input.get_u16_le();
        let uv_count = input.get_u16_le();
        let normal_count = input.get_u16_le();
        let color_count = input.get_u16_le();
        let triangle_count = input.get_u16_le(); // face count
        let vertex_piece_count = input.get_u16_le(); // skin assignment group
        let material_group_count = input.get_u16_le(); // face material group
        let vertex_texture_count = input.get_u16_le(); // vertex material
        let mesh_animated_bone_count = input.get_u16_le(); // meshop count
        let scale = 1f32 / (1u32 << input.get_u16_le()) as f32;

        let mut vertex = Vec::new();
        for _ in 0..vertex_count {
            let (v1, v2, v3) = (input.get_i16_le(), input.get_i16_le(), input.get_i16_le());
            vertex.push([
                (v1 as f32) * scale,
                (v2 as f32) * scale,
                (v3 as f32) * scale,
            ]);
        }

        let mut uv = Vec::new();
        for _ in 0..uv_count {
            if settings.is_old_world() {
                // TODO: this value assumes all textures are 256x256 as it is in pixels
                uv.push([
                    (input.get_i16_le() as f32) / 256f32,
                    (input.get_i16_le() as f32) / 256f32,
                ]);
            } else {
                uv.push([
                    (input.get_i32_le() as f32) / 256f32,
                    (input.get_i32_le() as f32) / 256f32,
                ]);
            }
        }

        let mut normal = Vec::new();
        for _ in 0..normal_count {
            normal.push([
                (input.get_i8() as f32) / 127f32,
                (input.get_i8() as f32) / 127f32,
                (input.get_i8() as f32) / 127f32,
            ]);
        }

        let mut color = Vec::new();
        for _ in 0..color_count {
            color.push([
                input.get_u8(),
                input.get_u8(),
                input.get_u8(),
                input.get_u8(),
            ]);
        }

        // MeshFragmentFaceEntry
        let mut triangle = Vec::new();
        for _ in 0..triangle_count {
            triangle.push([
                input.get_u16_le(),
                input.get_u16_le(),
                input.get_u16_le(),
                input.get_u16_le(),
            ]);
        }

        // skin_assignment_groups_count -- vertices assigned to each bone
        let mut vertex_piece = BTreeMap::new();
        let mut mobstart = 0;
        for _ in 0..vertex_piece_count {
            let count = input.get_u16_le();
            let bone_index = input.get_u16_le();
            let item = (bone_index, count);
            vertex_piece.insert(mobstart, item);
            mobstart += count;
        }

        // face_material_groups_count
        let mut material_group = Vec::new();
        for _ in 0..material_group_count {
            material_group.push((input.get_u16_le(), input.get_u16_le()));
        }

        // vertex_material_groups_count
        let mut vertex_texture = Vec::new();
        for _ in 0..vertex_texture_count {
            vertex_texture.push([input.get_u16_le(), input.get_u16_le()]);
        }

        // MeshFragmentMeshOpEntry
        let mut mesh_animated_bone = Vec::new();
        for _ in 0..mesh_animated_bone_count {
            mesh_animated_bone.push([
                (input.get_i16_le()),
                (input.get_i16_le()),
                (input.get_i16_le()),
            ]);
        }

        // info!("Remainder : {:?}", input.to_vec());

        Ok(Self {
            animation_ref,
            centre,
            color,
            color_count,
            flags,
            material_list_ref,
            max_distance,
            max,
            scale,
            mesh_animated_bone,
            mesh_animated_bone_count,
            min,
            name,
            normal,
            normal_count,
            material_group,
            material_group_count,
            triangle,
            triangle_count,
            unk1_frag_ref,
            unk2_frag_ref,
            params2,
            uv,
            uv_count,
            position: vertex,
            vertex_count,
            vertex_piece,
            vertex_piece_count,
            vertex_texture,
            vertex_texture_count,
        })
    }
}
