
use jppe_derive::{ByteEncode, ByteDecode};
use jdefault_derive::Jdefault;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
pub struct SimpleExample {
    #[jppe(byte_count=1, default="\"123\".to_string()")]
    pub value: String,
    #[jppe(byte_count=1)]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
#[repr(u8)]
pub enum SimpleExampleBody {
    Read {
        address: u8,
    } = 1,
    Write {
        address: u8,
        value: [u8; 3],
    },
    #[jppe(branch_default)]
    Unknown {
        #[jppe(default=10)]
        value: u8,
    },
}


fn main() {
    let value = SimpleExample::default();
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });

    assert_eq!(jppe::encode(value), b"\x03\x31\x32\x33\x03\x0a");

    let (input_remain, value) = jppe::decode::<SimpleExample>(b"\x03\x31\x32\x33\x03\x0a").unwrap();
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });
    assert_eq!(input_remain.is_empty(), true);
}