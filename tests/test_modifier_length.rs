
use jppe::{BorrowByteEncode, BorrowByteDecode, FieldAttrModifiers};
// use jppe::prelude::*;
use jppe_derive::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct StructLengthExample1<'a> {
    #[jppe(length=3)]
    pub a: u32,
    #[jppe(length=1)]
    pub b: &'a [u8],
    #[jppe(count=1, length=3)]  // count indicates the Vec size and length indicates bytes of the int type.
    pub c: Vec<u32>
}


#[test]
fn test_struct_length_example1() {
    let value = StructLengthExample1 { a: 1, b: &[1], c: vec![1]};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01]);

    let (input, value2) = StructLengthExample1::decode(&[0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct StructLengthExample2<'a> {
    pub a: u8,
    #[jppe(length="a")]
    pub b: &'a [u8],
    pub c: u8,
    #[jppe(length="c - 1")]
    pub d: &'a [u8],
}


#[test]
fn test_struct_length_example2() {
    let input = vec![0x01, 0x01, 0x02, 0x01];
    let value = StructLengthExample2 { a: 1, b: &[1], c: 2, d: &[1]};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = StructLengthExample2::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub enum EnumLengthExample1<'a> {
    Jkc {
        a: u8,
        #[jppe(length="a")]
        b: &'a [u8],
        c: u8,
        #[jppe(length="c - 1")]
        d: &'a [u8],    
    },
    #[jppe(branch_default)]
    #[default]
    Unknown
}


#[test]
fn test_struct_length_example3() {
    let input = vec![0x01, 0x01, 0x02, 0x01];
    let value = EnumLengthExample1::Jkc { a: 1, b: &[1], c: 2, d: &[1]} ;
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, [0x00, 0x01, 0x01, 0x02, 0x01]);

    let fattr = FieldAttrModifiers { branch: Some(0), ..Default::default() };
    let (input, value2) = EnumLengthExample1::decode(&input, None, Some(&fattr)).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}
