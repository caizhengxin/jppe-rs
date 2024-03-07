#![feature(let_chains)]
#![feature(slice_take)]
#![feature(byte_slice_trim_ascii)]
#![feature(extract_if)]
#![feature(slice_internals)]
#![feature(const_trait_impl)]

#[cfg(feature = "jppe_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jppe_derive;

#[cfg(feature = "jppe_derive")]
pub use jppe_derive::{ByteDecode, ByteEncode, BorrowByteDecode, BorrowByteEncode};

mod decode;
mod encode;
mod parser;
pub mod fields;
mod byteorder;
mod modifiers;
mod find_substring;

pub use byteorder::ByteOrder;
pub use decode::*;
pub use encode::*;
pub use errors::*;
pub use parser::*;
pub use modifiers::*;

pub mod prelude {
    pub use jkcenum::FromInt;
    pub use std::str::FromStr;
    // pub use crate::parser::*;
    pub use crate::byteorder::ByteOrder;
    // pub use crate::decode::*;
    // pub use crate::encode::*;
    pub use crate::fields::*;
    pub use crate::modifiers::*;
}


#[derive(Debug, Clone)]
pub enum StringOrUsize {
    String(String),
    Usize(usize),
}
