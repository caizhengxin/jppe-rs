# jppe-rs

[![Crates.io](https://img.shields.io/crates/v/jppe)](https://crates.io/crates/jppe)
[![Crates.io](https://img.shields.io/crates/d/jppe)](https://crates.io/crates/jppe)
[![License](https://img.shields.io/crates/l/jppe)](LICENSE)

这是一个基于Rust实现的字节流结构化序列化/反序列化通用库，可以应用于网络数据包解析、网络数据包组包、网络通信、文件内容解析等，觉得不错的小伙伴们请点个赞👍~

## 安装

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

### 简单例子

```rust
#![feature(let_chains)]
use jppe::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    // 这里是指定动态长度，也可以指定固定数值，比如：`#[jppe(length=3)]`
    // 还可以不指定`length`, 指定`byte_count=<1|2|4|8>`表示提前取走几个字节根据字节序转为长度数值
    #[jppe(length="length")]
    pub value: String,
    pub cmd: u8,
    // 这里指定了branch, 表示根据cmd的值进行枚举类型(enum)模式匹配, 同样也可以指定`byte_count`修饰符
    #[jppe(branch="cmd")]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum SimpleExampleBody {
    Read {
        address: u8,
    } = 1,                    // 这里表示当前面的`cmd`字段为1，则会进入该分支解析
    Write {
        address: u8,
        value: [u8; 3],
    },                        // 这里如果不指定，默认是递增的关系为2
    #[jppe(enum_default)]
    Unknown,                  // 这里由于指定了默认分支，所以会被映射为`_ => { ... }`, 如果没有指定，Unknown序号为3，其他则会返回解析错误
}


fn main() {
    let input = b"\x00\x03\x31\x32\x33\x01\x05";
    let (input_remain, value) = jppe::decode::<SimpleExample>(input).unwrap();
    assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(input_remain.is_empty(), true);
    assert_eq!(jppe::encode(value), input);
}
```

### 网络例子

- [TCP通信例子](./examples/socket_example.rs)
- [以太网解析例子](./examples/ethernet_example.rs)
- [IPv4解析例子](./examples/ipv4_example.rs)
- [TCP解析例子](./examples/tcp_example.rs)
- [HTTP解析例子](./examples/http_example.rs)
- [解析例子](./examples/parse_example.rs): 包含Ethernet/IPv4/TCP/UDP

## 常见功能

### ContainerAttrModifiers

主要用于修饰某个struct/enum全局定义，表示struct/enum里面的字段都遵循，也可以通过`FieldAttrModifiers`修饰单个内容。

> 通用修饰符

- [x] `byteorder=<"BE"|"LE">`: 这是struct/enum类型全局字节序，BE(大端字节序)/LE(小端字节序), eg: `#[jppe(byteorder="LE")]`.
- [x] `encode_with`: 自定义encode函数, eg: [with_encode_example](./tests/test_modifier_with.rs).
- [x] `decode_with`: 自定义decode函数, eg: [with_decode_example](./tests/test_modifier_with.rs).
- [x] `with`: 自定义encode/decode函数, eg: [with_encode_example](./tests/test_modifier_with.rs).
- [x] `get_variable_name`: 获取缓存变量, 必须配合`variable_name`使用，可以用于不用struct或enum类型传递，目前仅仅支持`decode`, eg: [variable_name_example](./tests/test_modifier_variable_name.rs).

> 枚举分支修饰符

- [ ] `branch_enum`

### FieldAttrModifiers

主要用于修饰struct/enum里面某个字段内容

- [x] `byteorder=<"BE"|"LE">`: 这是struct/enum类型局部字段字节序，BE(大端字节序)/LE(小端字节序), eg: `#[jppe(byteorder="LE")]`
- [x] `length=<num|variable>`: 数据长度, 支持`int/&str/String/&[u8]`类型, eg: [length_example](./tests/test_modifier_length.rs).
- [x] `offset=<num|variable>`: 字节流偏移，表示跳过几个字节.
- [x] `count==<num|variable>`: 数据条目数量, 支持`Vec/HashMap`类型.
- [x] `full=<int>`: 主要用于encode填充值, 默认为0, 常常用于offset偏移之后进行encode编码填充.
- [x] `untake`: 表示解析内容, 但是不取走字节, 后面的解析任然可以读取该字节.
- [x] `linend|end_with=<string|bytes>`: 指定结束位置, 支持`String/&str/&[u8]/HashMap`等类型.
- [x] `key|starts_with`: 指定精准匹配关键字, 必须配合`linend`使用, 支持`string/&str/&[u8]`等类型.
- [x] `split`: 指定分隔符, 常常用于`Key: Value`这种内容, 支持`HashMap`类型, eg: [split_example](./tests/test_type_hashmap.rs)
- [x] `if_expr <bool expr>`: 指定if表达式, 支持`Option<T>`类型, eg: [if_expr_example](./tests/test_modifier_if_expr.rs).
- [x] `encode_with`: 自定义encode函数, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `decode_with`: 自定义decode函数, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `with`: 自定义encode/decode函数, eg: [with_example](./tests/test_modifier_with.rs).
- [x] `with_args`: 自定义encode/decode函数参数, eg: [with_args_example](./tests/test_modifier_with_args.rs).
- [x] `encode_value`: value处理表达式, eg: `#[jppe(encode_value="length * 2")]`.
- [x] `decode_value`: value处理表达式, eg: `#[jppe(decode_value="length / 2")]`.
- [x] `variable_name`: 指定整型类型缓存变量, 目前仅仅支持`decode`函数, eg: [variable_name_example](./tests/test_modifier_variable_name.rs).
- [x] `byte_count=<1|2|4|8>`: 指定`byte_count`字节数量, 会取走对应字节映射数字, 常常用于下面类型:
  + [x] `String/&str/&[u8]`: 提前取n个字节映射长度, eg: [byte_count](./tests/test_modifier_byte_count.rs).
  + [x] `HexString/HexBytes`: 提前取n个字节映射长度, eg: [byte_count](./tests/test_modifier_byte_count.rs).
  + [x] `Enum`: 提前取n个字节映射枚举索引, eg: [enum_byte_count](./tests/test_type_enum_byte_count.rs)
- [x] `skip`: 数据类型需要实现`Default`trait.
- [x] `skip_encode`: 跳过encode函数.
- [x] `skip_decode`: 数据类型需要实现`Default`trait.
- [ ] `check_value`：主要用于检查结果是否正常, 异常会返回错误
- [x] `default`: eg: [default example](./crates/jdefault-rs/tests/test_jppe.rs)

> enum branch

- [x] `branch`: 指定枚举(Enum)类型分支条件: [branch example](./tests/test_type_enum.rs)
- [x] `branch`: 指定枚举(Enum)类型分支条件: [branch_option example](./tests/test_modifier_branch_option.rs)
- [x] `branch_default|enum_default`: 指定枚举(Enum)类型默认值, eg: [branch_default example](./tests/test_type_enum.rs)
- [x] `branch_bits`: 指定枚举(Enum)分支判断条件, eg: [branch_bits example](./tests/test_type_enum_bits.rs)
- [x] `branch_range`: 指定枚举(Enum)分支判断条件范围, eg: [branch_range example](./tests/test_type_enum_range.rs)
- [x] `branch_value`: 指定枚举(Enum)分支判断条件, eg: [branch_value example](./tests/test_type_enum_value.rs)

### 数据类型

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
- [x] `HashMap`: 支持`String/&str/&[u8]`类型, 可以指定`#[jppe(count=xxx)]`修饰符表示解析数量, 默认是`50`, eg: [hashmap_example](./tests/test_type_hashmap.rs).
- [x] `HashSet<T>`: 需要指定`#[jppe(count=xxx)]`修饰符表示解析数量, 仅仅支持decode函数, 默认`count=0`, eg: [hashset_example](./tests/test_type_hashset.rs).
- [x] `MacAddress`: 内部实现MacAddress类型, eg: [mac_example](./tests/test_type_mac_address.rs).
- [x] `std::net::Ipv4Addr`: Ipv4类型.
- [x] `std::net::Ipv6Addr`: Ipv6类型.
- [x] `std::net::IpAddr`: 必须指定`length=<16|4>`修饰符, 否则返回错误, eg: [ip_address_example](./tests//test_type_ip_address.rs).
- [x] `PpeAddress`: 内部实现PpeAddress类型, 包含`IPv4/IPv6/Mac/Usize`地址类型, 必须指定`length=<16|4|6|usize>`修饰符, 否则返回错误, eg: [ppe_address_example](./tests//test_type_ppe_address.rs).
- [x] `HexString`: 内部实现`HexString`类型, eg: [hex_example](./tests/test_type_hex.rs)
- [x] `HexBytes`: 内部实现`HexBytes`类型, `HexBytes`引用类型, eg: [hex_bytes_example](./tests/test_type_hex_bytes.rs)
- [ ] `DateTime`
- [ ] `Bit`

## TODO

- [ ] `jnet-rs`: 基于`jppe`实现常见网络协议结构化定义.
- [x] `jget-rs`: 通过修饰符暴露`pub fn get_xxxx(&self) -> xxxx`函数，避免手动定义.
- [x] `jdefault-rs`：通过修饰符设置默认值, 可以集合`jppe`结构化定义实现默认值编码功能.
- [ ] `jfmt-rs`: 主要用于格式化`jppe`定义结构化内容数据.
