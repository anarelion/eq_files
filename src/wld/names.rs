use bytes::{Buf, Bytes};
use std::collections::HashMap;

use crate::utils::HASH_KEY;
use crate::{Decoder, EQFilesError};

#[derive(Debug, Default)]
pub struct WldNames(HashMap<u32, String>);

impl Decoder for WldNames {
    type Settings = u32;

    fn new(input: &mut Bytes, size: Self::Settings) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let mut last_offset = 0;
        let mut temp = Vec::new();
        let mut res = HashMap::new();
        for i in 0..size {
            let c = input.get_u8() ^ HASH_KEY.get((i % 8) as usize).unwrap();
            if c == 0 {
                let name =
                    String::from_utf8(temp).map_err(|e| EQFilesError::ErrorDecodingString(e))?;
                res.insert(last_offset, name);
                last_offset = i;
                temp = Vec::new();
            } else {
                temp.push(c);
            }
        }
        Ok(WldNames(res))
    }
}

impl WldNames {
    pub fn get_name(&self, input: &mut Bytes) -> Option<String> {
        let ref_id = input.get_u32_le();
        match ref_id {
            0 => None,
            _ => {
                let name_ref = (!ref_id) as u32;
                Some(
                    self.0
                        .get(&name_ref)
                        .or(Some(&format!("{}", name_ref)))
                        .unwrap()
                        .clone(),
                )
            }
        }
    }
}
