use std::collections::BTreeMap;
use std::sync::Arc;
use tracing::info;

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
    pub face_material_group_count: u16,
    pub material_list_ref: u32,
    pub max_distance: f32,
    pub max: (f32, f32, f32),
    pub mesh_op_count: u16,
    pub min: (f32, f32, f32),
    pub normal_count: u16,
    pub params2: (u32, u32, u32),
    pub triangle_count: u16,
    pub unk1_frag_ref: u32,
    pub unk2_frag_ref: u32,
    pub uv_count: u16,
    pub vertex_count: u16,
    pub vertex_bone_group_count: u16,
    pub vertex_material_group_count: u16,

    pub color: Vec<[u8; 4]>,
    pub mesh_op: Vec<MeshOp>,
    pub normal: Vec<[f32; 3]>,
    pub position: Vec<[f32; 3]>,
    pub triangle: Vec<[u16; 4]>,
    pub uv: Vec<[f32; 2]>,
    pub face_material_group: Vec<(u16, u16, u16)>,
    pub vertex_bone_group: Vec<(u16, u16, u16)>,
    pub vertex_material_group: Vec<(u16, u16, u16)>,
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
        let vertex_bone_group_count = input.get_u16_le(); // skin assignment group
        let face_material_group_count = input.get_u16_le(); // face material group
        let vertex_material_group_count = input.get_u16_le(); // vertex material
        let mesh_op_count = input.get_u16_le(); // meshop count
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
                panic!("Uncertain about texture coordinates new format {:?}", uv);
            }
        }

        let mut normal = Vec::new();
        for _ in 0..normal_count {
            normal.push([
                (input.get_i8() as f32) / 128f32,
                (input.get_i8() as f32) / 128f32,
                (input.get_i8() as f32) / 128f32,
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

        // which vertices are assigned to each bone
        let mut vertex_bone_group = Vec::new();
        let mut idx1 = 0;
        for _ in 0..vertex_bone_group_count {
            let count = input.get_u16_le();
            let target = input.get_u16_le();
            vertex_bone_group.push((idx1, count, target));
            idx1 += count;
        }

        // which faces/triangles use a material
        let mut face_material_group = Vec::new();
        let mut idx2 = 0;
        for _ in 0..face_material_group_count {
            let count = input.get_u16_le();
            let target = input.get_u16_le();
            face_material_group.push((idx2, count, target));
            idx2 += count;
        }

        // which vertices use a material
        let mut vertex_material_group = Vec::new();
        let mut idx3 = 0;
        for _ in 0..vertex_material_group_count {
            let count = input.get_u16_le();
            let target = input.get_u16_le();
            vertex_material_group.push((idx3, count, target));
            idx3 += count;
        }

        // MeshFragmentMeshOpEntry
        let mut mesh_op = Vec::new();
        for _ in 0..mesh_op_count {
            let mut unknown = input.clone().take(4);
            input.get_i32_le();
            let param1 = input.get_u8();
            let type_field = input.get_u8();
            let offset = match type_field {
                4 => Some(unknown.get_f32_le()),
                _ => None,
            };
            let (index1, index2) = match type_field {
                4 => (None, None),
                _ => (Some(unknown.get_u16_le()), Some(unknown.get_u16_le())),
            };
            mesh_op.push(MeshOp {
                type_field,
                index1,
                index2,
                offset,
                param1,
            })
        }

        // if input.remaining() > 0 {
        //     info!("Remainder for {:?}: {:?}", name, input.to_vec());
        // }

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
            mesh_op,
            mesh_op_count,
            min,
            name,
            normal,
            normal_count,
            face_material_group,
            face_material_group_count,
            triangle,
            triangle_count,
            unk1_frag_ref,
            unk2_frag_ref,
            params2,
            uv,
            uv_count,
            position: vertex,
            vertex_count,
            vertex_bone_group,
            vertex_bone_group_count,
            vertex_material_group,
            vertex_material_group_count,
        })
    }
}

// copied from https://github.com/bryab/libeq/blob/master/crates/libeq_wld/src/parser/fragments/mesh.rs#L457
#[derive(Clone, Debug)]
pub struct MeshOp {
    /// _Unknown_ - It seems to control whether `index1`, `index2`, and `offset` exist. It can only
    /// contain values in the range 1-4. It looks like the [MeshFragmentMeshOpEntry]s are broken up into
    /// blocks, where each block is terminated by an entry where `type_field` is 4.
    ///
    /// The type of MESHOP, one of:
    /// 1: SW (vertex_index: u16, vertex_index: u16, type: u8) e.g. "MESHOP_SW 1553 1 1569" where the arguments are re-arranged to 1553 1569 0
    /// 2: FA (face_index: u16) + 3 empty bytes
    /// 3: VA (vertex_index: u16) + 3 empty bytes
    /// 4: EL (offset: f32) + 1 empty byte
    pub type_field: u8,

    /// _Unknown_ - This seems to reference one of the vertex entries. This field is only valid if
    /// `type_field` contains 1. Otherwise, this field must contain 0.
    pub index1: Option<u16>,

    /// _Unknown_ - This seems to reference one of the vertex entries. This field is only valid if
    /// `type_field` contains 1. Otherwise, this field must contain 0.
    pub index2: Option<u16>,

    /// _Unknown_ - If `type_field` contains 4, then this field exists instead of `index1`
    /// and `index2`. [MeshFragmentMeshOpEntry]s seem to be sorted by this value.
    pub offset: Option<f32>,

    /// _Unknown_ - It seems to only contain values in the range 0-2.
    pub param1: u8,
}
