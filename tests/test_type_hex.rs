use std::str::FromStr;

use jppe::{ByteDecode, ByteEncode};
use jppe::fields::HexString;
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct HexExample {
    pub length: u8,
    #[jppe(length="length")]
    pub value: HexString,
}


#[test]
fn test_type_hex() {
    let (input, value) = HexExample::decode(b"\x09jankincai", None, None).unwrap();
    assert_eq!(value.value.to_string(), "6a616e6b696e636169");
    assert_eq!(value, HexExample {
        length: 9,
        value: HexString::from_str("6a616e6b696e636169").unwrap(),
    });
    assert_eq!(input.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x09jankincai");
}