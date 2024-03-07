use crate::fields::MacAddress;


impl crate::ByteEncode for MacAddress {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {        
        input.extend_from_slice(self);
    }
}


impl crate::BorrowByteEncode for MacAddress {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) {
        input.extend_from_slice(self);
    }
}
