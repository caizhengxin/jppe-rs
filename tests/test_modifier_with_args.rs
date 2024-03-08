#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode, JResult, InputTrait, ContainerAttrModifiers, FieldAttrModifiers};
use jppe_derive::{ByteEncode, ByteDecode};


/// This is just a demonstration, so go straight back.
fn custom_with_decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>, key: u8) -> JResult<&'da [u8], u32> {
    assert_eq!(key, 2);

    let (input, value) = input.to_be_bits(key)?;

    Ok((input, value as u32))
}


fn custom_with_encode(input: &mut Vec<u8>, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, value: &u32, key: u8) {
    let value_bytes = value.to_be_bytes();

    input.extend_from_slice(&value_bytes[value_bytes.len() - key as usize..])
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct WithArgsExample {
    pub key: u8,
    #[jppe(decode_with="custom_with_decode", encode_with="custom_with_encode", with_args="key")]
    // #[jppe(length=3)]
    pub value: u32,
}


#[test]
fn test_modifier_with_args() {
    let input = b"\x02\x00\x01";
    let (_, value) = WithArgsExample::decode(input, None, None).unwrap();
    assert_eq!(value, WithArgsExample { key: 2, value: 1 });

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\x02\x00\x01");
}