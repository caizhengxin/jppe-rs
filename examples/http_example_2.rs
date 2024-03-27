#![feature(let_chains)]
use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jppe(linend="\x20")]
    pub method: &'a str,
    #[jppe(linend="\x20")]
    pub uri: &'a str,
    #[jppe(linend="\r\n")]
    pub version: &'a str,
    pub headers: Vec<HttpEnum<'a>>,
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct HttpEnum<'a> {
    #[jppe(linend=": ")]
    pub key: &'a str,
    #[jppe(linend="\r\n")]
    pub value: &'a str,
}


fn main() {
    let input = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let (input_remain, value) = Http::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
    assert_eq!(input_remain.is_empty(), true);
}