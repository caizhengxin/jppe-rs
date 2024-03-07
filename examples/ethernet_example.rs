#![feature(let_chains)]
use std::str::FromStr;

use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::prelude::MacAddress;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct Ethernet {
    pub smac: MacAddress,
    pub dmac: MacAddress,
    pub r#type: u16,
}


fn main() {
    let input = b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00\x45\x00";

    // decode
    let (input_remain, value) = Ethernet::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(value, Ethernet {
        smac: MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap(),
        dmac: MacAddress::from_str("00:00:00:00:00:00").unwrap(),
        r#type: 0x0800,
    });

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00");
    assert_eq!(input_remain, b"\x45\x00");
}
