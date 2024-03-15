use crate::{ByteEncode, BorrowByteEncode};
use crate::{int_to_vec, get_byteorder};


macro_rules! encode_string {
    ($value:expr, $input:expr, $cattr:expr, $fattr:expr) => {
        let mut byte_count_status = false;

        // key and split
        if let Some(fr) = $fattr {
            if let Some(byte_count) = fr.byte_count {
                $input.extend(int_to_vec($value.len(), byte_count, &get_byteorder($cattr, $fattr)));
                byte_count_status = true;
            }
            if let Some(key) = &fr.key { $input.extend(key); }
            if let Some(splits) = &fr.split && let Some(split) = splits.first() {
                $input.extend(split);
            }
        }

        // value
        $input.extend($value.as_bytes());

        if byte_count_status { return (); }

        if let Some(fr) = $fattr {
            if let Some(linend_value_list) = &fr.linend_value && let Some(linend_value) = linend_value_list.first() {
                if !$value.as_bytes().ends_with(linend_value) {
                    $input.extend(linend_value)
                }
            }
            else if fr.length.is_none() {
                $input.push(b'\n');
            }
        }
        else {
            $input.push(b'\n');
        }    
    };
}


impl ByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl BorrowByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl ByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl BorrowByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


#[cfg(test)]
mod tests {
    use crate::encode::ByteEncode;
    use crate::FieldAttrModifiers;

    #[test]
    fn test_encode_string() {
        let mut buf = vec![];
        let value = String::from("abc");
        value.encode(&mut buf, None, None);
        assert_eq!(buf, b"abc\n");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["\r\n".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = String::from("abc");
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc\r\n");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["abc".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = String::from("abc");
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["123".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = String::from("abc");
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc123");

        let fattr = FieldAttrModifiers { key: Some(b"Host: ".to_vec()), linend_value: Some(vec![b"\r\n".to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = "abc".to_string();
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"Host: abc\r\n");
    }

    #[test]
    fn test_encode_str() {
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, None);
        assert_eq!(buf, b"abc\n");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["\r\n".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc\r\n");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["abc".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc");

        let fattr = FieldAttrModifiers { linend_value: Some(vec!["123".as_bytes().to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"abc123");

        let fattr = FieldAttrModifiers { key: Some(b"Host: ".to_vec()), linend_value: Some(vec![b"\r\n".to_vec()]), ..Default::default() };
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"Host: abc\r\n");

        let fattr = FieldAttrModifiers { byte_count: Some(2), ..Default::default() };
        let mut buf = vec![];
        let value = "abc";
        value.encode(&mut buf, None, Some(&fattr));
        assert_eq!(buf, b"\x00\x03abc");
    }
}