use crate::fields::{HexBytes, HexString};
use crate::InputTrait;


#[inline]
fn decode_hex<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], &'da [u8]> {
    let mut input = input;
    let length;

    if let Some(fr) = fattr {
        if let Some(length_tmp) = fr.length {
            length = length_tmp;
        }
        else if let Some(byte_count) = fr.byte_count {
            (input, length) = input.to_bits_usize(crate::get_byteorder(cattr, fattr), byte_count as u8)?;
        }
        else { 
            (input, length) = input.to_be_bits_usize(1)?;
        }
    }
    else {
        (input, length) = input.to_be_bits_usize(1)?;
    };

    let (input, value) = input.input_take(length)?;

    Ok((input, value))
}


impl crate::ByteDecode for HexString {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = decode_hex(input, cattr, fattr)?;
        Ok((input, HexString::new(value)))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HexString {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HexBytes<'de> {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = decode_hex(input, cattr, fattr)?;
        Ok((input, HexBytes::new(value)))
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ByteDecode;
    use crate::FieldAttrModifiers;
    use super::*;

    #[test]
    fn test_decode_hex() {
        let (input, value) = HexString::decode(b"\x03\x00\x01\x02", None, None).unwrap();
        assert_eq!(value, HexString::from_str("000102").unwrap());
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { length: Some(3), ..Default::default() };
        let (input, value) = HexString::decode(b"\x00\x01\x02", None, Some(&fattr)).unwrap();
        assert_eq!(value, HexString::from_str("000102").unwrap());
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { byte_count: Some(1), ..Default::default() };
        let (input, value) = HexString::decode(b"\x03\x00\x01\x02", None, Some(&fattr)).unwrap();
        assert_eq!(value, HexString::from_str("000102").unwrap());
        assert_eq!(input.is_empty(), true);
    }
}