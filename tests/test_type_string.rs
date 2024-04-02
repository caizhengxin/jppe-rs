use jppe::prelude::*;
use jppe::{BorrowByteDecode, BorrowByteEncode};


#[test]
fn test_type_string() {
    // example - #[jppe(linend=b"\r\n")]
    let fattr = FieldAttrModifiers { linend_value: Some(b"\r\n"), ..Default::default() };
    let (input, value) = String::decode(b"jankincai\r\n", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"jankincai\r\n");

    // example - #[jppe(length=9)]
    let fattr = FieldAttrModifiers { length: Some(9), ..Default::default() };
    let (input, value) = String::decode(b"jankincai\r\n", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input, b"\r\n");
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"jankincai");

    // example - #[jppe(length=20)]
    let fattr = FieldAttrModifiers { length: Some(20), ..Default::default() };
    assert_eq!(String::decode(b"jankincai\r\n", None, Some(&fattr)).is_err(), true);
}


#[test]
fn test_type_string_byte_count() {
    // example - #[jppe(byte_count=1)]  default
    let (input, value) = String::decode(b"\x09jankincai", None, None).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x09jankincai");

    // example - #[jppe(byte_count=1)]
    let fattr = FieldAttrModifiers { byte_count: Some(1), ..Default::default() };
    let (input, value) = String::decode(b"\x09jankincai", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"\x09jankincai");

    // example - #[jppe(byte_count=2)]
    let fattr = FieldAttrModifiers { byte_count: Some(2), ..Default::default() };
    let (input, value) = String::decode(b"\x00\x09jankincai", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"\x00\x09jankincai");

    // example - #[jppe(byte_count=4)]
    let fattr = FieldAttrModifiers { byte_count: Some(4), ..Default::default() };
    let (input, value) = String::decode(b"\x00\x00\x00\x09jankincai", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"\x00\x00\x00\x09jankincai");

    // example - #[jppe(byte_count=8)]
    let fattr = FieldAttrModifiers { byte_count: Some(8), ..Default::default() };
    let (input, value) = String::decode(b"\x00\x00\x00\x00\x00\x00\x00\x09jankincai", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"\x00\x00\x00\x00\x00\x00\x00\x09jankincai");
}


#[test]
fn test_type_string_key_split_linend() {
    // example - #[jppe(key="Header: ", linend=b"\r\n")]
    let fattr = FieldAttrModifiers { key: Some(b"Header: ".to_vec()), linend_value: Some(b"\r\n"), ..Default::default() };
    let (input, value) = String::decode(b"Header: jankincai\r\n", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"Header: jankincai\r\n");

    // example - #[jppe(key="Header", split=": ", linend=b"\r\n")]
    let fattr = FieldAttrModifiers { key: Some(b"Header".to_vec()), split: Some(vec![b": ".to_vec()]), linend_value: Some(b"\r\n"), ..Default::default() };
    let (input, value) = String::decode(b"Header: jankincai\r\n", None, Some(&fattr)).unwrap();
    assert_eq!(value, "jankincai".to_string());
    assert_eq!(input.is_empty(), true);
    let mut buf = Vec::new();
    value.encode(&mut buf, None, Some(&fattr));
    assert_eq!(buf, b"Header: jankincai\r\n");
}