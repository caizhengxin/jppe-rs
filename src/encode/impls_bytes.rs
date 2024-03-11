use crate::{ByteEncode, BorrowByteEncode};


macro_rules! encode_bytes {
    ($value:expr, $input:expr, $fattr:expr) => {
        // key and split
        if let Some(fattr) = $fattr {
            if let Some(key) = &fattr.key { $input.extend(key); }
            if let Some(splits) = &fattr.split && let Some(split) = splits.first() {
                $input.extend(split);
            }
        }

        $input.extend($value.to_vec());

        if let Some(fattr) = $fattr && let Some(linend_value_list) = &fattr.linend_value && let Some(linend_value) = linend_value_list.first() {
            if !$value.to_vec().ends_with(linend_value) {
                $input.extend(linend_value)
            }
        }
    };
}


impl ByteEncode for &[u8] {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self , input, fattr);
    }
}


impl<'de> BorrowByteEncode for &'de [u8] {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self, input, fattr);
    }
}


#[cfg(test)]
mod tests {
    use crate::encode::BorrowByteEncode;
    use crate::FieldAttrModifiers;

    #[test]
    fn test_encode_bytes() {
        let mut buf = vec![];
        let value = &[b'a', b'b', b'c'][..];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, b"abc");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["\r\n".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = &[b'a', b'b', b'c'][..];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc\r\n");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["abc".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = &[b'a', b'b', b'c'][..];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["123".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = &[b'a', b'b', b'c'][..];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc123");

        let fattr = FieldAttrModifiers { key: Some(b"Host: ".to_vec()), linend_value: Some(vec![b"\r\n".to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = &b"abc"[..];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"Host: abc\r\n");
    }
}
