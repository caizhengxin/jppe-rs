use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use crate::fields::MacAddress;


impl crate::ByteDecode for MacAddress {
    #[inline]
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
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}


impl crate::ByteDecode for Ipv4Addr {
    #[inline]
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let byteorder = crate::get_byteorder(cattr, fattr);
        let (input, value) = crate::parse_u128(input, &byteorder, 4)?;
        
        Ok((input, Ipv4Addr::from_bits(value as u32)))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for Ipv4Addr {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}


impl crate::ByteDecode for Ipv6Addr {
    #[inline]
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let byteorder = crate::get_byteorder(cattr, fattr);
        let (input, value) = crate::parse_u128(input, &byteorder, 4)?;
        
        Ok((input, Ipv6Addr::from_bits(value)))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for Ipv6Addr {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}


impl crate::ByteDecode for IpAddr {
    #[inline]
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        if let Some(fattr_var) = fattr && let Some(length) = fattr_var.length && length == 6 {
            let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

            return Ok((input, IpAddr::V6(addr)));    
        }
        
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, IpAddr::V4(addr)))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for IpAddr {
    #[inline]
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let (input, addr) = crate::ByteDecode::decode(input, cattr, fattr)?;

        Ok((input, addr))
    }
}


#[cfg(test)]
mod test {
    use crate::{decode::ByteDecode, FieldAttrModifiers};
    use super::*;

    #[test]
    fn test_decode_mac_address() {
        let input = b"\xff\xff\xff\xff\xff\xff\x00\x00";
        let (input, value) = MacAddress::decode(input, None, None).unwrap();
        assert_eq!(value, MacAddress::from_bits(0xffffffffffff));
        assert_eq!(value.is_broadcast(), true);
        assert_eq!(input, &[0x00, 0x00]);    
    }

    #[test]
    fn test_decode_ip_address() {
        let input = b"\x12\x34\x56\x78";
        let (input, value) = Ipv4Addr::decode(input, None, None).unwrap();
        assert_eq!(value, Ipv4Addr::from_bits(0x12345678));
        assert_eq!(input.is_empty(), true);

        let input = b"\x12\x34\x56\x78";
        let fattr = FieldAttrModifiers { byteorder: Some(crate::ByteOrder::Le), ..Default::default() };
        let (input, value) = Ipv4Addr::decode(input, None, Some(&fattr)).unwrap();
        assert_eq!(value, Ipv4Addr::from_bits(0x78563412));
        assert_eq!(input.is_empty(), true);

        let input = b"\x12\x34\x56\x78";
        let fattr = FieldAttrModifiers { byteorder: Some(crate::ByteOrder::Le), ..Default::default() };
        let (input, value) = IpAddr::decode(input, None, Some(&fattr)).unwrap();
        assert_eq!(value, Ipv4Addr::from_bits(0x78563412));
        assert_eq!(input.is_empty(), true);
    }
}