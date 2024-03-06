#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    #[jppe(length="length")]
    pub value: String,
    pub cmd: u8,
    #[jppe(branch="cmd")]
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
    let input = b"\x00\x03\x31\x32\x33\x01\x05";

    let (input_remain, value) = SimpleExample::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");

    let mut buf = vec![];
    value.encode(&mut buf, None, None);

    assert_eq!(buf, input);
}