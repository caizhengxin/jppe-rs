#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct BitsValueExample {
    #[jppe(bits_start=0xf0, untake)]
    pub version: u8,
    #[jppe(bits=0x0f, value_decode="length * 2", value_encode="length / 2")]
    pub length: u8,
}


#[test]
fn test_monidifer_bits_value() {
    // decode
    let (input, value) = BitsValueExample::decode(b"\x12", None, None).unwrap();
    assert_eq!(value, BitsValueExample { version: 1, length: 4 });
    assert_eq!(input.is_empty(), true);

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x12");
}
