use crate::std::*;
use std::collections::HashMap;
use crate::parse_subsequence;


#[derive(Debug)]
struct KeyValueIterator<'da, 'db> {
    input: &'da [u8],
    linend: Option<&'db [u8]>,
    split_str: Option<&'db [u8]>,
    count: usize,
    curruent_count: usize,
}


impl<'da, 'db> KeyValueIterator<'da, 'db> {
    pub fn new(input: &'da [u8], _cattr: Option<&'db crate::ContainerAttrModifiers>, fattr: Option<&'db crate::FieldAttrModifiers>) -> Self {
        let mut count = 50;
        let mut linend = None;
        let mut split_str = None;
    
        if let Some(fattr) = fattr {
            linend = fattr.linend_value;
            split_str = fattr.split;
            count = fattr.count.unwrap_or(50);
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


impl<'da, 'db> Iterator for KeyValueIterator<'da, 'db> {
    // (input, key, value)
    type Item = (&'da [u8], &'da [u8], &'da [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let split_str = self.split_str.unwrap_or(b": ");
        let linend = self.linend.unwrap_or(b"\r\n");

        if self.curruent_count < self.count {
    
            if let Ok((input, key)) = parse_subsequence(self.input, split_str, false)
            {
                if let Ok((input, value)) = parse_subsequence(input, linend, false) {
                    self.input = input;
                    self.curruent_count += 1;
        
                    return Some((input, key, value));    
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
            let key = match str::from_utf8(key) {
                Ok(v) => v,
                Err(_e) => return Err(crate::make_error(input, crate::ErrorKind::Fail { offset: input.len() })),
            };

            let value = match str::from_utf8(value) {
                Ok(v) => v,
                Err(_e) => return Err(crate::make_error(input, crate::ErrorKind::Fail { offset: input.len() })),
            };

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
