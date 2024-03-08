#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode, JResult, InputTrait, ContainerAttrModifiers, FieldAttrModifiers};
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::prelude::*;


/// This is just a demonstration, so go straight back.
fn custom_with_decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], WithExample> {
    let (input, value) = input.input_take(4)?;

    let key = match value {
        b"read" => 1,
        b"write" => 2,
        _ => return Err(make_error(input, ErrorKind::InvalidByteLength { offset: input.len() })),
    };

    let (input, value) = input.to_be_bits(3)?;

    Ok((input, WithExample {key, value: value as u32}))
}


fn custom_with_encode(input: &mut Vec<u8>, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, value: &WithExample) {
    match value.key {
        1 => input.extend_from_slice(b"read"),
        2 => input.extend_from_slice(b"write"),
        _ => {},
    }

    input.extend_from_slice(&value.value.to_be_bytes()[1..])
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jppe(decode_with="custom_with_decode", encode_with="custom_with_encode")]
pub struct WithExample {
    pub key: u8,
    pub value: u32,
}


#[test]
fn test_modifier_with() {
    let input = b"read\x00\x00\x01\x02";
    let (_, value) = WithExample::decode(input, None, None).unwrap();
    assert_eq!(value, WithExample { key: 1, value: 1 });

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"read\x00\x00\x01");
}