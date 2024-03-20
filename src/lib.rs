//!
//! ```
//! #![feature(let_chains)]
//! use jppe::{ByteEncode, ByteDecode};
//! use jppe_derive::{ByteEncode, ByteDecode};
//!
//!
//! #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
//! pub struct SimpleExample {
//!     pub length: u16,
//!     #[jppe(length="length")]
//!     pub value: String,
//!     pub cmd: u8,
//!     #[jppe(branch="cmd")]
//!     pub body: SimpleExampleBody,
//! }
//! 
//! 
//! #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
//! #[repr(u8)]
//! pub enum SimpleExampleBody {
//!     Read {
//!         address: u8,
//!     } = 1,
//!     Write {
//!         address: u8,
//!         value: [u8; 3],
//!     },
//!     #[jppe(enum_default)]
//!     Unknown, 
//! }
//! 
//! 
//! fn main() {
//!     let input = b"\x00\x03\x31\x32\x33\x01\x05";
//!     let (input_remain, value) = jppe::decode::<SimpleExample>(input).unwrap();
//!     assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
//!     assert_eq!(input_remain.is_empty(), true);
//!     assert_eq!(jppe::encode(value), input);
//! }
//! ```
#![feature(let_chains)]
#![feature(slice_take)]
#![feature(byte_slice_trim_ascii)]
#![feature(extract_if)]
#![feature(slice_internals)]
#![feature(const_trait_impl)]
#![feature(ip_bits)]
#![feature(array_chunks)]

#[cfg(feature = "jppe_derive")]
extern crate jppe_derive;
#[cfg(feature = "jppe_derive")]
pub use jppe_derive::{ByteDecode, ByteEncode, BorrowByteDecode, BorrowByteEncode};

#[cfg(feature = "jget")]
extern crate jget;
#[cfg(feature = "jget")]
pub use jget::Jget;

#[cfg(feature = "jdefault_derive")]
extern crate jdefault_derive;
#[cfg(feature = "jdefault_derive")]
pub use jdefault_derive::Jdefault;


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


/// Decode byte stream
/// 
/// # Examples:
/// 
/// ```
/// #![feature(let_chains)]
/// use jppe_derive::{ByteDecode};
/// 
/// #[derive(Debug, PartialEq, Eq, ByteDecode)]
/// pub struct SimpleExample {
///     pub length: u8,
///     #[jppe(length="length")]
///     pub data: String,
/// }
/// let (input, value) = jppe::decode::<SimpleExample>(b"\x02\x31\x32").unwrap();
/// assert_eq!(value, SimpleExample { length: 2, data: "12".to_string() });
/// assert_eq!(input.is_empty(), true);
/// ```
#[inline]
pub fn decode<'a, T: ByteDecode>(input: &'a [u8]) -> JResult<&'a [u8], T> {
    T::decode(input, None, None)
}


/// Encode byte stream
/// 
/// # Examples:
/// 
/// ```
/// #![feature(let_chains)]
/// use jppe_derive::{ByteEncode};
/// 
/// #[derive(Debug, PartialEq, Eq, ByteEncode)]
/// pub struct SimpleExample {
///     pub length: u8,
///     #[jppe(length="length")]
///     pub data: String,
/// }
/// let value = SimpleExample { length: 2, data: "12".to_string() };
/// assert_eq!(jppe::encode(value), b"\x02\x31\x32");
/// ```
#[inline]
pub fn encode<'a, T: ByteEncode>(t: T) -> Vec<u8> {
    let mut buf = Vec::new();

    t.encode(&mut buf, None, None);

    buf
}


/// Decode byte stream
/// 
/// # Examples:
/// 
/// ```
/// #![feature(let_chains)]
/// use jppe_derive::{BorrowByteDecode};
/// 
/// #[derive(Debug, PartialEq, Eq, BorrowByteDecode)]
/// pub struct SimpleExample<'a> {
///     pub length: u8,
///     #[jppe(length="length")]
///     pub data: &'a [u8],
/// }
/// let (input, value) = jppe::decode_borrow::<SimpleExample>(b"\x02\x03\x01").unwrap();
/// assert_eq!(value, SimpleExample { length: 2, data: b"\x03\x01" });
/// assert_eq!(input.is_empty(), true);
/// ```
#[inline]
pub fn decode_borrow<'a, T: BorrowByteDecode<'a>>(input: &'a [u8]) -> JResult<&'a [u8], T> {
    T::decode(input, None, None)
}


/// Encode byte stream
/// 
/// # Examples:
/// 
/// ```
/// #![feature(let_chains)]
/// use jppe_derive::{BorrowByteEncode};
/// 
/// #[derive(Debug, PartialEq, Eq, BorrowByteEncode)]
/// pub struct SimpleExample<'a> {
///     pub length: u8,
///     #[jppe(length="length")]
///     pub data: &'a [u8],
/// }
/// let value = SimpleExample { length: 2, data: b"12" };
/// assert_eq!(jppe::encode_borrow(value), b"\x02\x31\x32");
/// ```
#[inline]
pub fn encode_borrow<'a, T: BorrowByteEncode>(t: T) -> Vec<u8> {
    let mut buf = Vec::new();

    t.encode(&mut buf, None, None);

    buf
}
