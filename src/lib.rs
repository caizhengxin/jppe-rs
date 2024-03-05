#![feature(let_chains)]
#![feature(slice_take)]
#![feature(byte_slice_trim_ascii)]
#![feature(extract_if)]
#![feature(slice_internals)]

#[cfg(feature = "jppe_rs_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jppe_rs_derive;

#[cfg(feature = "jppe_rs_derive")]
pub use jppe_rs_derive::jppe_rs;


mod decode;
mod encode;
mod parser;
mod fields;
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
    // pub use crate::parser::*;
    pub use crate::byteorder::ByteOrder;
    // pub use crate::decode::*;
    // pub use crate::encode::*;
    pub use crate::modifiers::*;
    pub use jkcenum::FromInt;
    pub use std::str::FromStr;
}


#[derive(Debug, Clone)]
pub enum StringOrUsize {
    String(String),
    Usize(usize),
}
