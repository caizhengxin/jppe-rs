use super::ByteDecode;


impl<T, const N: usize> crate::ByteDecode for [T; N]
where
    T: std::marker::Copy + ByteDecode + Default,
{
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let mut array: [T; N] = [T::default(); N];
        let mut input = input;
        let mut value;

        for _array in array.iter_mut() {
            (input, value) = T::decode(input, cattr, fattr)?;

            *_array = value;
        }

        Ok((input, array))
    }
}


impl<'de, T, const N: usize> crate::BorrowByteDecode<'de> for [T; N]
where
    T: std::marker::Copy + crate::BorrowByteDecode<'de> + Default,
{
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
        where 
            Self: Sized
    {
        let mut array: [T; N] = [T::default(); N];
        let mut input = input;
        let mut value;

        for _array in array.iter_mut() {
            (input, value) = T::decode(input, cattr, fattr)?;

            *_array = value;
        }

        Ok((input, array))
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
