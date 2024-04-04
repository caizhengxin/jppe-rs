
use jppe::{ByteEncode, ByteDecode, FieldAttrModifiers};
// use jppe::prelude::*;
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample1 {
    #[jppe(offset=2)]
    pub a: u8,
    pub b: u16,
}

#[test]
fn test_struct_offset_example1() {
    let value = OffsetExample1 { a: 1, b: 2};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x00, 0x01, 0x00, 0x02]);

    let (input, value2) = OffsetExample1::decode(&[0x00, 0x00, 0x01, 0x00, 0x02], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample2 {
    pub a: u8,
    #[jppe(offset="a")]
    pub b: u16,
}


#[test]
fn test_struct_offset_example2() {
    let value = OffsetExample2 { a: 1, b: 2 };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x01, 0x00, 0x00, 0x02]);

    let (input, value2) = OffsetExample2::decode(&[0x01, 0x00, 0x00, 0x02], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample3 {
    pub a: u8,
    #[jppe(offset="a - 1")]
    pub b: u16,
}


#[test]
fn test_struct_offset_example3() {
    let value = OffsetExample3 { a: 2, b: 2 };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x02, 0x00, 0x00, 0x02]);

    let (input, value2) = OffsetExample3::decode(&[0x02, 0x00, 0x00, 0x02], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumOffsetExample1 {
    Jkc {
        #[jppe(offset=2)]
        a: u8,
        b: u16,    
    } = 0x02,
    #[jppe(branch_default)]
    Unknown
}


#[test]
fn test_enum_offset_example1() {
    let value = EnumOffsetExample1::Jkc { a: 1, b: 2};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x02, 0x00, 0x00, 0x01, 0x00, 0x02]);

    let fattr = FieldAttrModifiers { branch: Some(2), ..Default::default() };
    let (input, value2) = EnumOffsetExample1::decode(&[0x00, 0x00, 0x01, 0x00, 0x02], None, Some(&fattr)).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumOffsetExample2 {
    Jkc {
        a: u8,
        #[jppe(offset="a")]
        b: u16,    
    } = 0x02,
    #[jppe(branch_default)]
    Unknown
}


#[test]
fn test_enum_offset_example2() {
    let value = EnumOffsetExample2::Jkc { a: 1, b: 2 };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x02, 0x01, 0x00, 0x00, 0x02]);

    let fattr = FieldAttrModifiers { branch: Some(2), ..Default::default() };
    let (input, value2) = EnumOffsetExample2::decode(&[0x01, 0x00, 0x00, 0x02], None, Some(&fattr)).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumOffsetExample3 {
    #[jppe(offset=1)]
    Jkc(u8) = 0x02,
    #[jppe(offset=1)]
    Jkc2(u8, u16) = 0x03,
    #[jppe(offset=1)]
    Jkc3((u8, u16)) = 0x04,
    #[jppe(branch_default)]
    Unknown
}


#[test]
fn test_enum_offset_example3() {
    let value = EnumOffsetExample3::Jkc(1);
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x02, 0x00, 0x01]);

    let (input, value2) = EnumOffsetExample3::decode(&[0x02, 0x00, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}
