use std::str::FromStr;
use std::net::{Ipv4Addr, Ipv6Addr};
use jppe::{ByteDecode, ByteEncode};
use jppe::fields::{MacAddress, PpeAddress};
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct PpeAddrExample {
    #[jppe(length=4)]
    pub ipv4: PpeAddress,
    #[jppe(length=16)]
    pub ipv6: PpeAddress,
    #[jppe(length=6)]
    pub mac: PpeAddress,
    pub length: u8,
    #[jppe(length="length")]
    pub addr: PpeAddress,
}


#[test]
fn test_ppe_address_example() {
    let input = b"\xc0\xa8\x39\xa0\
    \xfe\x80\x00\x00\x00\x00\x00\x00\x41\x59\xf7\xb2\xb9\xed\x96\x89\
    \xff\xff\xff\xff\xff\xff\
    \x02\
    \x00\x01";

    let (input_remain, value) = PpeAddrExample::decode(input, None, None).unwrap();
    assert_eq!(value, PpeAddrExample {
        ipv4: PpeAddress::V4(Ipv4Addr::from_str("192.168.57.160").unwrap()),
        ipv6: PpeAddress::V6(Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:9689").unwrap()),
        mac: PpeAddress::Mac(MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap()),
        length: 2,
        addr: PpeAddress::Usize(1),
    });
    assert_eq!(input_remain.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
}