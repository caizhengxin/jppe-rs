use crate::std::*;
use std::collections::HashSet;


#[inline]
fn hashset_decode<'da, 'db, T>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], HashSet<T>>
where
    T: crate::ByteDecode + hash::Hash + cmp::Eq,
{
    let mut input = input;
    let mut hashset = HashSet::new();
    let mut value;

    if let Some(fr) = fattr {
        if let Some(count) = fr.count {
            for _ in 0..count {
                (input, value) = T::decode(input, cattr, fattr)?;
                hashset.insert(value);
            }    
        }
    }

    return Ok((input, hashset));
}


impl<T> crate::ByteDecode for HashSet<T>
where
    T: crate::ByteDecode + hash::Hash + cmp::Eq,
{
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        hashset_decode(input, cattr, fattr)
    }
}


impl<'de, T> crate::BorrowByteDecode<'de> for HashSet<T>
where
    T: crate::ByteDecode + hash::Hash + cmp::Eq,
{
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        hashset_decode(input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::{decode::ByteDecode, FieldAttrModifiers};
    use super::*;

    #[test]
    fn test_decode_hashset() {
        let (input, value) = HashSet::<u16>::decode(b"\x00\x01\x00\x02\x00\x03", None, None).unwrap();
        assert_eq!(value.is_empty(), true);
        assert_eq!(input, b"\x00\x01\x00\x02\x00\x03");

        let fattr = FieldAttrModifiers { count: Some(3), ..Default::default() };
        let (input, value) = HashSet::<u16>::decode(b"\x00\x01\x00\x02\x00\x03", None, Some(&fattr)).unwrap();
        assert_eq!(value, HashSet::from([1, 2, 3]));
        assert_eq!(input.is_empty(), true);

        let (input, value) = HashSet::<u16>::decode(b"\x00\x01\x00\x01\x00\x03", None, Some(&fattr)).unwrap();
        assert_eq!(value, HashSet::from([1, 3]));
        assert_eq!(input.is_empty(), true);
    }
}
