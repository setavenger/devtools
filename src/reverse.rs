use crate::utils::util::HexSlice;

pub fn reverse_byte_slice(data: &String) {
    let mut interim = match hex::decode(data) {
        Ok(bytes) => bytes,
        Err(e) => panic!("unable to decode: {:?}", e),
    };

    interim.reverse();

    println!("{}", HexSlice::new(&interim))
}
