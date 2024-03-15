#![feature(let_chains)]
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct EnumByteCountExample {
    pub version: u8,
    #[jppe(byte_count=1)]
    pub body: EnumByteCountBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
enum EnumByteCountBody {
    Read {
        value: u8,
    } = 1,
    Write {
        address: u8,
    }
}


#[test]
fn test_type_enum_byte_count() {
    let (input, value) = jppe::decode::<EnumByteCountExample>(b"\x01\x01\x02").unwrap();
    assert_eq!(value, EnumByteCountExample { version: 1, body: EnumByteCountBody::Read { value: 2 } });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x01\x01\x02");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct EnumByteCountExample2 {
    pub version: u8,
    #[jppe(byte_count=2)]
    pub body: EnumByteCountBody2,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
enum EnumByteCountBody2 {
    Read {
        value: u8,
    } = 1,
    Write {
        address: u8,
    }
}


#[test]
fn test_type_enum_byte_count2() {
    let (input, value) = jppe::decode::<EnumByteCountExample2>(b"\x01\x00\x01\x02").unwrap();
    assert_eq!(value, EnumByteCountExample2 { version: 1, body: EnumByteCountBody2::Read { value: 2 } });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x01\x00\x01\x02");
}