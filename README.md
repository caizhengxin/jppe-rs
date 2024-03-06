# jppe-rs

This is a byte stream structured serialization and deserialization library that is still under development.

## Usage

### Cargo.toml

```toml
jppe = { git = "https://github.com/caizhengxin/jppe.git", features = ["derive"] }
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

## Feature

### ContainerAttrModifiers

- [x] `byteorder`
- [ ] `with_encode`: encode function.
- [ ] `with_decode`: decode function.
- [ ] `with`: encode/decode function.

> enum branch

- [ ] `branch_byte`
- [ ] `branch_byteorder`
- [ ] `branch_func`
- [ ] `branch_enum`

### FieldAttrModifiers

- [x] `byteorder`
- [x] `length`
- [x] `offset`
- [x] `count`
- [x] `full=<int>`: encode full value.
- [x] `untake`
- [x] `linend=<string|bytes>`
- [x] `key`
- [x] `split`
- [ ] `if_expr`
- [ ] `with_encode`: encode function.
- [ ] `with_decode`: decode function.
- [ ] `with`: encode/decode function.

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
- [x] `HashMap`
- [ ] `HashSet`
- [ ] `Macaddress`
- [ ] `IPv4` or `IPv6`
- [ ] `Hex`
- [ ] `DateTime`
