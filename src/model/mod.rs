use std::collections::BTreeMap;

use bytes::{Buf, Bytes};

use crate::EQFilesError;

pub struct Model {
    version: u32,
    materials: Vec<ModelMaterial>,
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    triangles: Vec<(u32, u32, u32, u32, u32)>,
}

impl Model {
    pub fn parse(bytes: &mut Bytes) -> Result<Self, EQFilesError> {
        let magic = bytes.get_u32_le();
        if magic != 0x4d475145 {
            return Err(EQFilesError::InvalidMagicNumber(magic));
        }
        let version = bytes.get_u32_le();
        let strings_length = bytes.get_u32_le();
        let material_count = bytes.get_u32_le();
        let vertex_count = bytes.get_u32_le();
        let triangle_count = bytes.get_u32_le();
        let bone_count = bytes.get_u32_le();

        let strings = ModelStringTable::parse(bytes, strings_length as usize)?;
        let mut materials = Vec::new();
        for _ in 0..material_count {
            materials.push(ModelMaterial::parse(bytes, &strings)?);
        }
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for _ in 0..vertex_count {
            vertices.push([bytes.get_f32_le(), bytes.get_f32_le(), bytes.get_f32_le()]);
            normals.push([bytes.get_f32_le(), bytes.get_f32_le(), bytes.get_f32_le()]);
            let u = bytes.get_f32_le();
            let v = bytes.get_f32_le();

            uvs.push([v, u]);
        }
        let mut triangles = Vec::new();
        for _ in 0..triangle_count {
            let a = bytes.get_u32_le();
            let b = bytes.get_u32_le();
            let c = bytes.get_u32_le();
            let material_id = bytes.get_u32_le();
            let flags = bytes.get_u32_le();
            triangles.push((a, b, c, material_id, flags));
        }

        //TODO: check format on bones

        Ok(Self {
            version,
            materials,
            vertices,
            normals,
            uvs,
            triangles,
        })
    }
}

struct ModelStringTable {
    arena: Bytes,
    size: usize,
}

impl ModelStringTable {
    fn parse(bytes: &mut Bytes, size: usize) -> Result<Self, EQFilesError> {
        Ok(Self {
            arena: bytes.copy_to_bytes(size),
            size,
        })
    }

    fn get_string(&self, offset: usize) -> Result<String, EQFilesError> {
        if offset > self.size {
            return Err(EQFilesError::UnknownError);
        }
        let mut temp = Vec::new();
        let mut it = self.arena.clone();
        it.advance(offset);
        let mut c = it.get_u8();
        while c != 0 {
            temp.push(c);
            c = it.get_u8();
        }
        Ok(String::from_utf8(temp)?)
    }
}

struct ModelMaterial {
    index: u32,
    material_name: String,
    shader_name: String,
    properties: ModelMaterialProperties,
}

impl ModelMaterial {
    fn parse(bytes: &mut Bytes, strings: &ModelStringTable) -> Result<Self, EQFilesError> {
        let index = bytes.get_u32_le();
        let material_name = strings.get_string(bytes.get_u32_le() as usize)?;
        let shader_name = strings.get_string(bytes.get_u32_le() as usize)?;
        let properties = ModelMaterialProperties::parse(bytes, &strings)?;

        Ok(Self {
            index,
            material_name,
            shader_name,
            properties,
        })
    }
}

#[derive(Debug)]
struct ModelMaterialProperties(BTreeMap<String, ModelMaterialPropertyValue>);

impl ModelMaterialProperties {
    fn parse(bytes: &mut Bytes, strings: &ModelStringTable) -> Result<Self, EQFilesError> {
        let mut result = BTreeMap::new();
        let count = bytes.get_u32_le();
        for _ in 0..count {
            let name = strings.get_string(bytes.get_u32_le() as usize)?;
            match bytes.get_u32_le() {
                0 => {
                    result.insert(name, ModelMaterialPropertyValue::Float(bytes.get_f32_le()));
                }
                2 => {
                    result.insert(
                        name,
                        ModelMaterialPropertyValue::String(
                            strings.get_string(bytes.get_u32_le() as usize)?,
                        ),
                    );
                }
                _ => todo!(),
            }
        }

        Ok(ModelMaterialProperties(result))
    }
}

#[derive(Debug)]
enum ModelMaterialPropertyValue {
    Float(f32),
    String(String),
}
