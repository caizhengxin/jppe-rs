#![feature(let_chains)]
extern crate proc_macro;

mod jppe;

use jppe::attribute::ContainerAttributes;
use jppe::derive_enum;
use jppe::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


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
            }.generate_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_decode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Decode");
    generator.finish()
}


#[proc_macro_derive(BorrowByteDecode, attributes(jppe))]
pub fn derive_borrow_decode(input: TokenStream) -> TokenStream {
    derive_borrow_decode_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_borrow_decode_inner(input: TokenStream) -> Result<TokenStream> {
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
            }.generate_borrow_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
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
            }.generate_encode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
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
            }.generate_borrow_encode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_borrow_encode(&mut generator)?;
        }
    }

    generator.export_to_file("jppe", "Encode");
    generator.finish()
}
