use std::collections::HashMap;

use crate::parse_subsequence;
use crate::find_substring::FindSubstring;


#[derive(Debug)]
struct KeyValueIterator<'da> {
    input: &'da [u8],
    linend: Vec<Vec<u8>>,
    split_str: Vec<Vec<u8>>,
    count: usize,
    curruent_count: usize,
}


impl<'da, 'db> KeyValueIterator<'da> {
    pub fn new(input: &'da [u8], _cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> Self {
        let mut count = 50;
        let mut linend = vec![b"\r\n".to_vec()];
        let mut split_str = vec![b": ".to_vec()];
    
        if let Some(fattr) = fattr {
            if let Some(linend_tmp) = &fattr.linend_value {
                linend = linend_tmp.to_vec();
            }
    
            if let Some(split_tmp) = &fattr.split {
                split_str = split_tmp.clone();
            }
    
            if let Some(count_tmp) = &fattr.count {
                count = *count_tmp;
            }
        }

        Self {
            input,
            linend,
            split_str,
            count,
            curruent_count: 0,
        }    
    }
}


impl<'a> Iterator for KeyValueIterator<'a> {
    // (input, key, value)
    type Item = (&'a [u8], &'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        if self.curruent_count < self.count {
            for linend in &self.linend {
                match parse_subsequence(self.input, linend) {
                    Ok((input_tmp, value)) => {
                        for split_str in &self.split_str {
                            if let Some(index) = value.find_substring(&split_str[..]) {
                                let key = &value[..index];
                                let value = &value[split_str.len() + index..value.len() - split_str.len()];
                                self.input = input_tmp;
                                self.curruent_count += 1;
        
                                return Some((input_tmp, key, value));
                            }    
                        }
                    },
                    Err(_e) => {
                        return None;
                    },
                }    
            }
        }

        None
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HashMap<&'de [u8], &'de [u8]> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut input = input;
        let mut hashmap = HashMap::new();
        let keyvalue_iter = KeyValueIterator::new(input, cattr, fattr);

        for (remain, key, value) in keyvalue_iter {
            hashmap.insert(key, value);
            input = remain;
        }

        Ok((input, hashmap))        
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HashMap<&'de str, &'de str> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut input = input;
        let mut hashmap = HashMap::new();
        let keyvalue_iter = KeyValueIterator::new(input, cattr, fattr);

        for (remain, key, value) in keyvalue_iter {
            let key = std::str::from_utf8(key).unwrap_or_default();
            let value = std::str::from_utf8(value).unwrap_or_default();

            hashmap.insert(key, value);
            input = remain;
        }

        Ok((input, hashmap))        
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HashMap<String, String> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut input = input;
        let mut hashmap = HashMap::new();
        let keyvalue_iter = KeyValueIterator::new(input, cattr, fattr);

        for (remain, key, value) in keyvalue_iter {
            let key = String::from_utf8_lossy(key).to_string();
            let value = String::from_utf8_lossy(value).to_string();

            hashmap.insert(key, value);
            input = remain;
        }

        Ok((input, hashmap))        
    }
}


impl crate::ByteDecode for HashMap<String, String> {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<&'da [u8], Self>
    where 
        Self: Sized
    {
        let mut input = input;
        let mut hashmap = HashMap::new();
        let keyvalue_iter = KeyValueIterator::new(input, cattr, fattr);

        for (remain, key, value) in keyvalue_iter {
            let key = String::from_utf8_lossy(key).to_string();
            let value = String::from_utf8_lossy(value).to_string();

            hashmap.insert(key, value);
            input = remain;
        }

        Ok((input, hashmap))        
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{ByteDecode, FieldAttrModifiers};

    #[test]
    fn test_impls_hashmap_decode() {
        let input = b"A1: jkc1\r\nA2: jkc2\r\nA3: \r\nabc\r\n";

        let (input, value): (&[u8], HashMap<String, String>) = ByteDecode::decode(input, None, None).unwrap();

        println!("{:?} {:?}", input, value);
        assert_eq!(input, b"abc\r\n");

        let mut hashmap_value = HashMap::new();
        hashmap_value.insert("A1".to_string(), "jkc1".to_string());
        hashmap_value.insert("A2".to_string(), "jkc2".to_string());
        hashmap_value.insert("A3".to_string(), "".to_string());

        assert_eq!(value, hashmap_value);

        let input = b"A1: jkc1\r\nA2: jkc2\r\nA3: \r\nabc\r\n";
        let fattr = FieldAttrModifiers { count: Some(2), ..Default::default() };
        let (input, value): (&[u8], HashMap<String, String>) = ByteDecode::decode(input, None, Some(&fattr)).unwrap();

        println!("{:?} {:?}", input, value);
        assert_eq!(input, b"A3: \r\nabc\r\n");

        let mut hashmap_value = HashMap::new();
        hashmap_value.insert("A1".to_string(), "jkc1".to_string());
        hashmap_value.insert("A2".to_string(), "jkc2".to_string());
        assert_eq!(value, hashmap_value);
    }
}
