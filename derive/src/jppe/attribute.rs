use virtue::prelude::*;
use virtue::utils::*;
use super::parse::{AttrValue, AttrValueTrait};


#[inline]
fn parse_value_string(value: &Literal) -> Result<String> {
    Ok(value.to_string().trim_end_matches('"').trim_start_matches('"').to_string())
}


#[derive(Debug, Default)]
pub struct ContainerAttributes {
    pub byteorder: Option<AttrValue>,

    // branch
    pub branch_byte: Option<u8>,
    pub branch_byteorder: Option<String>,
    pub branch_func: Option<String>,
    pub branch_enum: Option<String>,
}


impl ContainerAttributes {
    pub fn to_code(&self, is_self: bool) -> String {
        let byteorder = self.byteorder.to_byteorder(is_self);

        format!("let mut cattr = jppe::ContainerAttrModifiers {{
            byteorder: {byteorder},
            ..Default::default()
        }};")
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jppe")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        // "name" => {},
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        // "alias" => {},
                        "byteorder" => result.byteorder = Some(AttrValue::parse_byteorder(&val)?),
                        "branch_byte" => result.branch_byte = Some(parse_value_string(&val)?.parse().unwrap()),
                        "branch_byteorder" => result.branch_byteorder = Some(parse_value_string(&val)?),
                        "branch_func" => result.branch_func = Some(parse_value_string(&val)?),
                        "branch_enum" => result.branch_enum = Some(parse_value_string(&val)?),
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
    pub bits: Option<AttrValue>,
    pub bits_start: bool,

    pub value_encode: Option<String>,
    pub value_decode: Option<String>,
    pub value: Option<String>,

    pub key: Option<AttrValue>,
    pub split: Option<AttrValue>,
    pub linend: Option<AttrValue>,

    // branch
    pub branch: Option<AttrValue>,
    pub branch_bits: Option<AttrValue>,
    pub branch_expr: Option<String>,
    pub branch_range: Option<String>,
    pub branch_value: Option<String>,
    pub branch_default: bool,
}


impl FieldAttributes {
    pub fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        let byteorder = self.byteorder.to_byteorder(is_self);
        let length = self.length.to_code(is_self, is_deref);
        let count = self.count.to_code(is_self, is_deref);
        let branch = self.branch.to_code(is_self, is_deref);
        let split = self.split.to_code(false, false);
        let linend = self.linend.to_code(false, false);
        let bits = self.bits.to_code(is_self, is_deref);
        let bits_start = self.bits_start;

        format!("let mut fattr = jppe::FieldAttrModifiers {{
            byteorder: {byteorder}, branch: {branch}, length: {length}, count: {count},
            split: {split}, linend_value: {linend}, bits: {bits}, bits_start: {bits_start},
            ..Default::default()
        }};")
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
                        "full" => result.full = Some(AttrValue::parse_usize(&val)?),
                        "split" => result.split = Some(AttrValue::parse_list(&val)?),
                        "linend" => result.linend = Some(AttrValue::parse_list(&val)?),
                        "branch" => result.branch = Some(AttrValue::parse_usize(&val)?),
                        "branch_expr" => result.branch_expr = Some(parse_value_string(&val)?),
                        "branch_range" => result.branch_range = Some(parse_value_string(&val)?),
                        "branch_value" => result.branch_value = Some(parse_value_string(&val)?),
                        "branch_bits" => result.branch_bits = Some(AttrValue::parse_usize(&val)?),
                        "bits" => result.bits = Some(AttrValue::parse_usize(&val)?),
                        "bits_start" => {
                            result.bits = Some(AttrValue::parse_usize(&val)?);
                            result.bits_start = true;
                        },
                        "value_encode" => result.value_encode = Some(parse_value_string(&val)?),
                        "value_decode" => result.value_decode = Some(parse_value_string(&val)?),
                        "value" => result.value = Some(parse_value_string(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
