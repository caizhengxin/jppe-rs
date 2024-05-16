use crate::std::*;

macro_rules! impls_tuple {
    ($($t:ident),+) => {
        #[allow(non_camel_case_types)]
        impl<$($t: crate::ByteEncode,)+> crate::ByteEncode for ($($t,)+)
        {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
            {
                let ($($t,)*) = self;
                $(
                    $t.encode(input, cattr, fattr);
                )*        
            }
        }


        #[allow(non_camel_case_types)]
        impl<$($t: crate::BorrowByteEncode,)+> crate::BorrowByteEncode for ($($t,)+)
        {
            fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
            {
                let ($($t,)*) = self;
                $(
                    $t.encode(input, cattr, fattr);
                )*        
            }
        }
    };

    () => {
        impls_tuple!(t1);
        impls_tuple!(t1, t2);
        impls_tuple!(t1, t2, t3);
        impls_tuple!(t1, t2, t3, t4);
        impls_tuple!(t1, t2, t3, t4, t5);
        impls_tuple!(t1, t2, t3, t4, t5, t6);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7, t8);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7, t8, t9);
    };
}


impls_tuple!();


#[cfg(test)]
mod tests {
    use crate::{encode::ByteEncode, FieldAttrModifiers, ByteOrder};

    #[test]
    fn test_tuple_encode() {
        let value = (1 as u16, 2 as u16);

        let mut buf = vec![];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, vec![0x00, 0x01, 0x00, 0x02]);

        let fattr = FieldAttrModifiers { byteorder: Some(ByteOrder::Le), ..Default::default() };
        let mut buf = vec![];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, vec![0x01, 0x00, 0x02, 0x00]);

        let value = (1 as u8, 2 as u16);

        let mut buf = vec![];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, vec![0x01, 0x00, 0x02]);
    }
}