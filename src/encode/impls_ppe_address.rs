use crate::std::*;
use crate::fields::PpeAddress;


impl crate::ByteEncode for PpeAddress {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {        
        match self {
            Self::V4(v) => v.encode(input, cattr, fattr),
            Self::V6(v) => v.encode(input, cattr, fattr),
            Self::Mac(v) => v.encode(input, cattr, fattr),
            Self::Usize(v) => v.encode(input, cattr, fattr),
        }
    }
}


impl crate::BorrowByteEncode for PpeAddress {
    #[inline]
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        match self {
            Self::V4(v) => v.encode(input, cattr, fattr),
            Self::V6(v) => v.encode(input, cattr, fattr),
            Self::Mac(v) => v.encode(input, cattr, fattr),
            Self::Usize(v) => v.encode(input, cattr, fattr),
        }
    }
}
