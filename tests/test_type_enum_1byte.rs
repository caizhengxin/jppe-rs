
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct EnumByteCountExample {
    pub version: u8,
    // Fetches 1 byte mapping enum in advance.
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