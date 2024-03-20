use jdefault_derive::Jdefault;
use jppe::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode, Jdefault)]
pub struct JppeExample {
    #[jppe(default=18)]
    pub value: u16,
}


#[test]
fn test_jppe_default() {
    let value = JppeExample::default();

    assert_eq!(value.value, 18);
}