#![feature(let_chains)]
use std::str::FromStr;

use jppe_derive::{ByteEncode, ByteDecode};
use jppe::prelude::HexString;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteCountExample {
    #[jppe(byte_count=2)]  // Fetches 2 byte mapping length in advance.
    pub value: String,
}


#[test]
fn test_modifier_byte_count() {
    let (input, value) = jppe::decode::<ByteCountExample>(b"\x00\x03\x31\x32\x33").unwrap();
    assert_eq!(value, ByteCountExample { value: "123".to_string() });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x00\x03\x31\x32\x33");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteCountExample2 {
    #[jppe(byte_count=2)] // Fetches 2 byte mapping length in advance.
    pub value: HexString,
}


#[test]
fn test_modifier_byte_count2() {
    let (input, value) = jppe::decode::<ByteCountExample2>(b"\x00\x03\x31\x32\x33").unwrap();
    assert_eq!(value, ByteCountExample2 { value: HexString::from_str("313233").unwrap() });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x00\x03\x31\x32\x33");
}
