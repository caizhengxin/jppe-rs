#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode, JResult, InputTrait, ContainerAttrModifiers, FieldAttrModifiers};
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::prelude::*;


/// This is just a demonstration, so go straight back.
fn custom_with_decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], u8> {
    let (input, value) = input.input_take(4)?;

    let key = match value {
        b"read" => 1,
        b"write" => 2,
        _ => return Err(make_error(input, ErrorKind::InvalidByteLength { offset: input.len() })),
    };

    Ok((input, key))
}


fn custom_with_encode(input: &mut Vec<u8>, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, value: &u8) {
    match value {
        1 => input.extend_from_slice(b"read"),
        2 => input.extend_from_slice(b"write"),
        _ => {},
    }
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct WithExample {
    #[jppe(decode_with="custom_with_decode", encode_with="custom_with_encode")]
    pub key: u8,
    pub value: u32,
}


#[test]
fn test_modifier_with2() {
    let input = b"read\x00\x00\x00\x02";
    let (_, value) = WithExample::decode(input, None, None).unwrap();
    assert_eq!(value, WithExample { key: 1, value: 2 });

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"read\x00\x00\x00\x02");
}