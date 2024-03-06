use crate::{ByteEncode, BorrowByteEncode};


impl ByteEncode for bool {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        if *self { input.push(1) } else { input.push(0) };
    }
}


impl BorrowByteEncode for bool {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>)
    {
        if *self { input.push(1) } else { input.push(0) };
    }
}


impl<T: ByteEncode> ByteEncode for Option<T> {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    {
        if let Some(value) = self {
            value.encode(input, cattr, fattr);
        }
    }
}


impl<T: BorrowByteEncode> BorrowByteEncode for Option<T> {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    {
        if let Some(value) = self {
            value.encode(input, cattr, fattr);
        }
    }
}


impl<T: ByteEncode> ByteEncode for Vec<T> {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    {
        for value in self {
            value.encode(input, cattr, fattr);
        }
    }
}


impl<T: BorrowByteEncode> BorrowByteEncode for Vec<T> {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    {
        for value in self {
            value.encode(input, cattr, fattr);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{ByteEncode, FieldAttrModifiers, ByteOrder};

    #[test]
    fn test_encode_bool() {
        let mut buf = vec![];
        let value = false;
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x00]);

        let mut buf = vec![];
        let value = true;
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x01]);
    }

    #[test]
    fn test_encode_option() {
        let mut buf = vec![];
        let value = Some(1_u8);
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x01]);

        let mut buf = vec![];
        let value = Some(1_u16);
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x00, 0x01]);

        let mut buf = vec![];
        let value = Some(1_u16);
        let fattr = FieldAttrModifiers { byteorder: Some(ByteOrder::Le), ..Default::default() };
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, [0x01, 0x00]);

        let mut buf = vec![];
        let value: Option<u8> = None;
        value.encode(&mut buf, None, None);
        assert_eq!(buf.is_empty(), true);
    }
}