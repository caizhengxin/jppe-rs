#[allow(unused_imports)]
use crate::{FieldAttrModifiers, ContainerAttrModifiers, ByteDecode, BorrowByteDecode};
use crate::{parser::*, InputTrait};
use crate::get_byteorder;


// #[inline]
pub fn parse_bytes<'a, 'b>(input: &'a [u8], cattr: Option<&'b ContainerAttrModifiers>, fattr: Option<&'b FieldAttrModifiers>) -> JResult<&'a [u8], &'a [u8]> {
    let mut value_tmp = None;
    let mut input = input;
    let mut input_tmp = input;

    if let Some(fr) = fattr {
        if let Some(key) = &fr.key {
            (input, _) = input.find_subsequence(key, false)?;
        }

        if let Some(splits) = fr.split {
            (input, _) = input.find_subsequence(splits, false)?;
        }

        // if fr.linend {
        //     let (input, value) = input.find_subsequences(&[b"\r\n", b"\n", b"\x00"], false)?;

        //     value_tmp = Some(value);
        //     input_tmp = input;
        // }
        if let Some(linend_value) = fr.linend_value {
            let (input, value) = input.find_subsequence(linend_value, false)?;

            // value_tmp = Some(value);
            // input_tmp = input;

            return Ok((input, value));
        }
        else if let Some(length) = fr.length {
            let (input, value) = input_take(input, length)?;

            value_tmp = Some(value);
            input_tmp = input;
        }
        else if let Some(byte_count) = fr.byte_count {
            let (input, length) = input.to_bits(get_byteorder(cattr, fattr), byte_count as u8)?;
            let (input, value) = input.input_take(length as usize)?;

            value_tmp = Some(value);
            input_tmp = input;
        }
        // else {
        //     let (input, length) = input.to_be_bits_usize(1)?;
        //     let (input, value) = input.input_take(length)?;
        //     input_tmp = input;
        //     value_tmp = Some(value);
        // }
    }
    // else {
    //     let (input, length) = input.to_be_bits_usize(1)?;
    //     let (input, value) = input.input_take(length)?;
    //     input_tmp = input;
    //     value_tmp = Some(value);
    // }

    if let Some(value) = value_tmp {
        return Ok((input_tmp, value));
    }

    Err(make_error(input, ErrorKind::Fail { offset: input.len() }))
}


impl<'de> BorrowByteDecode<'de> for &'de [u8] {
    fn decode<'da: 'de>(input: &'da [u8], cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
{
        let mut value_tmp = None;
        let mut input_tmp = input;

        match parse_bytes(input, cattr, fattr) {
            Ok((input, value)) => {
                input_tmp = input;
                value_tmp = Some(value);    
            },
            Err(e) => {
                if let crate::ErrorKind::Fail { .. } = e.code {
                    input_tmp = &[];
                    value_tmp = Some(input);
                }
            }
        }
    
        if let Some(value) = value_tmp {
            return Ok((input_tmp, value));
        }

        Err(make_error(input_tmp, ErrorKind::InvalidByteLength { offset: input_tmp.len() }))
    }
}
