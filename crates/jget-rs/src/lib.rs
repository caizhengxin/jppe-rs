#![feature(let_chains)]

#[cfg(feature = "jget_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jget_derive;

#[cfg(feature = "jget_derive")]
pub use jget_derive::Jget;
