pub mod errors;
mod traits;

pub use errors::*;
use crate::ByteOrder;


pub fn input_take<T>(input: &[T], length: usize) -> JResult<&[T], &[T]> {
    let mut input = input;

    if let Some(value) = input.take(..length) {
        return Ok((input, value));
    }

    Err(make_error(input, ErrorKind::InvalidByteLength { offset: input.len() }))
}


pub fn parse_subsequence<'a, 'b,  T>(input: &'a [T], needle: &'b [T], is_save_needle: bool) -> JResult<&'a [T], &'a [T]>
where
    for<'c> &'c [T]: PartialEq,
    // T: std::fmt::Debug,
{
    let needle_len = needle.len();
    let input_len = input.len();
    let mut input = input;

    if needle_len > input_len {
        return Err(make_error(input, ErrorKind::InvalidByteLength { offset: input_len }));
    }

    if let Some(index) = input
        .windows(needle_len)
        .position(|window| window == needle) &&
       let Some(value) = input.take(..index + needle_len)
    {
        return Ok((input, if is_save_needle {value} else {&value[..index]}));
    }

    Err(make_error(input, ErrorKind::SubSequence { offset: input_len }))
}


pub fn parse_subsequences<'a, T>(input: &'a [T], needles: &'a [&'a [T]], is_save_needle: bool) -> JResult<&'a [T], &'a [T]>
where
    for<'b> &'b [T]: PartialEq,
    // T: std::fmt::Debug,
{
    let input_len = input.len();

    for needle in needles {
        if let Ok((input, value)) = parse_subsequence(input, needle, is_save_needle) {
            return Ok((input, value));
        }
    }

    Err(make_error(input, ErrorKind::SubSequence { offset: input_len }))
}


pub fn parse_u128<'a>(input: &'a [u8], byteorder: &crate::ByteOrder, byte_count: u8) -> JResult<&'a [u8], u128>
{
    let input_len = input.len();
    let mut value: u128 = 0;

    if input_len < byte_count.into() {
        return Err(make_error(input, ErrorKind::InvalidByteLength { offset: input_len }));
    }

    match byteorder {
        ByteOrder::Be => {
            for byte in input.iter().take(byte_count.into()) {
                value = (value << 8) + *byte as u128;
            }
        },
        ByteOrder::Le => {
            for (index, byte) in input.iter().enumerate().take(byte_count.into()) {
                value += (*byte as u128) << (8 * index);
            }
        }
    }

    Ok((&input[byte_count.into()..], value))
}
