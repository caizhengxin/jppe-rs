use crate::ByteOrder;
use crate::{ContainerAttrModifiers, FieldAttrModifiers};
use super::{BorrowByteEncode, ByteEncode};


macro_rules! impls_int {
    ($type:ident, $byte:expr) => {
        impl ByteEncode for $type {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
                where 
                    Self: Sized
            {
                let mut byte_tmp = None;
                let mut byteorder = ByteOrder::Be;

                let mut value = *self;

                if let Some(cr) = cattr && let Some(byteorder_tmp) = cr.byteorder { byteorder = byteorder_tmp; }

                if let Some(fr) = fattr {
                    if let Some(length) = fr.length && length < $byte { byte_tmp = Some(length); }
                    if let Some(byteorder_tmp) = fr.byteorder { byteorder = byteorder_tmp; }

                    if let Some(bits) = fr.bits {
                        let mut bits = bits as u128;
                                                
                        for _i in 0..$type::BITS {
                            if bits & 0x01 == 0 {
                                value <<= 1;
                                bits >>= 1;
                            }
                        }

                        if !fr.bits_start {
                            let byte = ($type::BITS / 8) as usize;
    
                            if input.len() >= byte && let Ok(v) = <[u8; $byte]>::try_from(&input[input.len() - byte..]) {
                                let prev_bits = $type::from_be_bytes(v);
                                value |= prev_bits;
                                for _ in 0..byte { input.pop(); }
                            }    
                        }
                    }
                }
                        
                match byteorder {
                    ByteOrder::Be => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&value.to_be_bytes()[$byte - byte..]);
                        }
                        else {
                            input.extend(value.to_be_bytes());
                        }
                    },
                    ByteOrder::Le => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&value.to_le_bytes()[..byte]);
                        }
                        else {
                            input.extend(value.to_le_bytes());
                        }
                    },
                }            
            }
        }        


        impl BorrowByteEncode for $type {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
                where 
                    Self: Sized
            {
                ByteEncode::encode(self, input, cattr, fattr);
            }
        }
    };
    () => {
        impls_int!(u8, 1);
        impls_int!(u16, 2);
        impls_int!(u32, 4);
        impls_int!(u64, 8);
        impls_int!(usize, 8);
        impls_int!(u128, 16);
        
        impls_int!(i8, 1);
        impls_int!(i16, 2);
        impls_int!(i32, 4);
        impls_int!(i64, 8);
        impls_int!(isize, 8);     
        impls_int!(i128, 16);   
    }
}


impls_int!();


#[cfg(test)]
mod tests {
    use crate::{ContainerAttrModifiers, ByteOrder, FieldAttrModifiers};
    use crate::encode::ByteEncode;

    #[test]
    fn test_encode_int() {
        let value: u16 = 1;

        let mut buf = vec![];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, vec![0, 1]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buf = vec![];
        value.encode(&mut buf, Some(&cattr), None);
        assert_eq!(buf, vec![1, 0]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };

        let mut buf = vec![];
        value.encode(&mut buf, Some(&cattr), Some(&fattr));
        assert_eq!(buf, vec![0, 1]);

        let value: u32 = 1;

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            length: Some(3),
            ..Default::default()
        };

        let mut buf = vec![];
        value.encode(&mut buf, Some(&cattr), Some(&fattr));
        assert_eq!(buf, vec![0, 0, 1]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(3),
            ..Default::default()
        };

        let mut buf = vec![];
        value.encode(&mut buf, Some(&cattr), Some(&fattr));
        assert_eq!(buf, vec![1, 0, 0]);
    }
}
