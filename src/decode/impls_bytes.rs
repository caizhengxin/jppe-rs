#[allow(unused_imports)]
use crate::{FieldAttrModifiers, ContainerAttrModifiers, ByteDecode, BorrowByteDecode};
use crate::{parser::*, InputTrait};


pub fn parse_bytes<'a, 'b>(input: &'a [u8], _cattr: Option<&'b ContainerAttrModifiers>, fattr: Option<&'b FieldAttrModifiers>) -> JResult<&'a [u8], &'a [u8]> {
    let mut value_tmp = None;
    let mut input = input;
    let mut input_tmp = input;

    if let Some(fattr) = fattr {
        if let Some(key) = &fattr.key {
            (input, _) = input.find_subsequence(key, false)?;
        }

        if let Some(splits) = &fattr.split {
            (input, _) = input.find_subsequences2(splits, false)?;
        }

        if fattr.linend {
            let (input, value) = input.find_subsequences(&[b"\r\n", b"\n", b"\x00"], false)?;

            value_tmp = Some(value);
            input_tmp = input;
        }
        else if let Some(linend_value_list) = &fattr.linend_value {
            let (input, value) = input.find_subsequences2(linend_value_list, false)?;

            value_tmp = Some(value);
            input_tmp = input;
        }
        else if let Some(length) = fattr.length {
            let (input, value) = input_take(input, length)?;

            value_tmp = Some(value);
            input_tmp = input;
        }
    }

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
                    let (input, value) = input_take(input, input.len())?;

                    input_tmp = input;
                    value_tmp = Some(value);        
                }
            }
        }
    
        if let Some(value) = value_tmp {
            return Ok((input_tmp, value));
        }

        Err(make_error(input_tmp, ErrorKind::InvalidByteLength { offset: input_tmp.len() }))
    }
}


#[cfg(test)]
mod tests {
    use crate::{decode::BorrowByteDecode, FieldAttrModifiers};

    #[test]
    fn test_decode_bytes() {
        let (input, value) = <&[u8]>::decode(b"12\n", None, None).unwrap();
        // println!("{:?} {:?}", value, input);
        assert_eq!(value, b"12\n");
        assert_eq!(input.is_empty(), true);

        let (input, value) = <&[u8]>::decode(b"12\x03", None, None).unwrap();
        // println!("{:?} {:?}", value, input);
        assert_eq!(value, b"12\x03");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { linend: true, ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"12\x00", None, Some(&fattr)).unwrap();
        // println!("{:?} {:?}", value, input);
        assert_eq!(value, b"12");
        assert_eq!(input.is_empty(), true);

        let (input, value) = <&[u8]>::decode(b"12\r\n", None, Some(&fattr)).unwrap();
        // println!("{:?} {:?}", value, input);
        assert_eq!(value, b"12");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { linend_value: Some(vec![vec![b'3', b'4']]), ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"1234", None, Some(&fattr)).unwrap();
        // println!("{:?} {:?}", value, input);
        assert_eq!(value, b"12");
        assert_eq!(input.is_empty(), true);

        // length
        let fattr = FieldAttrModifiers { length: Some(4), ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"1234", None, Some(&fattr)).unwrap();
        assert_eq!(value, b"1234");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { length: Some(3), ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"1234", None, Some(&fattr)).unwrap();
        assert_eq!(value, b"123");
        assert_eq!(input, b"4");

        let fattr = FieldAttrModifiers { length: Some(5), ..Default::default() };
        assert_eq!(<&[u8]>::decode(b"1234", None, Some(&fattr)).is_err(), true);

        let (input, value) = <&[u8]>::decode(b"1234", None, None).unwrap();
        assert_eq!(value, b"1234");
        assert_eq!(input.is_empty(), true);

        // key
        let fattr = FieldAttrModifiers { key: Some(b"Header: ".to_vec()), linend: true, ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"Header: 123\r\n", None, Some(&fattr)).unwrap();
        assert_eq!(value, b"123");
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { key: Some(b"Header".to_vec()), split: Some(vec![b": ".to_vec()]), linend: true, ..Default::default() };
        let (input, value) = <&[u8]>::decode(b"Header: 123\r\n", None, Some(&fattr)).unwrap();
        assert_eq!(value, b"123");
        assert_eq!(input.is_empty(), true);
    }
}
