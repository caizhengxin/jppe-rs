#![feature(let_chains)]
use std::str::FromStr;
use jkcenum::JkcEnum;
use jppe::BorrowByteDecode;
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jppe(linend=b"\x20", from_str)]
    pub method: HttpMethodEnum,
    #[jppe(linend=b"\x20")]
    pub uri: &'a str,
    #[jppe(linend=b"\r\n")]
    pub version: &'a str,
    #[jppe(try_count=20)]
    pub headers: Vec<HttpHeader<'a>>,
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct HttpHeader<'a> {
    #[jppe(linend=b": ")]
    pub key: &'a str,
    #[jppe(linend=b"\r\n")]
    pub value: &'a str,
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode, JkcEnum)]
#[jppe(byte_count_disable)]
pub enum HttpMethodEnum {
    #[default]
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}


fn main() {
    // decode
    let input = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let (input_remain, value) = jppe::decode_borrow::<Http>(input).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(input_remain.is_empty(), true);

    // encode
    assert_eq!(jppe::encode_borrow(value), input);

    // error
    let input: &[u8; 97] = b"SET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    assert_eq!(jppe::decode_borrow::<Http>(input).is_err(), true);
}