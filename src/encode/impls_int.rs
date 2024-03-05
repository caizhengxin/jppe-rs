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

                if let Some(value) = cattr && let Some(byteorder_tmp) = value.byteorder {
                    byteorder = byteorder_tmp;
                }

                if let Some(value) = fattr {
                    if let Some(length) = value.length && length < $byte {
                        byte_tmp = Some(length);
                    }

                    if let Some(byteorder_tmp) = value.byteorder {
                        byteorder = byteorder_tmp;
                    }
                }
                        
                match byteorder {
                    ByteOrder::Be => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&self.to_be_bytes()[$byte - byte..]);
                        }
                        else {
                            input.extend(self.to_be_bytes());
                        }
                    },
                    ByteOrder::Le => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&self.to_le_bytes()[..byte]);
                        }
                        else {
                            input.extend(self.to_le_bytes());
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
                let mut byte_tmp = None;
                let mut byteorder = None;

                if let Some(value) = fattr {
                    if let Some(length) = value.length && length < $byte {
                        byte_tmp = Some(length);
                    }

                    byteorder = value.byteorder;
                }
                else if byteorder.is_none() && let Some(value) = cattr {
                    byteorder = value.byteorder;
                }
            
                let mut byteorder_tmp = ByteOrder::Be;
            
                if let Some(byteorder) = byteorder {
                    byteorder_tmp = byteorder;
                }
            
                match byteorder_tmp {
                    ByteOrder::Be => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&self.to_be_bytes()[$byte - byte..]);
                        }
                        else {
                            input.extend(self.to_be_bytes());
                        }
                    },
                    ByteOrder::Le => {
                        if let Some(byte) = byte_tmp {
                            input.extend(&self.to_le_bytes()[..byte]);
                        }
                        else {
                            input.extend(self.to_le_bytes());
                        }
                    },
                }            
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
