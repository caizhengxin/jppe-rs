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
        let value_tmp;
        let input_tmp;
    
        if let Ok((input, value)) = parse_bytes(input, cattr, fattr) {
            input_tmp = input;
            value_tmp = Some(value);
        }
        else {
            let (input, length) = input.to_be_bits_usize(1)?;
            let (input, value) = input.input_take(length)?;
            input_tmp = input;
            value_tmp = Some(value);
        }
    
        if let Some(value) = value_tmp {
            if let Ok(v) = core::str::from_utf8(value) {
                return Ok((input_tmp, v));
            }    
        }
    
        Err(make_error(input_tmp, ErrorKind::InvalidByteLength { offset: input_tmp.len() }))    
    }
}        
