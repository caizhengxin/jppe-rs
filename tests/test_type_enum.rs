#![feature(let_chains)]
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct TestTypeStructEnumExample1 {
    pub cmd: u16,
    #[jppe(branch="cmd")]
    pub body: TestTypeEnumExample,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u16)]
enum TestTypeEnumExample {
   V0 = 0x0001,
   V1(u8),
   V2(u8, u16),
   V3((u8, u16)),
   V4 {
       a: u8,
       b: u16,
   },
   #[jppe(enum_default)]
   Unknown,
}


#[test]
fn test_type_enum() {
    // V0
    let value = TestTypeStructEnumExample1 { cmd: 1, body: TestTypeEnumExample::V0 };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x01]);

    let (input, value2) = TestTypeStructEnumExample1::decode(&[0x00, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input, &[]);

    // V1
    let value = TestTypeStructEnumExample1 { cmd: 2, body: TestTypeEnumExample::V1(1) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, vec![0x00, 0x02, 0x01]);

    let (input, value2) = TestTypeStructEnumExample1::decode(&[0x00, 0x02, 0x01], None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input, &[]);

    // V2
    let input = vec![0x00, 0x03, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample1 { cmd: 3, body: TestTypeEnumExample::V2(1, 2) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample1::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input, &[]);

    // V3
    let input = vec![0x00, 0x04, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample1 { cmd: 4, body: TestTypeEnumExample::V3((1, 2)) };
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample1::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input, &[]);

    // V4
    let input = vec![0x00, 0x05, 0x01, 0x00, 0x02];
    let value = TestTypeStructEnumExample1 { cmd: 5, body: TestTypeEnumExample::V4 {a: 1, b: 2}};
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);

    let (input, value2) = TestTypeStructEnumExample1::decode(&input, None, None).unwrap();
    assert_eq!(value2, value);
    assert_eq!(input, &[]);
}
