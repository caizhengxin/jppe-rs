use crate::{ByteEncode, BorrowByteEncode, ByteOrder, ContainerAttrModifiers, FieldAttrModifiers};


macro_rules! encode_float {
    ($t:ident, $as_t:ident) => {
        impl ByteEncode for $t {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
            where 
                Self: Sized
            {
                match crate::get_byteorder(cattr, fattr) {
                    ByteOrder::Be => input.extend(self.to_be_bytes()),
                    ByteOrder::Le => input.extend(self.to_le_bytes()),
                }
            }
        }        


        impl BorrowByteEncode for $t {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
            where 
                Self: Sized
            {
                match crate::get_byteorder(cattr, fattr) {
                    ByteOrder::Be => input.extend(self.to_be_bytes()),
                    ByteOrder::Le => input.extend(self.to_le_bytes()),
                }
            }
        }
    };

    () => {
        encode_float!(f32, u32);
        encode_float!(f64, u64);
    }
}


encode_float!();


#[cfg(test)]
mod tests {
    use crate::BorrowByteEncode;

    #[test]
    fn test_encode_f32() {
        let mut buf = vec![];
        let value = 0.1_f32;
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [61, 204, 204, 205]);

        let mut buf = vec![];
        let value = 0.1_f32;
        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        value.encode(&mut buf, Some(&cattr), None);
        assert_eq!(buf, [205, 204, 204, 61]);

        let mut buf = vec![];
        let value = 0.1_f32;
        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        let fattr = crate::FieldAttrModifiers {
            byteorder: Some(crate::ByteOrder::Be),
            ..Default::default()
        };
        value.encode(&mut buf, Some(&cattr), Some(&fattr));
        assert_eq!(buf, [61, 204, 204, 205]);
    }

    #[test]
    fn test_encode_f64() {
        let mut buf = vec![];
        let value = 0.1_f64;
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [63, 185, 153, 153, 153, 153, 153, 154]);

        let mut buf = vec![];
        let value = 0.1_f64;
        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        value.encode(&mut buf, Some(&cattr), None);
        assert_eq!(buf, [154, 153, 153, 153, 153, 153, 185, 63]);

        let mut buf = vec![];
        let value = 0.1_f64;
        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        let fattr = crate::FieldAttrModifiers {
            byteorder: Some(crate::ByteOrder::Be),
            ..Default::default()
        };
        value.encode(&mut buf, Some(&cattr), Some(&fattr));
        assert_eq!(buf, [63, 185, 153, 153, 153, 153, 153, 154]);
    }
}