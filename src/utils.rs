use std::string::FromUtf8Error;

use bytes::{Buf, Bytes};

use crate::EQFilesError;

pub(crate) const HASH_KEY: [u8; 8] = [0x95u8, 0x3A, 0xC5, 0x2A, 0x95, 0x7A, 0x95, 0x6A];

pub fn count<T, E, S: Clone>(
    input: &mut Bytes,
    count: usize,
    settings: S,
    f: fn(&mut Bytes, S) -> Result<T, E>,
) -> Result<Vec<T>, E> {
    let mut temp = Vec::new();
    for _ in 0..count {
        match f(input, settings.clone()) {
            Ok(valid) => temp.push(valid),
            Err(e) => return Err(e),
        }
    }
    Ok(temp)
}

pub fn string(input: &mut Bytes, length: usize) -> Result<String, FromUtf8Error> {
    let data = take(input, length as usize);
    Ok(String::from_utf8(data.to_vec())?
        .trim_end_matches('\0')
        .to_string())
}

pub fn take(input: &mut Bytes, count: usize) -> Bytes {
    input.copy_to_bytes(count)
}

pub fn decode_string(buffer: &mut Bytes, length: usize) -> Result<String, EQFilesError> {
    let mut result = Vec::new();
    for i in 0..length {
        let c = buffer.get_u8() ^ HASH_KEY.get(i % 8).unwrap();
        if c != 0 {
            result.push(c);
        }
    }
    match String::from_utf8(result) {
        Ok(ok) => Ok(ok),
        Err(e) => Err(EQFilesError::ErrorDecodingString(e)),
    }
}
