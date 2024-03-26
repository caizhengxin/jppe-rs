#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::parse::AttrValue;
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
                self.generate_decode_body(crate_name, fn_body)?;

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
                self.generate_decode_body(crate_name, fn_body)?;
                Ok(())
            })?;

        Ok(())
    }

    #[inline]
    fn generate_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
        if let Some(func) = &self.attributes.with_decode {
            fn_body.push_parsed(format!("{func}(input, cattr, fattr)"))?;
        }
        else if let Some(func) = &self.attributes.with {
            fn_body.push_parsed(format!("{func}::decode(input, cattr, fattr)"))?;
        }
        else {
            fn_body.push_parsed(self.attributes.to_code(false))?;

            if let Some(value) = &self.attributes.get_variable_name && let AttrValue::List(variable_names) = value {
                for variable_name in variable_names {
                    let variable_name_str = variable_name.to_string();

                    fn_body.push_parsed(format!("let {variable_name_str} = if let Some(cr) = cattr && let Some(value) = cr.variable_name.borrow().get(&\"{variable_name_str}\".to_string()) {{*value}} else {{0}};"))?;
                }
            }

            self.generate_byte_decode_body(crate_name, fn_body)?;    
        }

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
                let mut input = input;

                if let Some(fr) = fattr {
                    if let Some(branch) = fr.branch {
                        value = (branch) as usize;
                    }
                    else if let Some(byte_count) = fr.byte_count {
                        (input, value) = jppe::parse_usize(input, &jppe::get_byteorder(cattr, fattr), byte_count as u8)?;
                    }
                    else {
                        // (input, value) = jppe::parse_usize(input, &jppe::ByteOrder::Be, 1)?;
                        return Err(jppe::make_error(input, jppe::ErrorKind::InvalidByteLength { offset: input.len() }));
                    }
                }
                else {
                    // (input, value) = jppe::parse_usize(input, &jppe::ByteOrder::Be, 1)?;
                    return Err(jppe::make_error(input, jppe::ErrorKind::InvalidByteLength { offset: input.len() }));
                }
            ";
            fn_body.push_parsed(code)?;
            fn_body.push_parsed("match value")?;
            fn_body.group(Delimiter::Brace, |variant_case| {
                let mut branch_default = false;

                for (variant_index, variant) in self.iter_fields() {
                    let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                    if attributes.branch_default {
                        variant_case.push_parsed("_")?;
                        branch_default = true;
                    }
                    else if let Some(branch_bits) = &attributes.branch_bits {
                        // match value {
                        //     value if (value & 0x80) == 0x80 => {},
                        //     value if (value & 0x01) == 0x80 => {},
                        //     0x02 => {},
                        //     _ => {},
                        // }

                        if let Some(branch_bits_value) = &attributes.branch_bits_value {
                            variant_case.push_parsed(format!("value if (value & {branch_bits}) == {branch_bits_value}"))?;
                        }
                        else {
                            variant_case.push_parsed(format!("value if (value & {branch_bits}) == {branch_bits}"))?;
                        }
                    }
                    else if let Some(branch_value) = &attributes.branch_value {
                        variant_case.push_parsed(branch_value.to_string())?;
                    }
                    else if let Some(branch_range) = &attributes.branch_range {
                        variant_case.push_parsed(branch_range.to_string())?;
                    }
                    else {
                        variant_case.push_parsed( variant_index.to_string())?;
                    }

                    variant_case.puncts("=>");
                    variant_case.group(Delimiter::Brace, |variant_body| {
                        variant_body.push_parsed(attributes.to_code(true, true))?;
                        generate_decode_body2(variant_body, &attributes)?;
                        generate_decode_struct_body(variant_body, crate_name, &variant.fields, &self.attributes, true)?;
                        generate_decode_return(variant_body, &variant.fields, Some(variant))?;
                        Ok(())
                    })?;
                }

                if !branch_default {
                    variant_case.push_parsed("_ => Err(jppe::make_error(input, jppe::ErrorKind::InvalidByteLength { offset: input.len() }))")?;
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    pub fn generate_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode_body("jppe::ByteEncode", generator)?;
        Ok(())
    }

    pub fn generate_borrow_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode_body("jppe::BorrowByteEncode", generator)?;
        Ok(())
    }

    fn generate_byte_encode_body(&self, crate_name: &str, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for(crate_name)
            .generate_fn("encode")
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("input", "&mut Vec<u8>")
            .with_arg("cattr", "Option<&jppe::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jppe::FieldAttrModifiers>")
            .body(|fn_body| {

                if let Some(func) = &self.attributes.with_encode {
                    fn_body.push_parsed(format!("{func}(input, cattr, fattr, self)"))?;
                    return Ok(());
                }
                else if let Some(func) = &self.attributes.with {
                    fn_body.push_parsed(format!("{func}::encode(input, cattr, fattr, self)"))?;
                    return Ok(());
                }

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

                if let Some(value) = &self.attributes.get_variable_name && let AttrValue::List(variable_names) = value {
                    for variable_name in variable_names {
                        let variable_name_str = variable_name.to_string();
    
                        fn_body.push_parsed(format!("let {variable_name_str} = if let Some(cr) = cattr && let Some(value) = cr.variable_name.borrow().get(&\"{variable_name_str}\".to_string()) {{*value}} else {{0}};"))?;
                    }
                }        

                fn_body.push_parsed(self.attributes.to_code(true))?;
                fn_body.push_parsed("match self")?;
                fn_body.group(Delimiter::Brace, |variant_case| {
                    for (variant_index, variant) in self.iter_fields() {
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

                            let code = format!("        
                                if let Some(fr) = fattr {{
                                    if let Some(branch) = fr.branch {{
                                    }}
                                    else if let Some(byte_count) = fr.byte_count {{
                                        input.extend(jppe::int_to_vec({variant_index}, byte_count, &jppe::get_byteorder(cattr, fattr)))
                                    }}
                                    // else {{
                                    //     input.push({variant_index} as u8);
                                    // }}
                                }}
                                // else {{
                                //     input.push({variant_index} as u8);
                                // }}
                            ");
                            variant_body.push_parsed(code)?;

                            if let Some(fields) = &variant.fields {
                                match fields {
                                    Fields::Struct(value) => {
                                        for (ident, field) in value {
                                            let mut attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            attributes.get_variable_name = self.attributes.get_variable_name.clone();

                                            if attributes.is_use {
                                                variant_body.push_parsed(attributes.to_code(false, true))?;
                                            }

                                            generate_encode_body(variant_body, &attributes, crate_name, &ident.to_string(), false)?;
                                        }
                                    },
                                    Fields::Tuple(value) => {
                                        for (index, field) in value.iter().enumerate() {
                                            let mut attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            attributes.get_variable_name = self.attributes.get_variable_name.clone();

                                            if attributes.is_use {
                                                variant_body.push_parsed(attributes.to_code(false, false))?;
                                            }

                                            generate_encode_body(variant_body, &attributes, crate_name, &format!("v{index}"), false)?;
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
