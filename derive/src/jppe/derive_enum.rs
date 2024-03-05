#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::derive_struct::{generate_decode_struct_body, generate_decode_return};
use super::encode::{generate_encode_body, generate_encode_body2};
use super::decode::generate_decode_body2;


#[allow(dead_code)]
pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
}


impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            variants: &self.variants,
            curruent_idx: 0,
        }
    }

    pub fn generate_decode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = "jppe::ByteDecode";

        generator
            .impl_for(crate_name)
            .generate_fn("decode")
            .with_lifetime("da")
            .with_lifetime("db")
            .with_arg("input", "&'da [u8]")
            .with_arg("cattr", "Option<&'db jppe::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&'db jppe::FieldAttrModifiers>")
            .with_return_type("jppe::JResult<&'da [u8], Self>")
            .body(|fn_body| {
                fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;
                Ok(())
            })?;

        Ok(())
    }


    pub fn generate_borrow_decode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = "jppe::BorrowByteDecode";

        generator
        .impl_for(format!("{crate_name}<'a>"))
            // .impl_for_with_lifetimes(crate_name, ["de"])
            .generate_fn("decode")
            .with_lifetime_deps("da", ["a"])
            .with_lifetime("db")
            .with_arg("input", "&'da [u8]")
            .with_arg("cattr", "Option<&'db jppe::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&'db jppe::FieldAttrModifiers>")
            .with_return_type("jppe::JResult<&'da [u8], Self>")
            .body(|fn_body| {
                fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;
                Ok(())
            })?;

        Ok(())
    }

    fn generate_byte_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
        // enum Example {
        //    V0,
        //    V1(u8),
        //    V2(u8, 16),
        //    #[jppe(byteorder="LE")]
        //    V3((u8, u16)),
        //    V4 {
        //        a: u8,
        //        #[jppe(length=2)]
        //        b: u16,
        //    }
        // }

        // match value {
        //     0 => {
        //         return Ok((input, Self::V0));
        //     },
        //     1 => {
        //         let v0: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok((input, Self::V1(v0)));
        //     },
        //     2 => {
        //         let v0: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         let v1: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok((input, Self::V2(v0, v1)));
        //     },
        //     3 => {
        //         let v0: (u8, u16) = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok((input, Self::V3(v0)));
        //     },
        //     4 => {
        //         let a: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         let b: u16 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok((input, Self::V4 { a, b }));
        //     },
        //     _ => {

        //     },
        // }

        if self.variants.is_empty() {
            fn_body.push_parsed("Ok((input, Self {{}}))")?;
        }
        else {
            let code = "
                let value;

                if let Some(fattr) = fattr && let Some(branch) = fattr.branch {
                    value = branch as usize;
                }
                else {
                    return Err(jppe::make_error(input, jppe::ErrorKind::Fail { offset: input.len() }));
                }
            ";
            fn_body.push_parsed(code)?;
            fn_body.push_parsed("match value")?;
            fn_body.group(Delimiter::Brace, |variant_case| {
                for (variant_index, variant) in self.iter_fields() {
                    let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                    variant_case.push_parsed(if attributes.branch_default { "_".to_string() } else { variant_index.to_string() })?;
                    variant_case.puncts("=>");
                    variant_case.group(Delimiter::Brace, |variant_body| {
                        variant_body.push_parsed(attributes.to_code(true, true))?;
                        generate_decode_body2(variant_body, &attributes)?;
                        generate_decode_struct_body(variant_body, crate_name, &variant.fields, true)?;
                        generate_decode_return(variant_body, &variant.fields, Some(variant))?;
                        Ok(())
                    })?;
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    pub fn generate_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode("jppe::ByteEncode", generator)?;
        Ok(())
    }

    pub fn generate_borrow_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode("jppe::BorrowByteEncode", generator)?;
        Ok(())
    }

    fn generate_byte_encode(&self, crate_name: &str, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for(crate_name)
            .generate_fn("encode")
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("input", "&mut Vec<u8>")
            .with_arg("cattr", "Option<&jppe::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jppe::FieldAttrModifiers>")
            .body(|fn_body| {
                // enum Example {
                //    V0,
                //    V1(u8),
                //    V2(u8, 16),
                //    #[jppe(byteorder="LE")]
                //    V3((u8, u16)),
                //    V4 {
                //        a: u8,
                //        #[jppe(length=2)]
                //        b: u16,
                //    }
                // }

                // match self {
                //     Self::V0 => {},
                //     Self::V1(v) => v.encode(input, Some(&cattr), Some(&fattr)),
                //     Self::V2(v1, v2) => {
                //         v1.encode(input, Some(&cattr), Some(&fattr));
                //         v2.encode(input, Some(&cattr), Some(&fattr));
                //     },
                //     Self::V3(v) => v.encode(input, Some(&cattr), Some(&fattr)),
                //     Self::V4 {a, b} => {
                //         a.encode(input, Some(&cattr), Some(&fattr));
                //         b.encode(input, Some(&cattr), Some(&fattr));
                //     }
                // }

                fn_body.push_parsed(self.attributes.to_code(true))?;
                fn_body.push_parsed("match self")?;
                fn_body.group(Delimiter::Brace, |variant_case| {
                    for (mut _variant_index, variant) in self.iter_fields() {
                        let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        if let Some(fields) = &variant.fields {
                            match fields {
                                Fields::Struct(value) => {
                                    let args = value
                                                        .iter()
                                                        .map(|(ident, _v)| ident.to_string())
                                                        .collect::<Vec<String>>()
                                                        .join(", ");

                                    variant_case.push_parsed(format!("Self::{}{{{args}}}", variant.name))?;
                                },
                                Fields::Tuple(value) => {
                                    let args = value
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(index, _v)| format!("v{index}"))
                                                        .collect::<Vec<String>>()
                                                        .join(", ");

                                    variant_case.push_parsed(format!("Self::{}({args})", variant.name))?;
                                },
                            }
                        }
                        else {
                            variant_case.push_parsed(format!("Self::{}", variant.name))?;
                        }

                        variant_case.puncts("=>");

                        variant_case.group(Delimiter::Brace, |variant_body| {
                            variant_body.push_parsed(attributes.to_code(true, false))?;
                            generate_encode_body2(variant_body, &attributes, false)?;

                            if let Some(fields) = &variant.fields {
                                match fields {
                                    Fields::Struct(value) => {
                                        for (ident, field) in value {
                                            let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            variant_body.push_parsed(attributes.to_code(false, true))?;
                                    
                                            if !attributes.untake {
                                                generate_encode_body(variant_body, &attributes, false)?;
                                                // jppe::ByteEncode::encode(&value, input, None, None);
                                                // variant_body.push_parsed(format!("{ident}.encode(input, Some(&cattr), Some(&fattr));"))?;    
                                                variant_body.push_parsed(format!("{crate_name}::encode({ident}, input, Some(&cattr), Some(&fattr));"))?;
                                            }
                                        }
                                    },
                                    Fields::Tuple(value) => {
                                        for (index, field) in value.iter().enumerate() {
                                            let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            if attributes.is_use {
                                                variant_body.push_parsed(attributes.to_code(false, false))?;
                                            }

                                            if !attributes.untake {
                                                generate_encode_body(variant_body, &attributes, false)?;
                                                // variant_body.push_parsed(format!("v{index}.encode(input, Some(&cattr), Some(&fattr));"))?;    
                                                variant_body.push_parsed(format!("{crate_name}::encode(v{index}, input, Some(&cattr), Some(&fattr));"))?;
                                            }
                                        }
                                    },
                                }
                            }
                            Ok(())
                        })?;

                        variant_case.puncts(",");
                    }

                    Ok(())
                })?;

                Ok(())
            })?;
        Ok(())
    }
}


struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
    curruent_idx: usize,
}


impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (TokenTree, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        // let mut idx = self.idx;
        let variant = self.variants.get(self.idx)?;

        if let Some(value) = &variant.value {
            // Literal
            let val_string = value.to_string();

            if val_string.starts_with("0x") {
                self.curruent_idx = usize::from_str_radix(&val_string[2..], 16).unwrap();
            }
            else {
                self.curruent_idx = val_string.parse::<usize>().unwrap();
            }
        }

        let tokens = TokenTree::Literal(Literal::usize_suffixed(self.curruent_idx));

        self.curruent_idx += 1;
        self.idx += 1;

        Some((tokens, variant))
    }
}
