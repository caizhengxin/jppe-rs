#![feature(let_chains)]
use std::collections::HashMap;

use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jppe(linend=b"\x20")]
    pub method: &'a str,
    #[jppe(linend=b"\x20")]
    pub uri: &'a str,
    #[jppe(linend=b"\r\n")]
    pub http: &'a str,
    #[jppe(linend=b"\r\n")]
    pub headers: HashMap<&'a str, &'a str>,
}


fn main() {
    let input = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let (input_remain, value) = Http::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    // The headers hashmap is out of order and cannot be compared.
    // assert_eq!(buf, input);
    assert_eq!(input_remain.is_empty(), true);
}