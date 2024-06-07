use std::fmt;

#[derive(Debug)]
pub struct HexSlice(Vec<u8>);

impl HexSlice {
    /// Creates a new [`HexSlice`].
    pub fn new(data: Vec<u8>) -> HexSlice {
        HexSlice(data)
    }
}

// You can choose to implement multiple traits, like Lower and UpperHex
impl fmt::Display for HexSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            match write!(f, "{:02x}", byte) {
                Ok(_) => continue,
                Err(e) => panic!("{}", e),
            }
        }
        Ok(())
    }
}

impl PartialEq for HexSlice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn print_hex_slice(hex_slice: HexSlice) {
    println!("{}", hex_slice)
}
