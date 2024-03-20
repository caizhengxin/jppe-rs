# jppe-rs

[![Crates.io](https://img.shields.io/crates/v/jppe)](https://crates.io/crates/jppe)
[![Crates.io](https://img.shields.io/crates/d/jppe)](https://crates.io/crates/jppe)
[![License](https://img.shields.io/crates/l/jppe)](LICENSE)

This is a byte stream structured serialization and deserialization library.

## Usage

### Cargo.toml

```toml
[dependencies]
jppe = { version="0.6.0", features = ["derive"] }
```

Or

```toml
[dependencies]
jppe = { version="0.6.0", features = ["derive", "serde"] }
```

### Simple Example

```rust
#![feature(let_chains)]
use jppe::{ByteEncode, ByteDecode};


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
    let (input_remain, value) = jppe::decode::<SimpleExample>(input).unwrap();
    assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(input_remain.is_empty(), true);
    assert_eq!(jppe::encode(value), input);
}
```

### Example

- [tcp_communication_example](./examples/socket_example.rs)
- [ethernet_example](./examples/ethernet_example.rs)
- [ipv4_example](./examples/ipv4_example.rs)
- [tcp_example](./examples/tcp_example.rs)
- [http_example](./examples/http_example.rs)
- [parse_example](./examples/parse_example.rs): Including Ethernet/IPv4/TCP/UDP

## Feature

### ContainerAttrModifiers

- [x] `byteorder=<"BE"|"LE">`: The global byte order of struct and enum, eg: `#[jppe(byteorder="LE")]`.
- [x] `encode_with`: custom encode function.
- [x] `decode_with`: custom decode function.
- [x] `with`: custom encode/decode function.
- [x] `get_variable_name`: Get cache variable, must be used with `variable_name`, only for decode, eg: `test_modifier_variable_name.rs`.

> enum branch

- [ ] `branch_enum`

### FieldAttrModifiers

- [x] `byteorder=<"BE"|"LE">`: The byte order of locality field, eg：`#[jppe(byteorder="LE")]`
- [x] `length=<num|variable>`: Data length, support `int/&str/String/&[u8]` type, eg: [length_example](./tests/test_modifier_length.rs).
- [x] `offset=<num|variable>`: Byte stream offset.
- [x] `count==<num|variable>`: Data count, support `Vec/HashMap` type.
- [x] `full=<int>`: encode full value.
- [x] `untake`: Bytes are not taken.
- [x] `linend|end_with=<string|bytes>`: Supporting `String/&str/&[u8]` type.
- [x] `key|starts_with`: It is suitable for accurate analysis of key/value structure data, supporting `string/&str/&[u8]` types.
- [x] `split`: Supporting `HashMap` type, eg: [split_example](./tests/test_type_hashmap.rs)
- [x] `if_expr`: Supporting `Option<T>` type, eg: [if_expr_example](./tests/test_modifier_if_expr.rs).
- [x] `encode_with`: custom encode function, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `decode_with`: custom decode function, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `with`: custom encode/decode function, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `with_args`: custom encode/decode function args, eg: [with_args_example](./tests/test_modifier_with_args.rs).
- [x] `encode_value`: value processing expression, eg: `#[jppe(encode_value="length * 2")]`.
- [x] `decode_value`: value processing expression, eg: `#[jppe(decode_value="length / 2")]`.
- [x] `variable_name`: Set integer cache variable, only for decode, eg: [variable_name_example](./tests/test_modifier_variable_name.rs).
- [x] `byte_count=<1|2|4|8>`: Specifies the byte count, automatic decode/encode length or other.
  + [x] `String/&str/&[u8]`: Fetches n byte mapping length in advance, eg: [byte_count](./tests/test_modifier_byte_count.rs).
  + [x] `HexString/HexBytes`: Fetches n byte mapping length in advance, eg: [byte_count](./tests/test_modifier_byte_count.rs).
  + [x] `Enum`: The `byte_count` byte mapping enumeration is taken in advance and encoded through the enumeration inidex, eg: [enum_byte_count](./tests/test_type_enum_byte_count.rs)
- [x] `skip`: Require implement `Default` trait for data type.
- [x] `skip_encode`: Skip encode function.
- [x] `skip_decode`: Require implement `Default` trait for data type.
- [ ] `check_value`
- [ ] `default`

> enum branch

- [x] `branch=<int|variable>`: eg: [branch example](./tests/test_type_enum.rs)
- [x] `branch_option=<variable>`: eg: [branch_option example.rs](./tests/test_modifier_branch_option.rs)
- [x] `branch_default`: eg: [branch_default example](./tests/test_type_enum.rs)
- [x] `branch_bits`: eg: [branch_bits example](./tests/test_type_enum_bits.rs)
- [x] `branch_range`: eg: [branch_range example](./tests/test_type_enum_range.rs)
- [x] `branch_value`: eg: [branch_value example](./tests/test_type_enum_value.rs)

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
- [x] `HashMap`: Support `String/&str/&[u8]`, eg: [hashmap_example](./tests/test_type_hashmap.rs).
- [x] `HashSet<T>`: The `HashSet` type must specify `#[jppe(count=xxx)]` modifier, only supporting decode function, default `count=0`, eg: [hashset_example](./tests/test_type_hashset.rs).
- [x] `MacAddress`: eg: [mac_example](./tests/test_type_mac_address.rs).
- [x] `std::net::Ipv4Addr/Ipv6Addr/IpAddr`: IpAddr type requres specifying the `length=<16|4>` modifier, Otherwise return an error, eg: [ip_address_example](./tests//test_type_ip_address.rs).
- [x] `PpeAddress`: Requres specifying the `length=<16|4|6|usize>` modifier, Otherwise return an error, eg: [ppe_address_example](./tests//test_type_ppe_address.rs).
- [x] `HexString`: eg: [hex_example](./tests/test_type_hex.rs)
- [x] `HexBytes`: eg: [hex_bytes_example](./tests/test_type_hex_bytes.rs)
- [ ] `DateTime`
- [ ] `Bit`


## TODO

- [ ] jnet-rs library.
- [ ] jget-rs library.
- [ ] jdefault-rs library.
- [ ] jfmt-rs library.
