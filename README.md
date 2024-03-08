# jppe-rs

[![Crates.io](https://img.shields.io/crates/v/jppe)](https://crates.io/crates/jppe)
[![Crates.io](https://img.shields.io/crates/d/jppe)](https://crates.io/crates/jppe)
[![License](https://img.shields.io/crates/l/jppe)](LICENSE)

This is a byte stream structured serialization and deserialization library that is still under development.

## Usage

### Cargo.toml

```toml
[dependencies]
jppe = { version="0.2.0", features = ["derive"] }
```

### Simple Example

```rust
#![feature(let_chains)]
use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    #[jppe(length="length")]
    pub value: String,
    pub cmd: u8,
    #[jppe(branch="cmd")]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum SimpleExampleBody {
    Read {
        address: u8,
    } = 1,
    Write {
        address: u8,
        value: [u8; 3],
    },
    #[jppe(enum_default)]
    Unknown, 
}


fn main() {
    let input = b"\x00\x03\x31\x32\x33\x01\x05";

    let (input_remain, value) = SimpleExample::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");

    let mut buf = vec![];
    value.encode(&mut buf, None, None);

    assert_eq!(buf, input);
}
```

### Ethernet Example

```rust
#![feature(let_chains)]
use std::str::FromStr;

use jppe::{ByteDecode, ByteEncode};
use jppe_derive::{ByteEncode, ByteDecode};
use jppe::prelude::MacAddress;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct Ethernet {
    pub smac: MacAddress,
    pub dmac: MacAddress,
    pub r#type: u16,
}


fn main() {
    let input = b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00\x45\x00";

    // decode
    let (input_remain, value) = Ethernet::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(value, Ethernet {
        smac: MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap(),
        dmac: MacAddress::from_str("00:00:00:00:00:00").unwrap(),
        r#type: 0x0800,
    });

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00");
    assert_eq!(input_remain, b"\x45\x00");
}
```

## Ipv4 Example

```rust
#![feature(let_chains)]
use std::net::Ipv4Addr;
use std::str::FromStr;
use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Ipv4<'a> {
    #[jppe(bits_start=0xf0, untake)]
    pub version: u8,
    #[jppe(bits=0x0f, decode_value="header_length << 2", encode_value="header_length >> 2")]
    pub header_length: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    #[jppe(bits_start=0xe000, untake)]
    pub flags: u16,
    #[jppe(bits=0x1fff)]
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
    #[jppe(length="header_length - 20")]
    pub options: &'a [u8],
}


fn main() {
    let input = b"\x45\x00\x00\x40\xb5\xf2\x00\x00\x40\x06\xa9\x7c\x0a\x01\x01\xea\x0a\x0a\x05\x55";    

    // decode
    let (input_remain, value) = Ipv4::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(value, Ipv4 {
        version: 4,
        header_length: 20,
        tos: 0,
        total_length: 64,
        identification: 46578,
        flags: 0,
        fragment_offset: 0,
        ttl: 64,
        protocol: 6,
        checksum: 43388,
        src: Ipv4Addr::from_str("10.1.1.234").unwrap(),
        dst: Ipv4Addr::from_str("10.10.5.85").unwrap(),
        options: &[],
    });

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
    assert_eq!(input_remain.is_empty(), true);
}
```

### Tcp Example

```rust
#![feature(let_chains)]
use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Tcp<'a> {
    pub sport: u16,
    pub dport: u16,
    pub seq: u32,
    pub ack: u32,
    #[jppe(bits_start=0xf000, decode_value="header_length * 4", encode_value="header_length / 4", untake)]
    pub header_length: u16,
    #[jppe(bits=0x0fff)]
    pub flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    #[jppe(length="header_length - 20")]
    pub options: &'a [u8],
}


fn main() {
    let input = b"\xc8\xd3\x01\xf6\xe0\x76\x90\x16\xc4\x44\x9b\x5a\x80\x18\xff\xff\
    \x6c\x1c\x00\x00\x01\x01\x08\x0a\x37\xc4\x50\xe2\x00\xba\x7c\x1c";    

    // decode
    let (input_remain, value) = Tcp::decode(input, None, None).unwrap();
    println!("{value:?} {input_remain:?}");
    assert_eq!(value, Tcp {
        sport: 51411,
        dport: 502,
        seq: 3765866518,
        ack: 3292830554,
        header_length: 32,
        flags: 24,
        window: 65535,
        checksum: 27676,
        urgent_pointer: 0,
        options: b"\x01\x01\x08\x0a\x37\xc4\x50\xe2\x00\xba\x7c\x1c",
    } );

    // encode
    let mut buf = vec![];
    value.encode(&mut buf, None, None);
    assert_eq!(buf, input);
    assert_eq!(input_remain.is_empty(), true);
}
```

### HTTP Example

```rust
#![feature(let_chains)]
use std::collections::HashMap;

use jppe::{BorrowByteDecode, BorrowByteEncode};
use jppe_derive::{BorrowByteEncode, BorrowByteDecode};


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jppe(linend="\x20")]
    pub method: &'a str,
    #[jppe(linend="\x20")]
    pub uri: &'a str,
    #[jppe(linend="\r\n")]
    pub http: &'a str,
    #[jppe(linend="\r\n")]
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
```

## Feature

### ContainerAttrModifiers

- [x] `byteorder=<"BE"|"LE">`: The global byte order of struct and enum, eg: `#[jppe(byteorder="LE")]`.
- [x] `encode_with`: custom encode function.
- [x] `decode_with`: custom decode function.
- [x] `with`: custom encode/decode function.
- [ ] `get_variable_name`

> enum branch

- [ ] `branch_byte`
- [ ] `branch_byteorder`
- [ ] `branch_func`
- [ ] `branch_enum`

### FieldAttrModifiers

- [x] `byteorder=<"BE"|"LE">`: The byte order of locality field, eg：`#[jppe(byteorder="LE")]`
- [x] `length=<num|variable>`: Data length, eg: `int/&str/String`.
- [x] `offset=<num|variable>`: Byte stream offset.
- [x] `count==<num|variable>`: Data count, eg: `Vec`;
- [x] `full=<int>`: encode full value.
- [x] `untake`: Bytes are not taken.
- [x] `linend=<string|bytes>`: eg: `string`.
- [ ] `key`
- [x] `split`: eg: `hashmap`
- [ ] `if_expr`
- [x] `encode_with`: custom encode function.
- [x] `decode_with`: custom decode function.
- [x] `with`: custom encode/decode function.
- [x] `encode_value`: value processing expression, eg: `#[jppe(encode_value="length * 2")]`.
- [x] `decode_value`: value processing expression, eg: `#[jppe(decode_value="length / 2")]`.
- [ ] `variable_name`

> enum branch

- [x] `branch`
- [x] `branch_default`
- [ ] `branch_bits`
- [ ] `branch_range`
- [ ] `branch_value`
- [ ] `branch_expr`

### DataType

- [x] `u8/u16/u32/u64/usize/u128`
- [x] `i8/i16/i32/i64/isize/i128`
- [x] `bool`
- [x] `f32/f64`
- [x] `String` and `&str`
- [x] `array[T; N]`
- [x] `Tuple`
- [x] `Vec<T>`
- [x] `&[u8]`
- [x] `Option<T>`
- [x] `Struct`
- [x] `Enum`
- [x] `PhantomData`
- [x] `HashMap`
- [ ] `HashSet`
- [x] `Macaddress`
- [x] `IPv4` or `IPv6`
- [ ] `Hex`
- [ ] `DateTime`
- [ ] `Bit`
