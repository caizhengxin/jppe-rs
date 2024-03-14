use crate::fields::{HexString, HexBytes};


impl crate::ByteEncode for HexString {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {        
        input.extend_from_slice(self);
    }
}


impl crate::BorrowByteEncode for HexString {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {
        input.extend_from_slice(self);
    }
}


impl<'da> crate::BorrowByteEncode for HexBytes<'da> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {
        input.extend_from_slice(self);
    }
}