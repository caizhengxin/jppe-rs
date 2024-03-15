#![feature(let_chains)]
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteCountExample {
    #[jppe(byte_count=2)]
    pub value: String,
}


#[test]
fn test_modifier_byte_count() {
    let (input, value) = jppe::decode::<ByteCountExample>(b"\x00\x03\x31\x32\x33").unwrap();
    assert_eq!(value, ByteCountExample { value: "123".to_string() });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x00\x03\x31\x32\x33");
}