use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};
use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct IpAddrExample {
    pub ipv4: Ipv4Addr,
    pub ipv6: Ipv6Addr,
    pub length: u8,
    #[jppe(length="length")]
    pub ipv46: IpAddr,
}


#[test]
fn test_ip_address_example() {
    let input = b"\xc0\xa8\x39\xa0\
    \xfe\x80\x00\x00\x00\x00\x00\x00\x41\x59\xf7\xb2\xb9\xed\x96\x89\
    \x04\
    \xc0\xa8\x39\xa1";

    let (input_remain, value) = IpAddrExample::decode(input, None, None).unwrap();
    assert_eq!(value, IpAddrExample {
        ipv4: Ipv4Addr::from_str("192.168.57.160").unwrap(),
        ipv6: Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:9689").unwrap(),
        length: 4,
        ipv46: IpAddr::V4(Ipv4Addr::from_str("192.168.57.161").unwrap()),
    });
    assert_eq!(input_remain.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);


    let input = b"\xc0\xa8\x39\xa0\
    \xfe\x80\x00\x00\x00\x00\x00\x00\x41\x59\xf7\xb2\xb9\xed\x96\x89\
    \x10\
    \xfe\x80\x00\x00\x00\x00\x00\x00\x41\x59\xf7\xb2\xb9\xed\x96\x8a";

    let (input_remain, value) = IpAddrExample::decode(input, None, None).unwrap();
    assert_eq!(value, IpAddrExample {
        ipv4: Ipv4Addr::from_str("192.168.57.160").unwrap(),
        ipv6: Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:9689").unwrap(),
        length: 16,
        ipv46: IpAddr::V6(Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:968a").unwrap()),
    });
    assert_eq!(input_remain.is_empty(), true);

    let mut buf = Vec::new();
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
}
