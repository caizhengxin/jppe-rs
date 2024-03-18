use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct SkipExample {
    #[jppe(skip)]
    pub version: u16,
    pub command: u16,
}


#[test]
fn test_modifier_skip() {
    let (input, value) = jppe::decode::<SkipExample>(b"\x00\x01").unwrap();
    assert_eq!(value, SkipExample{ version: 0, command: 1 });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x00\x01");
}