use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use crate::fields::MacAddress;
use crate::{ByteOrder, get_byteorder};


impl crate::ByteEncode for MacAddress {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {        
        input.extend_from_slice(self);
    }
}


impl crate::BorrowByteEncode for MacAddress {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {
        input.extend_from_slice(self);
    }
}


impl crate::ByteEncode for Ipv4Addr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {        
        let byteorder = get_byteorder(cattr, fattr);

        if byteorder == ByteOrder::Be {
            input.extend(self.to_bits().to_be_bytes());            
        }
        else {
            input.extend(self.to_bits().to_le_bytes());
        }
    }
}


impl crate::BorrowByteEncode for Ipv4Addr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        let byteorder = get_byteorder(cattr, fattr);

        if byteorder == ByteOrder::Be {
            input.extend(self.to_bits().to_be_bytes());            
        }
        else {
            input.extend(self.to_bits().to_le_bytes());
        }
    }
}


impl crate::ByteEncode for Ipv6Addr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {        
        let byteorder = get_byteorder(cattr, fattr);

        if byteorder == ByteOrder::Be {
            input.extend(self.to_bits().to_be_bytes());            
        }
        else {
            input.extend(self.to_bits().to_le_bytes());
        }
    }
}


impl crate::BorrowByteEncode for Ipv6Addr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        let byteorder = get_byteorder(cattr, fattr);

        if byteorder == ByteOrder::Be {
            input.extend(self.to_bits().to_be_bytes());            
        }
        else {
            input.extend(self.to_bits().to_le_bytes());
        }
    }
}


impl crate::ByteEncode for IpAddr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {        
        match self {
            Self::V4(v) => v.encode(input, cattr, fattr),
            Self::V6(v) => v.encode(input, cattr, fattr),
        }
    }
}


impl crate::BorrowByteEncode for IpAddr {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        match self {
            Self::V4(v) => v.encode(input, cattr, fattr),
            Self::V6(v) => v.encode(input, cattr, fattr),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{encode::ByteEncode, FieldAttrModifiers};
    use super::*;

    #[test]
    fn test_encode_mac_address() {
        let mut buf = vec![];
        let value = MacAddress::from_bits(0x1234567890ff);
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x12, 0x34, 0x56, 0x78, 0x90, 0xff]);
        assert_eq!(value.to_bits(), 0x1234567890ff);
    }

    #[test]
    fn test_encode_ip_address() {
        let mut buf = vec![];
        let value = Ipv4Addr::from_bits(0x12345678);
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x12, 0x34, 0x56, 0x78]);

        let mut buf = vec![];
        let value = Ipv4Addr::from_bits(0x12345678);
        let fattr = FieldAttrModifiers { byteorder: Some(ByteOrder::Le), ..Default::default() };
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, [0x78, 0x56, 0x34, 0x12]);

        let mut buf = vec![];
        let value = IpAddr::V4(Ipv4Addr::from_bits(0x12345678));
        let fattr = FieldAttrModifiers { byteorder: Some(ByteOrder::Le), ..Default::default() };
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, [0x78, 0x56, 0x34, 0x12]);
    }
}