use std::collections::HashMap;


macro_rules! hashmap_encode_bytes {
    ($value:expr, $input:expr, $fattr:expr) => {
        let mut linend = b"\r\n".to_vec();
        let mut split_str = b": ".to_vec();

        if let Some(fattr) = $fattr {
            if let Some(linend_list) = &fattr.linend_value && let Some(linend_tmp) = linend_list.first() {
                linend = linend_tmp.to_vec();
            }
    
            if let Some(split_list) = &fattr.split && let Some(split_tmp) = split_list.first() {
                split_str = split_tmp.clone();
            }    
        }

        for (key, value) in $value {
            $input.extend(key.to_vec());
            $input.extend(split_str.clone());
            $input.extend(value.to_vec());
            $input.extend(linend.clone());
        }
    };
}


macro_rules! hashmap_encode_bytes2 {
    ($value:expr, $input:expr, $fattr:expr) => {
        let mut linend = b"\r\n".to_vec();
        let mut split_str = b": ".to_vec();

        if let Some(fattr) = $fattr {
            if let Some(linend_list) = &fattr.linend_value && let Some(linend_tmp) = linend_list.first() {
                linend = linend_tmp.to_vec();
            }
    
            if let Some(split_list) = &fattr.split && let Some(split_tmp) = split_list.first() {
                split_str = split_tmp.clone();
            }    
        }

        for (key, value) in $value {
            $input.extend(key.as_bytes().to_vec());
            $input.extend(split_str.clone());
            $input.extend(value.as_bytes().to_vec());
            $input.extend(linend.clone());
        }
    };
}



impl crate::ByteEncode for HashMap<&[u8], &[u8]> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes!(self , input, fattr);
    }
}


impl crate::BorrowByteEncode for HashMap<&[u8], &[u8]> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes!(self, input, fattr);
    }
}


impl crate::ByteEncode for HashMap<&str, &str> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes2!(self, input, fattr);
    }
}


impl crate::BorrowByteEncode for HashMap<&str, &str> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes2!(self, input, fattr);
    }
}


impl crate::ByteEncode for HashMap<String, String> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes2!(self, input, fattr);
    }
}


impl crate::BorrowByteEncode for HashMap<String, String> {
    fn encode(&self, input: &mut Vec<u8>, _cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        hashmap_encode_bytes2!(self, input, fattr);
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ByteEncode;

    #[test]
    fn test_impls_hashmap_encode() {
        let mut hashmap_value = HashMap::new();
        hashmap_value.insert("A1".to_string(), "jkc1".to_string());
        hashmap_value.insert("A2".to_string(), "jkc2".to_string());
        hashmap_value.insert("A3".to_string(), "".to_string());
        let mut buf = vec![];
        hashmap_value.encode(&mut buf, None, None);
        let value = String::from_utf8_lossy(&buf).to_string();
        println!("{value:?}");
    }
}