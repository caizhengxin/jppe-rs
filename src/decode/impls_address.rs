use crate::fields::MacAddress;


impl crate::ByteDecode for MacAddress {
    fn decode<'da, 'db>(input: &'da [u8], _cattr: Option<&'db crate::ContainerAttrModifiers>, _fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = crate::input_take(input, 6)?;

        if let Ok(value) = <[u8; 6]>::try_from(value) {
            return Ok((input, MacAddress::new(value)));
        }

        Err(crate::make_error(input, crate::ErrorKind::InvalidByteLength { offset: input.len() }))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for MacAddress {
    fn decode<'da: 'de, 'db>(input: &'da [u8], _cattr: Option<&'db crate::ContainerAttrModifiers>, _fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, value) = crate::input_take(input, 6)?;

        if let Ok(value) = <[u8; 6]>::try_from(value) {
            return Ok((input, MacAddress::new(value)));
        }

        Err(crate::make_error(input, crate::ErrorKind::InvalidByteLength { offset: input.len() }))   
    }
}
