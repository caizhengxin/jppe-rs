use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct OptionExample {
    pub flags: bool,
    #[jppe(if_expr="flags == true")]
    pub value: Option<u16>,
}


#[test]
fn test_modifier_if_expr() {
    let (input, value) = OptionExample::decode(b"\x00\x31", None, None).unwrap();
    assert_eq!(value, OptionExample { flags: false, value: None });
    assert_eq!(input, b"\x31");

    let (input, value) = OptionExample::decode(b"\x01\x00\x31", None, None).unwrap();
    assert_eq!(value, OptionExample { flags: true, value: Some(0x31) });
    assert_eq!(input.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x01\x00\x31");
}