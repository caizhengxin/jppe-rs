use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe::fields::MacAddress;


#[test]
fn test_type_mac_address() {
    let input = b"\xff\xff\xff\xff\xff\xff\x00\x00";

    let (input_remain, mac) = MacAddress::decode(input, None, None).unwrap();
    assert_eq!(mac.is_broadcast(), true);
    assert_eq!(input_remain, &[0x00, 0x00]);

    let mut buf = vec![];
    mac.encode(&mut buf, None, None);

    assert_eq!(buf, b"\xff\xff\xff\xff\xff\xff");
}
