#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::parse::Fields;
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::decode::generate_decode_body;
use super::encode::generate_encode_body;


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}


pub fn get_field_type(field: &UnnamedField) -> String {
    let field_type: Vec<String> = field.r#type.iter().map(|f|f.to_string()).collect();
    let field_type = field_type.iter().map(|v| { if v == "'" { v.to_string() } else { format!("{} ", v) } }).collect::<Vec<String>>();
    let field_type = field_type.join("");

    field_type
}


pub fn generate_decode_return(fn_body: &mut StreamBuilder, fields: &Option<Fields>, variant: Option<&EnumVariant>) -> Result<()> {
    if variant.is_some() {
        fn_body.push_parsed("return")?;
    }

    fn_body.ident_str("Ok");
    fn_body.group(Delimiter::Parenthesis, |ok_group| {
        ok_group.group(Delimiter::Parenthesis, |ok_group| {
            let mut is_enum = false;

            if let Some(variant) = variant {
                ok_group.push_parsed(format!("input, Self::{}", variant.name.clone()))?;
                is_enum = true;
            }
            else {
                ok_group.push_parsed("input, Self")?;
            }

            if let Some(fields) = fields.as_ref() {
                match fields {
                    Fields::Struct(value) => {
                        let args = value
                                            .iter()
                                            .map(|(ident, _v)| ident.to_string())
                                            .collect::<Vec<String>>()
                                            .join(", ");

                        ok_group.push_parsed(format!("{{{args}}}"))?;
                    },
                    Fields::Tuple(value) => {
                        let args = value
                                            .iter()
                                            .enumerate()
                                            .map(|(index, _v)| format!("v{index}"))
                                            .collect::<Vec<String>>()
                                            .join(", ");

                        ok_group.push_parsed(format!("({args})"))?;
                    },
                }
            }
            else if !is_enum {
                // Ok((input, Self {}))
                ok_group.push_parsed("{{}}")?;
            }

            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}


pub fn generate_decode_struct_body(fn_body: &mut StreamBuilder, crate_name: &str, fields: &Option<Fields>, _is_enum: bool) -> Result<()> {
    if let Some(fields) = fields.as_ref() {
        match fields {
            Fields::Struct(value) => {
                for (ident, field) in value {
                    let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                    fn_body.push_parsed(attributes.to_code(false, false))?;
                    generate_decode_body(fn_body, crate_name, &attributes, ident.to_string(), &get_field_type(field), false)?;
                }
            },
            Fields::Tuple(value) => {
                for (index, field) in value.iter().enumerate() {
                    let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                    if attributes.is_use {
                        fn_body.push_parsed(attributes.to_code(false, false))?;
                    }
                    generate_decode_body(fn_body, crate_name, &attributes, index.to_string(), &get_field_type(field), true)?;
                }
            },
        }

        // for field in fields.names() {
        //     let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
    
        //     // TODO
        //     match field {
        //         IdentOrIndex::Ident { ident, attributes: _attributes } => {
        //             fn_body.push_parsed(attributes.to_code(false))?;

        //             generate_decode_body(fn_body, crate_name, &attributes, ident.to_string(), false)?;
        //         },
        //         IdentOrIndex::Index { index, span: _span, attributes: _attributes } => {
        //             fn_body.push_parsed(attributes.to_code(false))?;
        //             generate_decode_body(fn_body, crate_name, &attributes, index.to_string(), true)?;
        //         },
        //     }
        // }    
    }

    Ok(())
}


impl DeriveStruct {
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
        generate_decode_struct_body(fn_body, crate_name, &self.fields, false)?;
        generate_decode_return(fn_body, &self.fields, None)?;
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
                fn_body.push_parsed(self.attributes.to_code(true))?;

                if let Some(fields) = self.fields.as_ref() {
                    for field in fields.names() {
                        let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        fn_body.push_parsed(attributes.to_code(true, false))?;
                        // self.xxx.encode(input, Some(&cattr), Some(&fattr))
                        // jppe::ByteEncode::encode(&value, &mut buf, None, None);
                        // fn_body.push_parsed(format!("self.{field}.encode(input, Some(&cattr), Some(&fattr));"))?;
                        if !attributes.untake {
                            generate_encode_body(fn_body, &attributes, true)?;
                            fn_body.push_parsed(format!("{crate_name}::encode(&self.{field}, input, Some(&cattr), Some(&fattr));"))?;
                        }
                    }
                }
                Ok(())
            })?;
        Ok(())
    }
}
