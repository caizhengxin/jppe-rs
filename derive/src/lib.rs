//!
//! ```
//! 
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
extern crate proc_macro;

mod jppe;

use jppe::attribute::ContainerAttributes;
use jppe::derive_enum;
use jppe::derive_struct;

use proc_macro::TokenStream;
use quote::ToTokens;
use virtue::prelude::*;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(ByteDecode, attributes(jppe))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    derive_decode_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_decode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
                lifetimes: None,
            }.generate_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
                lifetimes: None,
            }
            .generate_decode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Decode");
    generator.finish()
}


#[proc_macro_derive(BorrowByteDecode, attributes(jppe))]
pub fn derive_borrow_decode(input: TokenStream) -> TokenStream {
    let input_tmp = input.clone();
    let derive_input = parse_macro_input!(input_tmp as DeriveInput);
    let lifetimes = derive_input.generics.to_token_stream().to_string();

    derive_borrow_decode_inner(input, lifetimes).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_borrow_decode_inner(input: TokenStream, lifetimes: String) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
                lifetimes: if lifetimes.is_empty() { None } else {Some(lifetimes)},
            }.generate_borrow_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
                lifetimes: if lifetimes.is_empty() { None } else {Some(lifetimes)},
            }
            .generate_borrow_decode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Decode");
    generator.finish()
}


#[proc_macro_derive(ByteEncode, attributes(jppe))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    derive_encode_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
                lifetimes: None,
            }.generate_encode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
                lifetimes: None,
            }
            .generate_encode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Encode");
    generator.finish()
}


#[proc_macro_derive(BorrowByteEncode, attributes(jppe))]
pub fn derive_borrow_encode(input: TokenStream) -> TokenStream {
    derive_borrow_encode_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_borrow_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
                lifetimes: None,
            }.generate_borrow_encode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
                lifetimes: None,
            }
            .generate_borrow_encode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Encode");
    generator.finish()
}
