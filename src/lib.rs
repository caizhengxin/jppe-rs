#![feature(let_chains)]
#![feature(slice_take)]
#![feature(byte_slice_trim_ascii)]
#![feature(extract_if)]
#![feature(slice_internals)]
#![feature(const_trait_impl)]
#![feature(ip_bits)]
#![feature(array_chunks)]

#[cfg(feature = "jppe_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jppe_derive;

#[cfg(feature = "jppe_derive")]
pub use jppe_derive::{ByteDecode, ByteEncode, BorrowByteDecode, BorrowByteEncode};

/// jankincai

mod decode;
mod encode;
mod parser;
pub mod fields;
mod byteorder;
mod modifiers;
mod input;

pub use byteorder::ByteOrder;
pub use decode::*;
pub use encode::*;
pub use errors::*;
pub use parser::*;
pub use modifiers::*;
pub use input::*;

pub mod prelude {
    pub use jkcenum::FromInt;
    pub use std::str::FromStr;
    pub use crate::parser::*;
    pub use crate::byteorder::ByteOrder;
    // pub use crate::decode::*;
    // pub use crate::encode::*;
    pub use crate::fields::*;
    pub use crate::modifiers::*;
    pub use crate::input::*;
    pub use crate::{encode, decode};
}


#[inline]
pub fn decode<'a, T: ByteDecode>(input: &'a [u8]) -> JResult<&'a [u8], T> {
    T::decode(input, None, None)
}


#[inline]
pub fn encode<'a, T: ByteEncode>(t: T) -> Vec<u8> {
    let mut buf = Vec::new();

    t.encode(&mut buf, None, None);

    buf
}


#[inline]
pub fn decode_borrow<'a, T: BorrowByteDecode<'a>>(input: &'a [u8]) -> JResult<&'a [u8], T> {
    T::decode(input, None, None)
}


#[inline]
pub fn encode_borrow<'a, T: BorrowByteEncode>(t: T) -> Vec<u8> {
    let mut buf = Vec::new();

    t.encode(&mut buf, None, None);

    buf
}
