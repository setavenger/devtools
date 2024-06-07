// convert: does the conversions between different data formats: hex, binary, decimal
use base64::{engine::general_purpose, Engine as _};
use clap::ValueEnum;
use hex;
use num_bigint::{BigInt, Sign};

use crate::utils::util;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum StringConversionOptions {
    Decimal,
    Hex,
    Base64,
    Binary,
    Ascii,
}

pub fn parse_data(input: String, from: StringConversionOptions, to: StringConversionOptions) {
    let interim: Vec<u8>;

    match from {
        StringConversionOptions::Hex => {
            interim = match hex::decode(input) {
                Ok(bytes) => bytes,
                Err(e) => panic!("unable to decode: {:?}", e),
            };
        }
        StringConversionOptions::Decimal => interim = decimal_str_to_bytes(&input),
        StringConversionOptions::Binary => {
            interim = match binary_string_to_bytes(&input) {
                Ok(data_bytes) => data_bytes,
                Err(e) => panic!("unable to decode: {:?}", e),
            }
        }
        _ => todo!(),
    };

    match to {
        StringConversionOptions::Base64 => {
            println!("{}", general_purpose::STANDARD.encode(&interim))
        }
        StringConversionOptions::Binary => {
            println!("{}", vec_to_binary_string(interim))
        }
        StringConversionOptions::Decimal => {
            println!("{:?}", vec_to_bigint(interim, true))
        }
        StringConversionOptions::Hex => {
            println!("{}", util::HexSlice::new(&interim))
        }
        _ => todo!(),
    }
}

fn vec_to_binary_string(vec: Vec<u8>) -> String {
    let binary_string = vec
        .iter()
        .map(|&byte| format!("{:08b}", byte)) // Convert each byte to a binary string with padding
        .collect::<Vec<String>>() // Collect into a vector of strings
        .join(""); // Join all strings into one

    // Trim leading zeros
    binary_string.trim_start_matches('0').to_string()
}

fn vec_to_bigint(vec: Vec<u8>, big_endian: bool) -> BigInt {
    if big_endian {
        // Convert from big-endian bytes
        BigInt::from_bytes_be(Sign::Plus, &vec)
    } else {
        // Convert from little-endian bytes
        BigInt::from_bytes_le(Sign::Plus, &vec)
    }
}

fn decimal_str_to_bytes(decimal: &str) -> Vec<u8> {
    decimal
        .parse::<BigInt>()
        .expect("Invalid number")
        .to_bytes_be()
        .1
}

fn binary_string_to_bytes(binary_str: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    binary_str
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let bit_str = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(bit_str, 2)
        })
        .collect()
}
