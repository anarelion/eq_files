use std::collections::BTreeMap;
use std::sync::Arc;

use bytes::Buf;
use bytes::Bytes;

use crate::utils::HASH_KEY;
use crate::Decoder;
use crate::EQFilesError;

#[derive(Clone, Debug, Default)]
pub struct WldNames(BTreeMap<u32, String>);

impl Decoder<u32> for WldNames {
    fn new(input: &mut Bytes, size: Arc<u32>) -> Result<Self, EQFilesError>
    where
        Self: Sized,
    {
        let mut last_offset = 0;
        let mut temp = Vec::new();
        let mut res = BTreeMap::new();
        for i in 0..*size {
            let c = input.get_u8() ^ HASH_KEY.get((i % 8) as usize).unwrap();
            if c == 0 {
                let name = String::from_utf8(temp).map_err(EQFilesError::ErrorDecodingString)?;
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
    pub fn get_name(&self, index: i32) -> Option<String> {
        match index {
            0 => None,
            _ => {
                let name_ref = (!index) as u32;
                Some(
                    self.0
                        .get(&name_ref)
                        .unwrap_or(&format!("{}", name_ref))
                        .clone(),
                )
            }
        }
    }
}
