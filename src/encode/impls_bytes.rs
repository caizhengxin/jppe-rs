use crate::{ByteEncode, BorrowByteEncode};
use crate::{int_to_vec, get_byteorder};


// #[macro_export]
macro_rules! encode_bytes {
    ($value:expr, $input:expr, $cattr:expr, $fattr:expr) => {
        // key and split
        if let Some(fr) = $fattr {
            if let Some(byte_count) = fr.byte_count {
                $input.extend(int_to_vec($value.len(), byte_count, &get_byteorder($cattr, $fattr)));
            }
            else if fr.linend_value.is_none() && fr.length.is_none() {
                $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
            }
            else {
                if let Some(key) = &fr.key { $input.extend(key); }
                if let Some(splits) = &fr.split && let Some(split) = splits.first() {
                    $input.extend(split);
                }    
            }
        }
        else {
            $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
        }

        $input.extend($value.to_vec());

        if let Some(fr) = $fattr && let Some(linend_value_list) = &fr.linend_value && let Some(linend_value) = linend_value_list.first() {
            if !$value.to_vec().ends_with(linend_value) {
                $input.extend(linend_value)
            }
        }
    };
}


impl ByteEncode for &[u8] {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self , input, cattr, fattr);
    }
}


impl<'de> BorrowByteEncode for &'de [u8] {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self, input, cattr, fattr);
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
        assert_eq!(buf, b"\x03abc");

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

        let fattr = FieldAttrModifiers { byte_count: Some(2), ..Default::default() };
        let mut buf = vec![];
        let value = &b"abc"[..];
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"\x00\x03abc");
    }
}
