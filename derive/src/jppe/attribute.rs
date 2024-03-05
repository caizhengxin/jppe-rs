use virtue::prelude::*;
use virtue::utils::*;


macro_rules! to_code_int {
    ($branch:expr, $self_arg:expr) => {
        if let Some(v) = $branch.as_ref() { format!("Some(({}{}) as usize)", $self_arg, v.to_string()) } else { "None".to_string() }
    };
}


macro_rules! to_code_int2 {
    ($branch:expr, $self_arg:expr, $deref_arg:expr) => {
        if let Some(v) = $branch.as_ref() { format!("Some(({}) as usize)", v.to_code2($self_arg, $deref_arg)) } else { "None".to_string() }
    };
}


fn parse_value_string(value: &Literal) -> Result<String> {
    let val_string = value.to_string();

    if val_string.starts_with("\"") && val_string.ends_with("\"") {
        return Ok(val_string[1..val_string.len() - 1].to_string());
    }

    Ok(val_string)
}


fn get_byteorder(byteorder: &Option<String>, self_arg: &str) -> String {
    if let Some(v) = byteorder {
        match v.as_str() {
            "BE" | "LE" | "0" | "1" | ">" | "<" => return format!("Some(jppe::ByteOrder::parse({v:?}).unwrap())"),
            _ => return format!("Some(jppe::ByteOrder::from_int({self_arg}{v} as isize).unwrap())"),
        }
    }

    "None".to_string()
}


#[derive(Debug)]
pub enum JAttrValue {
    String(String),
    Usize(usize),
    // Bytes(Vec<u8>),
}


impl JAttrValue {
    pub fn parse(s: &Literal) -> Result<Self> {
        let mut value = parse_value_string(s)?;
        let mut value_type = 10;

        if value.starts_with("0x") {
            value_type = 16;
            value = value[2..].to_string();
        }

        if let Ok(v) = usize::from_str_radix(&value, value_type) {
            return Ok(Self::Usize(v));
        }

        Ok(Self::String(value))

        // Err(Error::custom_at("Unknown field attribute", s.span()))
    }

    pub fn to_code2(&self, self_arg: &str, deref_arg: &str) -> String {
        match self {
            Self::String(v) => format!("{deref_arg}{self_arg}{v}"),
            Self::Usize(v) => format!("{deref_arg}{v}"),
        }
    }

    pub fn to_code(&self, is_self: bool) -> String {
        let self_arg = if is_self { "self." } else { "*" };

        match self {
            Self::String(v) => format!("{self_arg}{v}"),
            Self::Usize(v) => v.to_string(),
        }
    }
}


impl ToString for JAttrValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Usize(v) => v.to_string(),
        }
    }
}


#[derive(Debug, Default)]
pub struct ContainerAttributes {
    pub byteorder: Option<String>,

    // branch
    pub branch_byte: Option<u8>,
    pub branch_byteorder: Option<String>,
    pub branch_func: Option<String>,
    pub branch_enum: Option<String>,
}


impl ContainerAttributes {
    pub fn to_code(&self, is_self: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };

        format!("let mut cattr = jppe::ContainerAttrModifiers {{
            byteorder: {},
            ..Default::default()
        }};",
            get_byteorder(&self.byteorder, self_arg),
        )
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
                        "byteorder" => result.byteorder = Some(parse_value_string(&val)?),
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
    pub byteorder: Option<String>,
    pub length: Option<JAttrValue>,
    pub offset: Option<JAttrValue>,
    pub untake: bool,
    pub full: Option<JAttrValue>,

    // branch
    pub branch: Option<String>,
    pub branch_bits: Option<JAttrValue>,
    pub branch_expr: Option<String>,
    pub branch_range: Option<String>,
    pub branch_value: Option<String>,
    pub branch_default: bool,
}


impl FieldAttributes {
    pub fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };
        let deref_arg = if is_deref { "*" } else { "" };

        format!("let mut fattr = jppe::FieldAttrModifiers {{
            byteorder: {},
            branch: {},
            length: {},
            ..Default::default()
        }};",
            get_byteorder(&self.byteorder, self_arg),
            to_code_int!(self.branch, self_arg),
            to_code_int2!(self.length, self_arg, deref_arg),
        )
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
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "byteorder" => result.byteorder = Some(parse_value_string(&val)?),
                        "length" => result.length = Some(JAttrValue::parse(&val)?),
                        "offset" => result.offset = Some(JAttrValue::parse(&val)?),
                        "full" => result.full = Some(JAttrValue::parse(&val)?),
                        "branch" => result.branch = Some(parse_value_string(&val)?),
                        "branch_expr" => result.branch_expr = Some(parse_value_string(&val)?),
                        "branch_range" => result.branch_range = Some(parse_value_string(&val)?),
                        "branch_value" => result.branch_value = Some(parse_value_string(&val)?),
                        "branch_bits" => result.branch_bits = Some(JAttrValue::parse(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
