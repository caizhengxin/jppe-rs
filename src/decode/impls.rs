use crate::{BorrowByteDecode, ByteDecode};
use crate::{FieldAttrModifiers, ContainerAttrModifiers};
#[allow(unused_imports)]
use crate::{JResult, make_error, ErrorKind};
#[allow(unused_imports)]
use crate::{ByteOrder, input_take};


impl ByteDecode for bool {
    fn decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let (input, value) = input_take(input, 1)?;

        Ok((input, value[0] != 0))
    }
}


impl<'de> BorrowByteDecode<'de> for bool {
    fn decode<'da: 'de, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let (input, value) = input_take(input, 1)?;

        Ok((input, value[0] != 0))        
    }
}


impl<T: ByteDecode> ByteDecode for Option<T> {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        if let Some(fattr) = fattr && let Some(length) = fattr.length && length == 0 {
            return Ok((input, None));
        }

        if let Ok((input, value)) = T::decode(input, cattr, fattr) {
            return Ok((input, Some(value)));
        }

        Ok((input, None))
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Option<T> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        if let Some(fattr) = fattr && let Some(length) = fattr.length && length == 0 {
            return Ok((input, None));
        }

        if let Ok((input, value)) = T::decode(input, cattr, fattr) {
            return Ok((input, Some(value)));
        }

        Ok((input, None))
    }
}


impl<T: ByteDecode> ByteDecode for Vec<T> {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let mut input = input;
        let mut value;
        let mut value_list = Vec::new();

        if let Some(fattr_tmp) = fattr && let Some(length) = fattr_tmp.length {
            let mut fattr_tmp = fattr_tmp.clone();
            fattr_tmp.length = None;

            for _i in 0..length {
                (input, value) = T::decode(input, cattr, Some(&fattr_tmp))?;
                value_list.push(value);
            }    
        }

        Ok((input, value_list))
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Vec<T> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let mut input = input;
        let mut value;
        let mut value_list = Vec::new();

        if let Some(fattr_tmp) = fattr && let Some(length) = fattr_tmp.length {
            let mut fattr_tmp = fattr_tmp.clone();
            fattr_tmp.length = None;

            for _i in 0..length {
                (input, value) = T::decode(input, cattr, Some(&fattr_tmp))?;
    
                value_list.push(value);
            }    
        }

        Ok((input, value_list))
    }
}


#[cfg(test)]
mod tests {
    use crate::{decode::ByteDecode, FieldAttrModifiers};

    #[test]
    fn test_decode_bool() {
        let (input, value) = bool::decode(&[0x00, 0x01], None, None).unwrap();
        assert_eq!(value, false);
        assert_eq!(input, [0x01]);

        let (input, value) = bool::decode(&[0x01, 0x02], None, None).unwrap();
        assert_eq!(value, true);
        assert_eq!(input, [0x02]);

        let (input, value) = bool::decode(&[0x04, 0x02], None, None).unwrap();
        assert_eq!(value, true);
        assert_eq!(input, [0x02]);
    }

    #[test]
    fn test_decode_option() {
        let (input, value) = <Option<bool>>::decode(&[0x00, 0x01], None, None).unwrap();
        assert_eq!(value, Some(false));
        assert_eq!(input, [0x01]);

        let (input, value) = <Option<bool>>::decode(&[], None, None).unwrap();
        assert_eq!(value, None);
        assert_eq!(input.is_empty(), true);

        let fattr = FieldAttrModifiers { length: Some(0), ..Default::default() };
        let (input, value) = <Option<bool>>::decode(&[0x00, 0x01], None, Some(&fattr)).unwrap();
        assert_eq!(value, None);
        assert_eq!(input, [0x00, 0x01]);
    }

    #[test]
    fn test_decode_vec() {
        let fattr = FieldAttrModifiers { length: Some(1), ..Default::default() };
        let (input, value) = <Vec<u16>>::decode(&[0x00, 0x01, 0x02], None, Some(&fattr)).unwrap();
        assert_eq!(value, [0x01]);
        assert_eq!(input, [0x02]);

        let fattr = FieldAttrModifiers { length: Some(2), ..Default::default() };
        assert_eq!(<Vec<u16>>::decode(&[0x00, 0x01, 0x02], None, Some(&fattr)).is_err(), true);
    }
}