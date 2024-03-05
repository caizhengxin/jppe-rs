#![feature(let_chains)]
use jppe_rs::{ByteEncode, ByteDecode};
use jppe_rs_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct UntakeExample1 {
    #[jppe_rs(untake)]
    pub a: u8,
    pub b: u16,
    #[jppe_rs(branch="a")]
    pub c: UntakeEnumExample1,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum UntakeEnumExample1 {
    Read {
        #[jppe_rs(untake)]
        a: u8,
        b: u16,    
    },
    #[jppe_rs(enum_default)]
    Unknown, 
}


#[test]
fn test_untake_example1() {
    let (input, value) = UntakeExample1::decode(&[0x00, 0x01, 0x00, 0x01], None, None).unwrap();
    assert_eq!(input.is_empty(), true);
    assert_eq!(value, UntakeExample1 { a: 0, b: 1, c: UntakeEnumExample1::Read { a: 0, b: 1 } });

    let value = UntakeExample1 { a: 0, b: 1, c: UntakeEnumExample1::Read { a: 0, b: 1 } };
    let mut buf = vec![];

    value.encode(&mut buf, None, None);

    assert_eq!(buf, [0x00, 0x01, 0x00, 0x01]);
}
