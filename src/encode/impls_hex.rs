use crate::std::*;
use crate::fields::{HexString, HexBytes};
use crate::{get_byteorder, int_to_vec};


#[inline]
fn decode_hex(input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>, length: usize) {
    if let Some(fr) = fattr {
        if let Some(byte_count) = fr.byte_count {
            input.extend(int_to_vec(length, byte_count, &get_byteorder(cattr, fattr)));
        }
    }
    else {
        input.extend(int_to_vec(length, 1, &crate::ByteOrder::Be));
    }
}


impl crate::ByteEncode for HexString {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {        
        decode_hex(input, cattr, fattr, self.len());
        input.extend_from_slice(self);
    }
}


impl crate::BorrowByteEncode for HexString {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        decode_hex(input, cattr, fattr, self.len());
        input.extend_from_slice(self);
    }
}


impl<'da> crate::BorrowByteEncode for HexBytes<'da> {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
        decode_hex(input, cattr, fattr, self.len());
        input.extend_from_slice(self);
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::ByteEncode;
    use crate::FieldAttrModifiers;
    use super::*;

    #[test]
    fn test_encode_hex() {
        let value = HexString::from_str("000102").unwrap();
        let mut buf = Vec::new();
        value.encode(&mut buf, None, None);
        assert_eq!(buf, b"\x03\x00\x01\x02");

        let fattr = FieldAttrModifiers { length: Some(3), ..Default::default() };
        let value = HexString::from_str("000102").unwrap();
        let mut buf = Vec::new();
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"\x00\x01\x02");

        let fattr = FieldAttrModifiers { byte_count: Some(1), ..Default::default() };
        let value = HexString::from_str("000102").unwrap();
        let mut buf = Vec::new();
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"\x03\x00\x01\x02");
    }
}