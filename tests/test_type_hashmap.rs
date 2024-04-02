use std::collections::HashMap; 
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};
use jppe::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample<'a> {
    pub kv: HashMap<&'a str, &'a str>,
}


#[test]
fn test_type_hashmap() {
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


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample2<'a> {
    #[jppe(split="=", linend=b"\r\n")]
    pub kv: HashMap<&'a [u8], &'a [u8]>,
}


#[test]
fn test_type_hashmap_key_and_split() {
    let input = b"A1=jkc1\r\nA2=jkc2\r\njkc";

    let (input, value) = HashMapExample2::decode(input, None, None).unwrap();

    let mut hashmap_value = HashMap::new();
    hashmap_value.insert("A1".as_bytes(), "jkc1".as_bytes());
    hashmap_value.insert("A2".as_bytes(), "jkc2".as_bytes());

    let hashmap_value = HashMapExample2 { 
        kv: hashmap_value,
    };
    
    assert_eq!(value, hashmap_value);
    assert_eq!(input, b"jkc");

    let mut buf = vec![];
    value.encode(&mut buf, None, None);

    assert!(buf == b"A1=jkc1\r\nA2=jkc2\r\n" || buf == b"A2=jkc2\r\nA1=jkc1\r\n" );
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample3<'a> {
    #[jppe(split="=", linend=b"\r\n", count=1)]
    pub kv: HashMap<&'a [u8], &'a [u8]>,
}


#[test]
fn test_type_hashmap_key_split_count() {
    let input = b"A1=jkc1\r\nA2=jkc2\r\njkc";

    let (input, value) = HashMapExample3::decode(input, None, None).unwrap();

    let mut hashmap_value = HashMap::new();
    hashmap_value.insert("A1".as_bytes(), "jkc1".as_bytes());

    let hashmap_value = HashMapExample3 { 
        kv: hashmap_value,
    };
    
    assert_eq!(value, hashmap_value);
    assert_eq!(input, b"A2=jkc2\r\njkc");
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample4<'a> {
    #[jppe(split="\x00,\x01", linend=b"\r\n")]
    pub kv: HashMap<&'a [u8], &'a [u8]>,
}


#[test]
fn test_type_hashmap_key_split() {
    let input = b"A1\x00jkc1\r\nA2\x01jkc2\r\njkc";

    let (input, value) = HashMapExample4::decode(input, None, None).unwrap();

    let mut hashmap_value = HashMap::new();
    hashmap_value.insert("A1".as_bytes(), "jkc1".as_bytes());
    hashmap_value.insert("A2".as_bytes(), "jkc2".as_bytes());

    let hashmap_value = HashMapExample4 { 
        kv: hashmap_value,
    };
    
    assert_eq!(value, hashmap_value);
    assert_eq!(input, b"jkc");
}