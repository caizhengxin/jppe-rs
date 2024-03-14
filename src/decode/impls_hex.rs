use crate::{fields::HexString, InputTrait};


impl crate::ByteDecode for HexString {
    fn decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let length = if let Some(fr) = fattr && let Some(length) = fr.length {length} else {input.len()};
        let (input, value) = input.input_take(length)?;

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
