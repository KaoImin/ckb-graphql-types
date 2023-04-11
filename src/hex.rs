use std::fmt::LowerHex;

use crate::error::Error;

const HEX_PREFIX: &str = "0x";

pub fn hex_encode<T: AsRef<[u8]>>(src: T) -> String {
    HEX_PREFIX.to_string() + &faster_hex::hex_string(src.as_ref())
}

pub fn hex_decode(src: &str) -> Result<Vec<u8>, Error> {
    if src.is_empty() {
        return Ok(Vec::new());
    }

    let src = clean_0x(src)?;
    let src = src.as_bytes();
    let mut ret = vec![0u8; src.len() / 2];
    faster_hex::hex_decode(src, &mut ret)?;

    Ok(ret)
}

pub fn hex_uint<T: LowerHex>(src: T) -> String {
    HEX_PREFIX.to_string() + &format!("{:x}", src)
}

pub fn clean_0x(s: &str) -> Result<String, Error> {
    if s.starts_with("0x") || s.starts_with("0X") {
        Ok(s[2..].to_owned())
    } else {
        Err(Error::HexPrefix)
    }
}
