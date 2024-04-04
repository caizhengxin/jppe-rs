use std::mem::MaybeUninit;


impl<T, const N: usize> crate::ByteDecode for [T; N]
where
    T: crate::ByteDecode,
{
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut array = MaybeUninit::<[T; N]>::uninit();
        let mut input = input;
        let mut value;

        let ptr = unsafe { &mut *array.as_mut_ptr() };

        for i in 0..N {
            (input, value) = T::decode(input, cattr, fattr)?;
            ptr[i] = value;
        }

        Ok((input, unsafe { array.assume_init() }))
    }
}


impl<'de, T, const N: usize> crate::BorrowByteDecode<'de> for [T; N]
where
    T: crate::BorrowByteDecode<'de>,
{
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut array = MaybeUninit::<[T; N]>::uninit();
        let mut input = input;
        let mut value;

        let ptr = unsafe { &mut *array.as_mut_ptr() };

        for i in 0..N {
            (input, value) = T::decode(input, cattr, fattr)?;
            ptr[i] = value;
        }

        Ok((input, unsafe { array.assume_init() }))
    }
}


#[cfg(test)]
mod tests {
    use crate::decode::ByteDecode;

    #[test]
    fn test_decode_array() {
        let (input, value) = <[u8;2]>::decode(&[0x01, 0x02, 0x03], None, None).unwrap();
        assert_eq!(value, [0x01, 0x02]);
        assert_eq!(input, [0x03]);

        let (input, value) = <[u16;2]>::decode(&[0x00, 0x01, 0x00, 0x02, 0x03], None, None).unwrap();
        assert_eq!(value, [0x01, 0x02]);
        assert_eq!(input, [0x03]);
    }
}
