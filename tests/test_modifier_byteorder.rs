
use jppe::{ByteEncode, ByteDecode};
use jppe::prelude::*;
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteorderExample1 {
    pub ua: u8,
    pub ub: u16,
    pub uc: u32,
    pub ud: u64,
    pub ue: usize,
    pub uf: u128,
    pub ia: i8,
    pub ib: i16,
    pub ic: i32,
    pub id: i64,
    pub ie: isize,
    pub r#if: i128,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jppe(byteorder="LE")]
pub struct ByteorderExample2 {
    pub ua: u8,
    pub ub: u16,
    pub uc: u32,
    pub ud: u64,
    pub ue: usize,
    pub uf: u128,
    pub ia: i8,
    pub ib: i16,
    pub ic: i32,
    pub id: i64,
    pub ie: isize,
    pub r#if: i128,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteorderExample3 {
    pub ua: u8,
    #[jppe(byteorder="LE")]
    pub ub: u16,
    pub uc: u32,
    pub ud: u64,
    pub ue: usize,
    pub uf: u128,
    pub ia: i8,
    #[jppe(byteorder="LE")]
    pub ib: i16,
    pub ic: i32,
    pub id: i64,
    pub ie: isize,
    pub r#if: i128,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteorderExample4 {
    // 0: BE, 1: LE
    pub a: u8,
    #[jppe(byteorder="a")]
    pub b: u16,
}


#[test]
fn test_byteorder_default() {
    let value = ByteorderExample1 {
        ua: 1, ub: 2, uc: 3, ud: 4, ue: 5, uf: 6,
        ia: -1, ib: -2, ic: -3, id: -4, ie: -5, r#if: -6,
    };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [
        1,
        0, 2,
        0, 0, 0, 3,
        0, 0, 0, 0, 0, 0, 0, 4,
        0, 0, 0, 0, 0, 0, 0, 5,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        255,
        255, 254,
        255, 255, 255, 253,
        255, 255, 255, 255, 255, 255, 255, 252,
        255, 255, 255, 255, 255, 255, 255, 251,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 250
    ]);

    let input = buf;
    let (input, value1) = ByteorderExample1::decode(&input, None, None).unwrap();
    assert_eq!(value, value1);
    assert_eq!(input.is_empty(), true);
}


#[test]
fn test_byteorder2() {
    let value = ByteorderExample2 {
        ua: 1, ub: 2, uc: 3, ud: 4, ue: 5, uf: 6,
        ia: -1, ib: -2, ic: -3, id: -4, ie: -5, r#if: -6,
    };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [
        1,
        2, 0,
        3, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 0, 0, 0, 0, 0, 0,
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        255,
        254, 255,
        253, 255, 255, 255,
        252, 255, 255, 255, 255, 255, 255, 255,
        251, 255, 255, 255, 255, 255, 255, 255,
        250, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
    ]);

    let input = buf;
    let (input, value1) = ByteorderExample2::decode(&input, None, None).unwrap();
    assert_eq!(value, value1);
    assert_eq!(input.is_empty(), true);    
}


#[test]
fn test_byteorder3() {
    let value = ByteorderExample3 {
        ua: 1, ub: 2, uc: 3, ud: 4, ue: 5, uf: 6,
        ia: -1, ib: -2, ic: -3, id: -4, ie: -5, r#if: -6,
    };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [
        1,
        2, 0,
        0, 0, 0, 3,
        0, 0, 0, 0, 0, 0, 0, 4,
        0, 0, 0, 0, 0, 0, 0, 5,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        255,
        254, 255,
        255, 255, 255, 253,
        255, 255, 255, 255, 255, 255, 255, 252,
        255, 255, 255, 255, 255, 255, 255, 251,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 250
    ]);

    let input = buf;
    let (input, value1) = ByteorderExample3::decode(&input, None, None).unwrap();
    assert_eq!(value, value1);
    assert_eq!(input.is_empty(), true);    
}


#[test]
fn test_byteorder4() {
    let value = ByteorderExample4 {
        a: 1,
        b: 1,
    };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [0x01, 0x01, 0x00]);

    let input = buf;
    let (input, value1) = ByteorderExample4::decode(&input, None, None).unwrap();
    assert_eq!(value, value1);
    assert_eq!(input.is_empty(), true);

    let value = ByteorderExample4 {
        a: 0,
        b: 1,
    };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [0x00, 0x00, 0x01]);

    let input = buf;
    let (input, value1) = ByteorderExample4::decode(&input, None, None).unwrap();
    assert_eq!(value, value1);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumByteorderExample1 {
    #[jppe(byteorder="LE")]
    Jkc1(u16, u16) = 0x01,
    #[jppe(branch_default)]
    Unknown
}


#[test]
fn test_enum_byteorder_example1() {
    let value = EnumByteorderExample1::Jkc1(1, 2);
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x01, 0x01, 0x00, 0x02, 0x00]);

    let fattr = FieldAttrModifiers { branch: Some(1), ..Default::default() };
    let (input, value2) = EnumByteorderExample1::decode(&[0x01, 0x00, 0x02, 0x00], None, Some(&fattr)).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}
