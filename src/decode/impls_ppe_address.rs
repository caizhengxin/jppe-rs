use crate::fields::PpeAddress;


impl crate::ByteDecode for PpeAddress {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        if let Some(fattr_var) = fattr && let Some(length) = fattr_var.length {
            match length {
                4 => {
                    let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;
                    return Ok((input, PpeAddress::V4(addr)))    
                },
                6 => {
                    let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;
                    return Ok((input, PpeAddress::Mac(addr)))    
                },
                16 => {
                    let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;
                    return Ok((input, PpeAddress::V6(addr)));    
                },
                _ => {
                    let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;
                    return Ok((input, PpeAddress::Usize(addr)));    
                },
            }
        }
        
        Err(crate::make_error(input, crate::ErrorKind::InvalidByteLength { offset: input.len() }))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for PpeAddress {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}