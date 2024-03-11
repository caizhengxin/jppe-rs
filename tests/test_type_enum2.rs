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
    #[jppe(branch_value=0x0001)]
    V0,
    #[jppe(branch_value=0x0003)]
    V1(u8),
    #[jppe(branch_value=0x0005)]
    V2(u8, u16),
    #[jppe(branch_value=0x0007)]
    V3((u8, u16)),
    #[jppe(branch_value=0x0009)]
    V4 {
        a: u8,
        b: u16,
    },
    #[jppe(enum_default)]
    Unknown,
}


#[test]
fn test_type_enum2() {
    // V0
    let value = TestTypeStructEnumExample { cmd: 1, body: TestTypeEnumExampleBody::V0 };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x01]);

    let (input, value2) = TestTypeStructEnumExample::decode(&[0x00, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);

    // V1
    let value = TestTypeStructEnumExample { cmd: 3, body: TestTypeEnumExampleBody::V1(1) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x03, 0x01]);

    let (input, value2) = TestTypeStructEnumExample::decode(&[0x00, 0x03, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);

    // V2
    let input = vec![0x00, 0x05, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample { cmd: 5, body: TestTypeEnumExampleBody::V2(1, 2) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);

    // V3
    let input = vec![0x00, 0x07, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample { cmd: 7, body: TestTypeEnumExampleBody::V3((1, 2)) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);

    // V4
    let input = vec![0x00, 0x09, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample { cmd: 9, body: TestTypeEnumExampleBody::V4 {a: 1, b: 2}};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input.is_empty(), true);
}
