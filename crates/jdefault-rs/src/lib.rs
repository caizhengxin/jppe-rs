// #![feature(let_chains)]
extern crate proc_macro;

mod jdefault;

use jdefault::attribute::ContainerAttributes;
use jdefault::derive_enum;
use jdefault::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


#[proc_macro_derive(Jdefault, attributes(jd))]
pub fn derive_jdefault(input: TokenStream) -> TokenStream {
    derive_jdefault_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jdefault_inner(input: TokenStream) -> Result<TokenStream> {
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
            }.generate_jdefault(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_jdefault(&mut generator)?;
        }
    }

    generator.export_to_file("jdefault", "Jdefault");
    generator.finish()
}
