use crate::{ByteOrder, JResult};
use crate::errors::{make_error, ErrorKind};


pub trait InputTrait<'a> {
    fn find_subsequence<'b>(&self, needle: &'b [u8], is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]>;

    fn find_subsequences<'b>(&self, needles: &'b [&'b [u8]], is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]>;

    fn find_subsequences2<'b>(&self, needles: &'b Vec<Vec<u8>>, is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]>;

    fn input_take(&self, length: usize) -> JResult<&'a [u8], &'a [u8]>;

    fn to_bits(&self, byteorder: ByteOrder, byte_count: u8) -> JResult<&'a [u8], u128>;

    fn to_le_bits(&self, byte_count: u8) -> JResult<&'a [u8], u128>;

    fn to_be_bits(&self, byte_count: u8) -> JResult<&'a [u8], u128>;

    #[inline]
    fn to_bits_usize(&self, byteorder: ByteOrder, byte_count: u8) -> JResult<&'a [u8], usize> {
        let (input, value) = self.to_bits(byteorder, byte_count)?;
        Ok((input, value as usize))
    }

    #[inline]
    fn to_le_bits_usize(&self, byte_count: u8) -> JResult<&'a [u8], usize> {
        let (input, value) = self.to_le_bits(byte_count)?;
        Ok((input, value as usize))
    }

    #[inline]
    fn to_be_bits_usize(&self, byte_count: u8) -> JResult<&'a [u8], usize> {
        let (input, value) = self.to_be_bits(byte_count)?;
        Ok((input, value as usize))
    }
}


impl<'a> InputTrait<'a> for &'a [u8] {
    fn find_subsequence<'b>(&self, needle: &'b [u8], is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]> {
        let input = *self;
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

        Err(make_error(input, ErrorKind::InvalidByteLength { offset: 0 }))
    }

    fn find_subsequences<'b>(&self, needles: &'b [&'b [u8]], is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]> {
        let input = *self;
        let input_len = input.len();

        for needle in needles {
            if let Ok((input, value)) = self.find_subsequence(needle, is_save_needle) {
                return Ok((input, value));
            }
        }
    
        Err(make_error(input, ErrorKind::SubSequence { offset: input_len }))      
    }

    fn find_subsequences2<'b>(&self, needles: &'b Vec<Vec<u8>>, is_save_needle: bool) -> JResult<&'a [u8], &'a [u8]> {
        let input = *self;
        let input_len = input.len();

        for needle in needles {
            if let Ok((input, value)) = self.find_subsequence(needle, is_save_needle) {
                return Ok((input, value));
            }
        }
    
        Err(make_error(input, ErrorKind::SubSequence { offset: input_len }))      
    }

    fn input_take(&self, length: usize) -> JResult<&'a [u8], &'a [u8]> {
        let mut input = *self;

        if let Some(value) = input.take(..length) {
            return Ok((input, value));
        }
    
        Err(make_error(input, ErrorKind::InvalidByteLength { offset: input.len() }))
    }

    fn to_bits(&self, byteorder: ByteOrder, byte_count: u8) -> JResult<&'a [u8], u128> {
        let input = *self;
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

    #[inline]
    fn to_be_bits(&self, byte_count: u8) -> JResult<&'a [u8], u128> {
        self.to_bits(ByteOrder::Be, byte_count)
    }

    #[inline]
    fn to_le_bits(&self, byte_count: u8) -> JResult<&'a [u8], u128> {
        self.to_bits(ByteOrder::Le, byte_count)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_trait() {
        let input = &b"\x00\x01\x00\x02"[..];
        let (input, value) = input.to_be_bits(2).unwrap();
        assert_eq!(value, 0x0001);
        assert_eq!(input, b"\x00\x02");

        let (input, value) = input.to_le_bits(2).unwrap();
        assert_eq!(value, 0x0200);
        assert_eq!(input.is_empty(), true);

        let input = &b"\x00\x01\x00\x02"[..];
        let (input, value) = input.input_take(2).unwrap();
        assert_eq!(value, b"\x00\x01");
        assert_eq!(input, b"\x00\x02");

        let input = &b"\x00\x01\x00\x02"[..];
        let (input, value) = input.find_subsequence(b"\x01\x00", false).unwrap();
        assert_eq!(value, b"\x00");
        assert_eq!(input, b"\x02");

        let input = &b"\x00\x01\x00\x02"[..];
        let (input, value) = input.find_subsequence(b"\x01\x00", true).unwrap();
        assert_eq!(value, b"\x00\x01\x00");
        assert_eq!(input, b"\x02");
    }
}