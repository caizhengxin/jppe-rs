use crate::{ByteEncode, BorrowByteEncode};


macro_rules! encode_string {
    ($value:expr, $input:expr, $fattr:expr) => {
        // key and split
        if let Some(fattr) = $fattr {
            if let Some(key) = &fattr.key { $input.extend(key); }
            if let Some(splits) = &fattr.split && let Some(split) = splits.first() {
                $input.extend(split);
            }
        }

        // value
        $input.extend($value.as_bytes());

        if let Some(fattr) = $fattr {
            if let Some(linend_value_list) = &fattr.linend_value && let Some(linend_value) = linend_value_list.first() {
                if !$value.as_bytes().ends_with(linend_value) {
                    $input.extend(linend_value)
                }
            }
            else if fattr.length.is_none() {
                $input.push(b'\n');
            }
        }
        else {
            $input.push(b'\n');
        }    
    };
}


impl ByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, fattr);
    }
}


impl BorrowByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, fattr);
    }
}


impl ByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, fattr);
    }
}


impl BorrowByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, fattr);
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
    }
}