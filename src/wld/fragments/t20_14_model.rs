use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;
use glam::Vec3;

use crate::Decoder;
use crate::Settings;
use crate::WldFragment;

#[derive(Clone, Debug)]
pub struct WldModel {
    pub name_ref: i32,
    pub name: Option<String>,
    pub flags: u32,
    pub callback_name_ref: i32,
    pub callback_name: Option<String>,
    pub action_count: u32,
    pub fragment_count: u32,
    pub bounds_ref: u32,
    pub current_action: Option<u32>,
    pub offset_rotation: Option<(Vec3, Vec3, u32)>,
    pub actions: Vec<WldModelAction>,
    pub fragments: Vec<u32>,
    pub some_other_count: u32,
}

#[derive(Clone, Debug)]
pub struct WldModelAction {
    pub lod_count: u32,
    pub unk1: u32,
    pub lod: Vec<f32>,
}

impl WldFragment for WldModel {
    const TYPE: u32 = 20;
}

impl Decoder<Settings> for WldModel {
    fn new(input: &mut Bytes, settings: Arc<Settings>) -> Result<Self, crate::EQFilesError>
    where
        Self: Sized,
    {
        let name_ref = settings.get_name_ref();
        let name = settings.get_name();
        let flags = input.get_u32_le();
        let callback_name_ref = input.get_i32_le();
        let callback_name = settings.get_from_name_ref(callback_name_ref);
        let action_count = input.get_u32_le();
        let fragment_count = input.get_u32_le();
        let bounds_ref = input.get_u32_le();
        let current_action = if flags & 1 == 1 {
            Some(input.get_u32_le())
        } else {
            None
        };

        let offset_rotation = if flags & 2 == 2 {
            Some((
                (input.get_f32_le(), input.get_f32_le(), input.get_f32_le()).into(),
                (input.get_f32_le(), input.get_f32_le(), input.get_f32_le()).into(),
                input.get_u32_le(),
            ))
        } else {
            None
        };

        let mut actions = Vec::new();
        for _ in 0..action_count {
            let lod_count = input.get_u32_le();
            let unk1 = input.get_u32_le();
            let mut lod = Vec::new();
            for _ in 0..lod_count {
                lod.push(input.get_f32_le());
            }
            actions.push(WldModelAction {
                lod_count,
                unk1,
                lod,
            })
        }

        let mut fragments = Vec::new();
        for _ in 0..fragment_count {
            fragments.push(input.get_u32_le());
        }

        let some_other_count = input.get_u32_le();

        Ok(Self {
            name_ref,
            name,
            flags,
            callback_name_ref,
            callback_name,
            action_count,
            fragment_count,
            bounds_ref,
            current_action,
            offset_rotation,
            actions,
            fragments,
            some_other_count,
        })
    }
}
