
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jppe(linend=b"\x20")]
    pub method: &'a str,
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


fn main() {
    // decode
    let input = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let (input_remain, value) = jppe::decode_borrow::<Http>(input).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(input_remain.is_empty(), true);

    // encode
    assert_eq!(jppe::encode_borrow(value), input);
}