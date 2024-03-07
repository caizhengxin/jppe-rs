#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct BitsExample {
    #[jppe(bits_start=0xf0, untake)]
    pub version: u8,
    #[jppe(bits=0x0f)]
    pub length: u8,
}


#[test]
fn test_monidifer_bits() {
    // decode
    let (input, value) = BitsExample::decode(b"\x12", None, None).unwrap();
    assert_eq!(value, BitsExample { version: 1, length: 2 });
    assert_eq!(input.is_empty(), true);

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x12");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct BitsExample2 {
    #[jppe(bits_start=0xf000, untake)]
    pub version: u16,
    #[jppe(bits=0x0f00, untake)]
    pub length: u16,
    #[jppe(bits=0x00ff)]
    pub value: u16,
}


#[test]
fn test_monidifer_bits2() {
    // decode
    let (input, value) = BitsExample2::decode(b"\x12\x34", None, None).unwrap();
    assert_eq!(value, BitsExample2 { version: 1, length: 2, value: 0x34 });
    assert_eq!(input.is_empty(), true);

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x12\x34");
}
