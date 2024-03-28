use crate::{BorrowByteDecode, ByteDecode, InputTrait};
use crate::{FieldAttrModifiers, ContainerAttrModifiers};
use crate::decode::impls_bytes::parse_bytes;
use crate::parser::errors::{ErrorKind, JResult, make_error};
// use crate::parse_subsequence;


#[inline]
fn parse_string<'a>(input: &'a [u8], cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<&'a [u8], String> {
    let value_tmp;
    let input_tmp;

    if let Ok((input, value)) = parse_bytes(input, cattr, fattr) {
        input_tmp = input;
        value_tmp = Some(value);
    }
    // else if let Ok((input, value)) = parse_subsequence(input, b"\n", false) {
    //     input_tmp = input;
    //     value_tmp = Some(value);
    // }
    else {
        let (input, length) = input.to_be_bits_usize(1)?;
        let (input, value) = input.input_take(length)?;
        input_tmp = input;
        value_tmp = Some(value);
    }

    if let Some(value) = value_tmp {
        // let value = value.trim_ascii_end().to_vec().extract_if(|v| *v != 0).collect::<Vec<_>>();
        // std::str::from_utf8(value).unwrap().to_string()
        // match String::from_utf8(value) {
        //     Ok(v) => return Ok((input_tmp, v)),
        //     Err(_e) => return Err(make_error(input_tmp, ErrorKind::Fail { offset: input_tmp.len() })),
        // }

        match std::str::from_utf8(value) {
            Ok(v) => return Ok((input_tmp, v.to_string())),
            Err(_e) => return Err(make_error(input_tmp, ErrorKind::Fail { offset: input_tmp.len() })),
        }
    }

    Err(make_error(input_tmp, ErrorKind::InvalidByteLength { offset: input_tmp.len() }))
}


impl ByteDecode for String {
    fn decode<'da>(input: &'da [u8], cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let (input, value) = parse_string(input, cattr, fattr)?;

        Ok((input, value))        
    }
}        


impl<'de> BorrowByteDecode<'de> for String {
    fn decode<'da: 'de>(input: &'da [u8], cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let (input, value) = parse_string(input, cattr, fattr)?;

        Ok((input, value))        
    }
}


impl<'de> BorrowByteDecode<'de> for &'de str {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let (input, value) = <&[u8]>::decode(input, cattr, fattr)?;

        if let Ok(v) = core::str::from_utf8(value) {
            return Ok((input, v));
        }

        Err(make_error(input, ErrorKind::Fail { offset: input.len() }))
    }
}        


#[cfg(test)]
mod tests {
    use crate::{decode::ByteDecode, FieldAttrModifiers};

    #[test]
    fn test_decode_string() {
        let (input, value) = String::decode(b"\x0212", None, None).unwrap();
        assert_eq!(value, "12".to_string());
        assert_eq!(input.is_empty(), true);

        assert_eq!(String::decode(b"12\x00", None, None).is_err(), true);

        let fattr = FieldAttrModifiers { linend: true, ..Default::default() };
        let (input, value) = String::decode(b"12\x00", None, Some(&fattr)).unwrap();
        assert_eq!(value, "12".to_string());
        assert_eq!(input.is_empty(), true);

        let (input, value) = String::decode(b"12\r\n", None, Some(&fattr)).unwrap();
        assert_eq!(value, "12".to_string());
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { linend_value: Some(vec![vec![b'3', b'4']]), ..Default::default() };
        let (input, value) = String::decode(b"1234", None, Some(&fattr)).unwrap();
        assert_eq!(value, "12".to_string());
        assert_eq!(input.is_empty(), true);

        // length
        let fattr = FieldAttrModifiers { length: Some(4), ..Default::default() };
        let (input, value) = String::decode(b"1234", None, Some(&fattr)).unwrap();
        assert_eq!(value, "1234".to_string());
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { length: Some(3), ..Default::default() };
        let (input, value) = String::decode(b"1234", None, Some(&fattr)).unwrap();
        assert_eq!(value, "123".to_string());
        assert_eq!(input, b"4");

        let fattr = FieldAttrModifiers { length: Some(5), ..Default::default() };
        assert_eq!(String::decode(b"1234", None, Some(&fattr)).is_err(), true);

        // key
        let fattr = FieldAttrModifiers { key: Some(b"Header: ".to_vec()), linend: true, ..Default::default() };
        let (input, value) = String::decode(b"Header: 123\r\n", None, Some(&fattr)).unwrap();
        assert_eq!(value, "123");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { key: Some(b"Header".to_vec()), split: Some(vec![b": ".to_vec()]), linend: true, ..Default::default() };
        let (input, value) = String::decode(b"Header: 123\r\n", None, Some(&fattr)).unwrap();
        assert_eq!(value, "123");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { byte_count: Some(2), ..Default::default() };
        let (input, value) = String::decode(b"\x00\x02\x31\x32", None, Some(&fattr)).unwrap();
        assert_eq!(value, "12");
        assert_eq!(input.is_empty(), true);
    }
}