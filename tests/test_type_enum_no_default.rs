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
    #[jppe(branch_value=1)]
    Read {
        a: u8,
        b: u16,
    },
    #[jppe(branch_value=3)]
    Write {
        a: u8,
        b: u16,
    },
}


#[test]
fn test_type_enum_range() {
    let input = vec![0x00, 0x05, 0x01, 0x00, 0x02];
    assert_eq!(TestTypeStructEnumExample::decode(&input, None, None).is_err(), true);

    let value = TestTypeStructEnumExample { cmd: 1, body: TestTypeEnumExampleBody::Read {a: 1, b: 2}};
    let input = vec![0x00, 0x01, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let value = TestTypeStructEnumExample { cmd: 3, body: TestTypeEnumExampleBody::Write {a: 1, b: 2}};
    let input = vec![0x00, 0x03, 0x01, 0x00, 0x02];
    let (input2, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input2.is_empty(), true);

    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
}
