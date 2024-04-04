
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub version: u8,
    pub value: String,
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum SimpleExampleBody {
    Read {
        address: u8,
    } = 1,
    Write {
        address: u8,
        value: [u8; 3],
    },
    #[jppe(enum_default)]
    Unknown, 
}


fn main() {
    let input = b"\x01\x03\x31\x32\x33\x01\x05";
    let (input_remain, value) = jppe::decode::<SimpleExample>(input).unwrap();
    assert_eq!(value, SimpleExample { version: 1, value: "123".to_string(), body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(input_remain.is_empty(), true);
    assert_eq!(jppe::encode(value), input);
}