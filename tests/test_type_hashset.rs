use std::collections::HashSet;
#[allow(unused_imports)]
use jppe_derive::{ByteEncode, ByteDecode};
#[allow(unused_imports)]
use jppe::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode)]
pub struct HashSetExample {
    pub count: u8,
    #[jppe(count="count")] // or #[jppe(count=3)]
    pub hashset: HashSet<u16>,
}


#[test]
fn test_type_hashmap() {
    let (input, value) = HashSetExample::decode(b"\x03\x00\x01\x00\x02\x00\x02", None, None).unwrap();
    assert_eq!(value, HashSetExample { count: 3, hashset: HashSet::from([1, 2]) });
    assert_eq!(input.is_empty(), true);

    // Encode is dangerous function with out-of-order and coding problems.
    // count=3 only have 2 elements.
    // let mut buf = Vec::new();
    // value.encode(&mut buf, None, None);
    // assert!(buf == b"\x03\x00\x01\x00\x02" ||  buf == b"\x03\x00\x02\x00\x01");
}
