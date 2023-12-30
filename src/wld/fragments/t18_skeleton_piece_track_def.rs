use std::sync::Arc;

use bytes::{Buf, Bytes};

use crate::{Decoder, Settings, WldFragment};

#[derive(Clone, Debug)]
pub struct WldSkeletonPieceTrackDef {
    pub name_ref: i32,
    pub name: Option<String>,
    pub flags: u32,
    pub frames: Vec<WldSkeletonPieceTrackFrameTransform>,
    pub remainder: Bytes,
}

impl WldFragment for WldSkeletonPieceTrackDef {
    const TYPE: u32 = 18;
}

impl Decoder<Settings> for WldSkeletonPieceTrackDef {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name_ref = settings.get_name_ref();
        let name = settings.get_name();
        let flags = input.get_u32_le(); // bit 3 means more values
        let frame_count = input.get_u32_le();
        let mut frames = Vec::new();
        for _ in 0..frame_count {
            let rotation_denominator = input.get_i16_le();
            let rotation_x = input.get_i16_le();
            let rotation_y = input.get_i16_le();
            let rotation_z = input.get_i16_le();
            let shift_x = input.get_i16_le();
            let shift_y = input.get_i16_le();
            let shift_z = input.get_i16_le();
            let shift_denominator = input.get_i16_le();
            let mut frame_transform = if shift_denominator != 0 {
                WldSkeletonPieceTrackFrameTransform {
                    translation: glam::vec3(
                        shift_x as f32 / 256f32,
                        shift_y as f32 / 256f32,
                        shift_z as f32 / 256f32,
                    ),
                    scale: shift_denominator as f32 / 256f32,
                    ..Default::default()
                }
            } else {
                WldSkeletonPieceTrackFrameTransform {
                    translation: glam::vec3(0.0, 0.0, 0.0),
                    ..Default::default()
                }
            };
            frame_transform.rotation = glam::quat(rotation_x as f32, rotation_y as f32, rotation_z as f32, rotation_denominator as f32).normalize();
            frames.push(frame_transform);
        }

        Ok(Self {
            name_ref,
            name,
            flags,
            frames,
            remainder: input.clone(),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct WldSkeletonPieceTrackFrameTransform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: f32,
    pub model_matrix: glam::Mat4,
}
