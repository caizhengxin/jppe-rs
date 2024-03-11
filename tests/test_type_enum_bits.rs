#![feature(let_chains)]
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
    #[jppe(branch_bits=0x0001)]
    Read {
        a: u8,
        b: u16,
    },
    #[jppe(branch_bits=0x0002)]
    Write {
        a: u8,
        b: u16,
    },
    #[jppe(enum_default)]
    Unknown,
}


#[test]
fn test_type_enum_bits() {
    let input = vec![0x00, 0x04, 0x01, 0x00, 0x02];
    let (input, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, TestTypeStructEnumExample { cmd: 4, body: TestTypeEnumExampleBody::Unknown });
    assert_eq!(input, b"\x01\x00\x02");


    let input = vec![0x00, 0x09, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample { cmd: 9, body: TestTypeEnumExampleBody::Read {a: 1, b: 2}};
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
}
