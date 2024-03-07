use crate::parser::parse_u128;
use crate::{BorrowByteDecode, ByteDecode};
use crate::{FieldAttrModifiers, ContainerAttrModifiers};
use crate::{JResult, ByteOrder};


#[inline]
fn parse_decode_int<'da, 'db>(input: &'da [u8], byte: u8, cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], u128> {
    let mut byte = byte;
    let mut byteorder = ByteOrder::Be;

    if let Some(cr) = cattr && let Some(byteorder_tmp) = cr.byteorder {
        byteorder = byteorder_tmp;
    }

    if let Some(fr) = fattr {
        if let Some(length) = fr.length && length < byte.into() { byte = length as u8; }
        if let Some(byteorder_tmp) = fr.byteorder { byteorder = byteorder_tmp; }
    }

    let (input, value) = parse_u128(input, &byteorder, byte)?;

    Ok((input, value))
}


macro_rules! impls_int {
    ($type:ident, $byte:expr) => {
        impl ByteDecode for $type {
            fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
                where 
                    Self: Sized
            {
                let (input, mut value) = parse_decode_int(input, $byte, cattr, fattr)?;

                if let Some(fr) = fattr && let Some(bits) = fr.bits {
                    let mut bits = bits as u128;
                    value &= bits;
                                            
                    for _i in 0..$type::BITS {
                        if bits & 0x01 == 0 {
                            value >>= 1;
                            bits >>= 1;
                        }
                    }
                }

                Ok((input, value as $type))
            }
        }        


        impl<'de> BorrowByteDecode<'de> for $type {
            fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
                where 
                    Self: Sized
            {
                let (input, mut value) = parse_decode_int(input, $byte, cattr, fattr)?;

                if let Some(fr) = fattr && let Some(bits) = fr.bits {
                    let mut bits = bits as u128;
                    value &= bits;
                                            
                    for _i in 0..$type::BITS {
                        if bits & 0x01 == 0 {
                            value >>= 1;
                            bits >>= 1;
                        }
                    }
                }

                Ok((input, value as $type))
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
    use crate::decode::ByteDecode;

    #[test]
    fn test_decode_int() {
        let (input, value) = u32::decode(&[0x00, 0x00, 0x00, 0x01], None, None).unwrap();
        assert_eq!(input.is_empty(), true);
        assert_eq!(value, 1);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let (input, value) = u32::decode(&[0x00, 0x00, 0x00, 0x01], Some(&cattr), None).unwrap();
        assert_eq!(input.is_empty(), true);
        assert_eq!(value, 0x01000000);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let (input, value) = u32::decode(&[0x00, 0x00, 0x00, 0x01], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(input.is_empty(), true);
        assert_eq!(value, 1);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            length: Some(3),
            ..Default::default()
        };
        let (input, value) = u32::decode(&[0x00, 0x00, 0x02, 0x01], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(input, &[0x01]);
        assert_eq!(value, 2);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(3),
            ..Default::default()
        };
        let (input, value) = u32::decode(&[0x00, 0x00, 0x02, 0x01], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(input, &[0x01]);
        assert_eq!(value, 0x020000);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            length: Some(0),
            ..Default::default()
        };
        let (input, value) = u32::decode(&[0x00, 0x00, 0x02, 0x01], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(input, &[0x00, 0x00, 0x02, 0x01]);
        assert_eq!(value, 0);
    }
}
