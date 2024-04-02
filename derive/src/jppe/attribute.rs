use virtue::prelude::*;
use virtue::utils::*;
use super::parse::{AttrValue, AttrValueTrait, parse_value_string};


#[derive(Debug, Default)]
pub struct ContainerAttributes {
    pub is_use: bool,
    pub byteorder: Option<AttrValue>,
    pub get_variable_name: Option<AttrValue>,
    pub default_value: Option<String>,
    pub default_bool: bool,
    pub byte_count_disable: bool,

    // branch
    pub branch_byte: Option<u8>,
    pub branch_byteorder: Option<String>,
    pub branch_func: Option<String>,
    pub branch_enum: Option<String>,

    // custom encode/decode function.
    pub with_encode: Option<String>,
    pub with_decode: Option<String>,
    pub with: Option<String>,
}


impl ContainerAttributes {
    pub fn to_code(&self, is_self: bool) -> String {
        let byteorder = self.byteorder.to_byteorder(is_self);

        if self.is_use {
            format!("let mut cattr_new = jppe::ContainerAttrModifiers {{
                byteorder: {byteorder},
                ..Default::default()}}; let cattr_new = Some(&cattr_new);")
        }
        else {
            "let mut cattr_new: Option<&jppe::ContainerAttrModifiers> = None;".to_string()
        }
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jppe")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        if !attributes.is_empty() {
            result.is_use = true;
        }

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "default" | "default_value" => result.default_bool = true,
                        "byte_count_disable" => result.byte_count_disable = true,
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "byteorder" => result.byteorder = Some(AttrValue::parse_byteorder(&val)?),
                        "get_variable_name" => result.get_variable_name = Some(AttrValue::parse_list(&val)?),
                        "default_value" | "default" => result.default_value = Some(parse_value_string(&val)?),
                        "branch_byte" => result.branch_byte = Some(parse_value_string(&val)?.parse().unwrap()),
                        "branch_byteorder" => result.branch_byteorder = Some(parse_value_string(&val)?),
                        "branch_func" => result.branch_func = Some(parse_value_string(&val)?),
                        "branch_enum" => result.branch_enum = Some(parse_value_string(&val)?),
                        // custom encode/decode
                        "with_encode" | "encode_with" => result.with_encode = Some(parse_value_string(&val)?),
                        "with_decode" | "decode_with" => result.with_decode = Some(parse_value_string(&val)?),
                        "with" => result.with = Some(parse_value_string(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}


#[derive(Debug, Default)]
pub struct FieldAttributes {
    pub is_use: bool,
    pub byteorder: Option<AttrValue>,
    pub length: Option<AttrValue>,
    pub offset: Option<AttrValue>,
    pub untake: bool,
    pub full: Option<AttrValue>,
    pub count: Option<AttrValue>,
    pub try_count: Option<AttrValue>,
    pub bits: Option<AttrValue>,
    pub bits_start: bool,
    pub byte_count: Option<AttrValue>,
    pub default_value: Option<String>,
    pub default_bool: bool,

    pub value_encode: Option<String>,
    pub value_decode: Option<String>,
    pub from_str_bool: bool,
    pub from_str: Option<String>,

    pub get_variable_name: Option<AttrValue>,
    pub variable_name: Option<AttrValue>,

    pub key: Option<AttrValue>,
    pub split: Option<AttrValue>,
    pub linend: Option<AttrValue>,

    // branch
    pub branch: Option<AttrValue>,
    pub branch_option: Option<AttrValue>,
    pub branch_bits: Option<String>,
    pub branch_bits_value: Option<String>,
    pub branch_range: Option<String>,
    pub branch_value: Option<String>,
    pub branch_default: bool,

    // custom encode/decode function.
    pub with_encode: Option<String>,
    pub with_decode: Option<String>,
    pub with: Option<String>,
    pub with_args: Option<String>,

    pub if_expr: Option<String>,

    // skip
    pub skip: bool,
    pub skip_encode: bool,
    pub skip_decode: bool,
}


impl FieldAttributes {
    pub fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        let byteorder = self.byteorder.to_byteorder(is_self);
        let length = if let Some(length) = &self.length
            && let Some(viariable_name) = &self.get_variable_name
            && viariable_name.to_string().contains(&length.to_string())
        {
            self.length.to_code(false, false)            
        }
        else { self.length.to_code(is_self, is_deref) };
        // let length = self.length.to_code(is_self, is_deref);
        let count = self.count.to_code(is_self, is_deref);
        let try_count = self.try_count.to_code(is_self, is_deref);
        let mut branch = self.branch.to_code(is_self, is_deref);
        if let Some(branch_option) = &self.branch_option {
            branch = branch_option.to_code(is_self, is_deref, false);
        }
        let key = self.key.to_code_string(false, false, true);
        let split = self.split.to_code(false, false);
        let linend = self.linend.to_code(false, false);
        let bits = self.bits.to_code(is_self, is_deref);
        let bits_start = self.bits_start;
        let byte_count = self.byte_count.to_code(is_self, is_deref);

        if self.is_use {
            let value = format!("let fattr_new = jppe::FieldAttrModifiers {{
                byteorder: {byteorder}, branch: {branch}, length: {length}, count: {count}, try_count: {try_count},
                split: {split}, linend_value: {linend}, bits: {bits}, bits_start: {bits_start},
                key: {key}, byte_count: {byte_count},
                ..Default::default()}}; let fattr_new = Some(&fattr_new);");

            if value.contains(": Some(") || value.contains(": true") || value.contains("if let Some(") {
                return value;
            }

        }

        "let fattr_new: Option<&jppe::FieldAttrModifiers> = None;".to_string()
    }
}


impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jppe")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        if !attributes.is_empty() {
            result.is_use = true;
        }

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "enum_default" | "branch_default" => result.branch_default = true,
                        "untake" => result.untake = true,
                        "bits_start" => result.bits_start = true,
                        "skip" => result.skip = true,
                        "skip_encode" => result.skip_encode = true,
                        "skip_decode" => result.skip_decode = true,
                        "default" | "default_value" => result.default_bool = true,
                        "from_str" => result.from_str_bool = true,
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "byteorder" => result.byteorder = Some(AttrValue::parse_byteorder(&val)?),
                        "length" => result.length = Some(AttrValue::parse_usize(&val)?),
                        "offset" => result.offset = Some(AttrValue::parse_usize(&val)?),
                        "count" => result.count = Some(AttrValue::parse_usize(&val)?),
                        "try_count" => result.try_count = Some(AttrValue::parse_usize(&val)?),
                        "full" => result.full = Some(AttrValue::parse_usize(&val)?),
                        "key" | "starts_with" => result.key = Some(AttrValue::parse_string(&val)?),
                        "split" => result.split = Some(AttrValue::parse_list(&val)?),
                        "linend" | "end_with" => result.linend = Some(AttrValue::parse_bytes(&val)?),
                        "branch" => result.branch = Some(AttrValue::parse_usize(&val)?),
                        "branch_option" => result.branch_option = Some(AttrValue::parse_option_string(&val)?),
                        "branch_range" => result.branch_range = Some(parse_value_string(&val)?),
                        "branch_value" => result.branch_value = Some(parse_value_string(&val)?),
                        "branch_bits" => result.branch_bits = Some(parse_value_string(&val)?),
                        "branch_bits_value" => result.branch_bits_value = Some(parse_value_string(&val)?),
                        "byte_count" | "byte_size" => result.byte_count = Some(AttrValue::parse_usize(&val)?),
                        "default_value" | "default" => result.default_value = Some(parse_value_string(&val)?),

                        "bits" => result.bits = Some(AttrValue::parse_usize(&val)?),
                        "bits_start" => {
                            result.bits = Some(AttrValue::parse_usize(&val)?);
                            result.bits_start = true;
                        },
                        "value_encode" | "encode_value" => result.value_encode = Some(parse_value_string(&val)?),
                        "value_decode" | "decode_value" => result.value_decode = Some(parse_value_string(&val)?),
                        "from_str" => result.from_str = Some(parse_value_string(&val)?),
                        // custom encode/decode
                        "with_encode" | "encode_with" => result.with_encode = Some(parse_value_string(&val)?),
                        "with_decode" | "decode_with" => result.with_decode = Some(parse_value_string(&val)?),
                        "with_args" => result.with_args = Some(parse_value_string(&val)?),
                        "with" => result.with = Some(parse_value_string(&val)?),
                        "variable_name" => result.variable_name = Some(AttrValue::parse_list(&val)?),
                        "if_expr" => result.if_expr = Some(parse_value_string(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
