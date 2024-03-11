#![feature(let_chains)]
#![feature(exclusive_range_pattern)]
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct TestTypeStructEnumExample {
    pub cmd: u16,
    #[jppe(branch="cmd")]
    pub body: TestTypeEnumExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum TestTypeEnumExampleBody {
    #[jppe(branch_range="1..=3")]
    Read {
        a: u8,
        b: u16,
    },
    #[jppe(branch_value=4)]
    Write {
        a: u8,
        b: u16,
    },
    #[jppe(enum_default)]
    Unknown,
}


#[test]
fn test_type_enum_range() {
    let input = vec![0x00, 0x05, 0x01, 0x00, 0x02];
    let (input, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, TestTypeStructEnumExample { cmd: 5, body: TestTypeEnumExampleBody::Unknown });
    assert_eq!(input, b"\x01\x00\x02");

    let value = TestTypeStructEnumExample { cmd: 1, body: TestTypeEnumExampleBody::Read {a: 1, b: 2}};
    let input = vec![0x00, 0x01, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let value = TestTypeStructEnumExample { cmd: 2, body: TestTypeEnumExampleBody::Read {a: 1, b: 2}};
    let input = vec![0x00, 0x02, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let value = TestTypeStructEnumExample { cmd: 3, body: TestTypeEnumExampleBody::Read {a: 1, b: 2}};
    let input = vec![0x00, 0x03, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let value = TestTypeStructEnumExample { cmd: 4, body: TestTypeEnumExampleBody::Write {a: 1, b: 2}};
    let input = vec![0x00, 0x04, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
}
