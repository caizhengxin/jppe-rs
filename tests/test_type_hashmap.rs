use std::collections::HashMap; 
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};
use jppe::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample<'a> {
    pub kv: HashMap<&'a str, &'a str>,
}


#[test]
fn test_impls_hashmap() {
    let mut hashmap_value = HashMap::new();
    hashmap_value.insert("A1", "jkc1");
    hashmap_value.insert("A2", "jkc2");
    hashmap_value.insert("A3", "");

    let hashmap_value = HashMapExample { 
        kv: hashmap_value,
    };

    let mut buf = vec![];
    hashmap_value.encode(&mut buf, None, None);

    let (input, value): (&[u8], HashMapExample) = BorrowByteDecode::decode(&buf, None, None).unwrap();

    assert_eq!(input.is_empty(), true);
    assert_eq!(value, hashmap_value);
}
