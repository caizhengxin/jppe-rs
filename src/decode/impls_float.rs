use crate::parser::parse_u128;
use crate::{ByteDecode, BorrowByteDecode, ContainerAttrModifiers, FieldAttrModifiers, JResult};
use crate::get_byteorder;


macro_rules! decode_float {
    ($t:ident, $as_t:ident, $byte:expr) => {
        impl ByteDecode for $t {
            // #[inline]
            fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
            where 
                Self: Sized
            {
                let (input, value) = parse_u128(input, &get_byteorder(cattr, fattr), $byte)?;
        
                Ok((input, $t::from_bits(value as $as_t)))
            }
        }        


        impl<'de> BorrowByteDecode<'de> for $t {
            // #[inline]
            fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
                where 
                    Self: Sized
            {
                let (input, value) = ByteDecode::decode(input, cattr, fattr)?;

                Ok((input, value))
            }
        }
    };

    () => {
        decode_float!(f32, u32, 4);
        decode_float!(f64, u64, 8);
    }
}


decode_float!();


#[cfg(test)]
mod tests {
    use crate::ByteDecode;


    #[test]
    fn test_decode_f32() {
        let (input, value) = f32::decode(&[61, 204, 204, 205], None, None).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);

        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        let (input, value) = f32::decode(&[205, 204, 204, 61], Some(&cattr), None).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);

        let fattr = crate::FieldAttrModifiers {
            byteorder: Some(crate::ByteOrder::Be),
            ..Default::default()
        };
        let (input, value) = f32::decode(&[61, 204, 204, 205], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);
    }

    #[test]
    fn test_decode_f64() {
        let (input, value) = f64::decode(&[63, 185, 153, 153, 153, 153, 153, 154], None, None).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);

        let cattr = crate::ContainerAttrModifiers {
            byteorder: Some(crate::ByteOrder::Le),
            ..Default::default()
        };
        let (input, value) = f64::decode(&[154, 153, 153, 153, 153, 153, 185, 63], Some(&cattr), None).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);

        let fattr = crate::FieldAttrModifiers {
            byteorder: Some(crate::ByteOrder::Be),
            ..Default::default()
        };
        let (input, value) = f64::decode(&[63, 185, 153, 153, 153, 153, 153, 154], Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(value, 0.1);
        assert_eq!(input.is_empty(), true);
    }
}
