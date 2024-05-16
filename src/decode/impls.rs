use crate::std::*;
use crate::{get_byteorder, BorrowByteDecode, ByteDecode, InputTrait};
use crate::{FieldAttrModifiers, ContainerAttrModifiers};
#[allow(unused_imports)]
use crate::{JResult, make_error, ErrorKind};
#[allow(unused_imports)]
use crate::{ByteOrder, input_take};


impl ByteDecode for bool {
    #[inline]
    fn decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = input_take(input, 1)?;

        Ok((input, value[0] != 0))
    }
}


impl<'de> BorrowByteDecode<'de> for bool {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = input_take(input, 1)?;

        Ok((input, value[0] != 0))        
    }
}


impl<T: ByteDecode> ByteDecode for Option<T> {
    #[inline]
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        if let Some(fattr) = fattr {
            if let Some(length) = fattr.length {
                if length == 0 {
                    return Ok((input, None));
                }
            }
        }

        if let Ok((input, value)) = T::decode(input, cattr, fattr) {
            return Ok((input, Some(value)));
        }

        Ok((input, None))
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Option<T> {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        if let Some(fattr) = fattr {
            if let Some(length) = fattr.length {
                if length == 0 {
                    return Ok((input, None));
                }
            }
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

        if let Some(fattr_tmp) = fattr {
            let count = if let Some(count) = fattr_tmp.count {
                count
            }
            else if let Some(byte_count) = fattr_tmp.byte_count {
                let (input_tmp, count) = input.to_bits_usize(get_byteorder(cattr, fattr), byte_count as u8)?;
                input = input_tmp;
                count
            }
            else if let Some(length) = fattr_tmp.length {
                length
            }
            else if let Some(count) = fattr_tmp.try_count {
                count
            }
            else {
                let (input_tmp, count) = input.to_be_bits_usize(1)?;
                input = input_tmp;
                count
            };

            if fattr_tmp.try_count.is_some() {
                for _i in 0..count {
                    if let Ok((input_tmp, value)) = T::decode(input, cattr, fattr) {
                        input = input_tmp;
                        value_list.push(value);    
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                for _i in 0..count {
                    (input, value) = T::decode(input, cattr, fattr)?;
                    value_list.push(value);    
                }      
            }
        }
        else {
            let (input_tmp, count) = input.to_be_bits_usize(1)?;
            input = input_tmp;

            for _i in 0..count {
                (input, value) = T::decode(input, cattr, fattr)?;
                value_list.push(value);
            } 
        }

        Ok((input, value_list))
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Vec<T> {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let mut input = input;
        let mut value;
        let mut value_list = Vec::new();

        if let Some(fattr_tmp) = fattr {
            let count = if let Some(count) = fattr_tmp.count {
                count
            }
            else if let Some(byte_count) = fattr_tmp.byte_count {
                let (input_tmp, count) = input.to_bits_usize(get_byteorder(cattr, fattr), byte_count as u8)?;
                input = input_tmp;
                count
            }
            else if let Some(length) = fattr_tmp.length {
                length
            } 
            else if let Some(count) = fattr_tmp.try_count {
                count
            }
            else {
                let (input_tmp, count) = input.to_be_bits_usize(1)?;
                input = input_tmp;
                count
            };

            if fattr_tmp.try_count.is_some() {
                for _i in 0..count {
                    if let Ok((input_tmp, value)) = T::decode(input, cattr, fattr) {
                        input = input_tmp;
                        value_list.push(value);    
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                for _i in 0..count {
                    (input, value) = T::decode(input, cattr, fattr)?;
                    value_list.push(value);    
                }      
            }
        }
        else {
            let (input_tmp, count) = input.to_be_bits_usize(1)?;
            input = input_tmp;

            for _i in 0..count {
                (input, value) = T::decode(input, cattr, fattr)?;
                value_list.push(value);
            }
        }

        Ok((input, value_list))
    }
}


impl<T> ByteDecode for PhantomData<T> {
    fn decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    {
        Ok((input, PhantomData))
    }
}


impl<'de, T> BorrowByteDecode<'de> for PhantomData<T> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], _cattr: Option<&'db ContainerAttrModifiers>, _fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    {
        Ok((input, PhantomData))
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
        let fattr = FieldAttrModifiers { count: Some(1), ..Default::default() };
        let (input, value) = <Vec<u16>>::decode(&[0x00, 0x01, 0x02], None, Some(&fattr)).unwrap();
        assert_eq!(value, [0x01]);
        assert_eq!(input, [0x02]);

        let fattr = FieldAttrModifiers { byte_count: Some(1), ..Default::default() };
        let (input, value) = <Vec<u16>>::decode(&[0x01, 0x00, 0x01, 0x02], None, Some(&fattr)).unwrap();
        assert_eq!(value, [0x01]);
        assert_eq!(input, [0x02]);

        let (input, value) = <Vec<u16>>::decode(&[0x02, 0x00, 0x01, 0x00, 0x02, 0x03], None, None).unwrap();
        assert_eq!(value, [0x0001, 0x0002]);
        assert_eq!(input, [0x03]);

        let fattr = FieldAttrModifiers { count: Some(2), ..Default::default() };
        assert_eq!(<Vec<u16>>::decode(&[0x00, 0x01, 0x02], None, Some(&fattr)).is_err(), true);
    }
}