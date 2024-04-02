#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::parse::Fields;
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::decode::generate_decode_body;
use super::encode::generate_encode_body;
use super::parse::AttrValue;


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
    pub lifetimes: Option<String>,
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


pub fn generate_decode_struct_body(fn_body: &mut StreamBuilder, crate_name: &str, fields: &Option<Fields>, _cattr: &ContainerAttributes, _is_enum: bool) -> Result<()> {
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
                // fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;

                Ok(())
            })?;

        Ok(())
    }

    pub fn generate_borrow_decode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = "jppe::BorrowByteDecode";
        let lifetimes = self.lifetimes.clone().unwrap_or("<'de>".to_string());

        let mut impl_for = if self.lifetimes.is_some() {
            generator
            .impl_for(format!("{crate_name}{lifetimes}"))    
        }
        else {
            generator
            .impl_for_with_lifetimes(crate_name, ["de"])
        };

        let lifetimes = if self.lifetimes.is_some() {
            lifetimes.trim_start_matches('<').trim_end_matches('>').split(',').map(|v|v.trim().replace('\'', "")).collect::<Vec<String>>()
        }
        else { vec!["de".to_string()] };

        // generator
        //     .impl_for(format!("{crate_name}<'a>"))
        //     // .impl_for_with_lifetimes(crate_name, ["de"])
        impl_for
            .generate_fn("decode")
            .with_lifetime_deps("da", lifetimes)
            .with_lifetime("db")
            .with_arg("input", "&'da [u8]")
            .with_arg("cattr", "Option<&'db jppe::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&'db jppe::FieldAttrModifiers>")
            .with_return_type("jppe::JResult<&'da [u8], Self>")
            .body(|fn_body| {
                // fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;

                Ok(())
            })?;

        Ok(())
    }

    fn generate_byte_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
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

            generate_decode_struct_body(fn_body, crate_name, &self.fields, &self.attributes, false)?;
            generate_decode_return(fn_body, &self.fields, None)?;
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
                if let Some(func) = &self.attributes.with_encode {
                    fn_body.push_parsed(format!("{func}(input, cattr, fattr, self)"))?;
                }
                else if let Some(func) = &self.attributes.with {
                    fn_body.push_parsed(format!("{func}::encode(input, cattr, fattr, self)"))?;
                }
                else {
                    fn_body.push_parsed(self.attributes.to_code(true))?;

                    if let Some(value) = &self.attributes.get_variable_name && let AttrValue::List(variable_names) = value {
                        for variable_name in variable_names {
                            let variable_name_str = variable_name.to_string();
        
                            fn_body.push_parsed(format!("let {variable_name_str} = if let Some(cr) = cattr && let Some(value) = cr.variable_name.borrow().get(&\"{variable_name_str}\".to_string()) {{*value}} else {{0}};"))?;
                        }
                    }

                    if let Some(fields) = self.fields.as_ref() {
                        for field in fields.names() {
                            let mut attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                            attributes.get_variable_name = self.attributes.get_variable_name.clone();
                            fn_body.push_parsed(attributes.to_code(true, false))?;
                            generate_encode_body(fn_body, &attributes, crate_name, &field.to_string(), true)?;
                        }
                    }
                }
        
                Ok(())
            })?;
        Ok(())
    }
}
