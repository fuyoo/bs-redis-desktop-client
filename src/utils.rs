use anyhow::Result;
use std::fmt;

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where
            T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

// You can choose to implement multiple traits, like Lower and UpperHex
impl fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x} ", byte)?;
        }
        Ok(())
    }
}


pub fn vec_u8_to_string(data: Vec<u8>) -> Result<String> {
    Ok(String::from_utf8(data)?)
}

pub fn vec_u8_to_hex_string(data: &Vec<u8>) -> Result<String> {
    Ok(HexSlice::new(data).to_string())
}

