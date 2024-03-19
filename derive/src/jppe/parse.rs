use virtue::prelude::*;


#[derive(Debug)]
pub enum AttrValue {
    String(String),
    Var(String),
    Usize(usize),
    Option(String),
    List(Vec<AttrValue>),
}


impl AttrValue {
    #[inline]
    pub fn parse_string(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();

        Ok(Self::String(value))
    }

    #[inline]
    pub fn parse_option_string(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();

        Ok(Self::Option(value))
    }

    #[inline]
    pub fn parse_usize(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();
        let value_type = if value.starts_with("0x") {16} else {10};

        if let Ok(v) = usize::from_str_radix(value.trim_start_matches("0x"), value_type) {
            return Ok(Self::Usize(v));
        }

        Ok(Self::Var(value))
    }

    #[inline]
    pub fn parse_list(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();
        let mut vlist = vec![];

        for v in value.split(',') {
            let value_type = if v.starts_with("0x") {16} else {10};

            if let Ok(v) = usize::from_str_radix(v.trim_start_matches("0x"), value_type) {
                vlist.push(Self::Usize(v));
            }

            vlist.push(Self::String(v.to_string()))
        }

        Ok(Self::List(vlist))
    }

    #[inline]
    pub fn parse_byteorder(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();

        match value.as_str() {
            "BE" | "LE" | "0" | "1" | ">" | "<" => Ok(Self::String(value)),
            _ => Ok(Self::Var(value)),
        }
    }

    pub fn to_code(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };
        let deref_arg = if is_deref { "*" } else { "" };
        let is_string = if is_string { "\"" } else { "" };

        let code = match self {
            Self::String(v) => format!("{deref_arg}{self_arg}{is_string}{v}{is_string}.into()"),
            Self::Var(v) => format!("({deref_arg}{self_arg}{is_string}{v}{is_string}) as usize"),
            Self::Usize(v) => format!("({deref_arg}{v}) as usize"),
            Self::Option(v) => format!("if let Some(v) = {deref_arg}{self_arg}{is_string}{v} {{Some(v as usize)}} else {{None}}"),
            Self::List(v) =>  {
                let value = v.iter().map(|v| format!("{}", v.to_code(is_self, is_deref, true))).collect::<Vec<String>>().join(", ");

                format!("vec![{value}]")
            },
        };

        code
    }

    pub fn to_code2(&self, is_self: bool, is_string: bool) -> String {
        let self_arg = if is_self { "self." } else { "*" };
        let is_string = if is_string { "\"" } else { "" };

        let code = match self {
            Self::String(v) => format!("{self_arg}{is_string}{v}{is_string}.into()"),
            Self::Var(v) => format!("({self_arg}{is_string}{v}{is_string}) as usize"),
            Self::Usize(v) => format!("({v}) as usize"),
            Self::Option(v) => format!("if let Some(v) = {self_arg}{is_string}{v} {{Some(v as usize)}} else {{None}}"),
            Self::List(v) =>  {
                let value = v.iter().map(|v| format!("{}", v.to_code2(is_self, true))).collect::<Vec<String>>().join(", ");

                format!("vec![{value}]")
            },
        };

        code
    }

    pub fn to_byteorder(&self, is_self: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };

        let code = match self {
            Self::String(v) => format!("jppe::ByteOrder::parse({v:?}).unwrap()"),
            Self::Var(v) => format!("jppe::ByteOrder::from_int({self_arg}{v} as isize).unwrap()"),
            _ => "".to_string(),
        };

        code
    }
}


impl ToString for AttrValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Var(v) => v.to_string(),
            Self::Usize(v) => v.to_string(),
            Self::Option(v) => v.to_string(),
            Self::List(v) => v.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", "),
        }
    }
}


pub trait AttrValueTrait {
    type Value;

    fn to_code(&self, is_self: bool, is_deref: bool) -> String;

    fn to_code_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String;

    fn to_code_option_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String;

    fn to_byteorder(&self, is_self: bool) -> String;
}


impl AttrValueTrait for Option<AttrValue> {
    type Value = AttrValue;

    #[inline]
    fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_code(is_self, is_deref, false));
        }

        "None".to_string()
    }

    #[inline]
    fn to_code_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_code(is_self, is_deref, is_string));
        }

        "None".to_string()
    }

    #[inline]
    fn to_code_option_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        if let Some(value) = self {
            return value.to_code(is_self, is_deref, is_string);
        }

        "None".to_string()
    }

    #[inline]
    fn to_byteorder(&self, is_self: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_byteorder(is_self));
        }

        "None".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attrvalue_parse() {
        let value = Some(AttrValue::List(vec![
            AttrValue::String("jkc".to_string()),
            AttrValue::String("jkc".to_string()),
        ]));

        println!("{:?}", value.to_code(false, false));
        assert_eq!(value.to_code(false, false), r#"Some(vec!["jkc".into(), "jkc".into()])"#);

        let value = Some(AttrValue::String("jkc".to_string()));
        assert_eq!(value.to_code(false, false), r#"Some(jkc.into())"#);
    }
}