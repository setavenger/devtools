use crate::utils::util::HexSlice;

pub fn reverse_byte_slice(data: &String) -> HexSlice {
    let mut interim = match hex::decode(data) {
        Ok(bytes) => bytes,
        Err(e) => panic!("unable to decode: {:?}", e),
    };

    interim.reverse();

    let data = interim.clone();

    HexSlice::new(data)
}

#[cfg(test)]
mod tests {
    use crate::utils::util::HexSlice;

    use super::reverse_byte_slice;

    #[test]
    fn test_small_reversal() {
        let target = HexSlice::new(vec![0xCD, 0xAB]);
        let result = reverse_byte_slice(&"abcd".to_owned());
        assert_eq!(result, target, "hex slices should be the same")
    }
}
