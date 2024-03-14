use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe::fields::HexBytes;
use jppe_derive::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HexBytesExample<'a> {
    pub length: u8,
    #[jppe(length="length")]
    pub value: HexBytes<'a>,
}


#[test]
fn test_type_hex_bytes() {
    let (input, value) = HexBytesExample::decode(b"\x09jankincai", None, None).unwrap();
    assert_eq!(value.length, 9);
    assert_eq!(value.value.to_string(), "6a616e6b696e636169");
    assert_eq!(input.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x09jankincai");
}