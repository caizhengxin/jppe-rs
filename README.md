# jppe-rs

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
