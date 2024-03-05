use virtue::prelude::*;


#[derive(Debug)]
pub enum AttrValue {
    String(String),
    Usize(usize),
    List(Vec<AttrValue>),
}


impl AttrValue {
    #[inline]
    pub fn parse(s: &Literal) -> Result<Self> {
        let value = s.to_string().trim_end_matches('"').trim_start_matches('"').to_string();
        let value_type = if value.starts_with("0x") {16} else {10};

        if let Ok(v) = usize::from_str_radix(value.trim_start_matches("0x"), value_type) {
            return Ok(Self::Usize(v));
        }

        Ok(Self::String(value))
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

    // pub fn to_code2(&self, self_arg: &str, deref_arg: &str) -> String {
    //     match self {
    //         Self::String(v) => format!("{deref_arg}{self_arg}{v}"),
    //         Self::Usize(v) => format!("{deref_arg}{v}"),
    //         Self::List(v) =>  {
    //             let value = v.iter().map(|v| format!("\"{}\".into()", v.to_string())).collect::<Vec<String>>().join(", ");

    //             format!("vec![{value}]")
    //         },
    //     }
    // }

    pub fn to_code(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };
        let deref_arg = if is_deref { "*" } else { "" };
        let is_string = if is_string { "\"" } else { "" };

        match self {
            Self::String(v) => format!("{deref_arg}{self_arg}{is_string}{v}{is_string}.into()"),
            Self::Usize(v) => format!("{deref_arg}{v}.into()"),
            Self::List(v) =>  {
                let value = v.iter().map(|v| format!("{}", v.to_code(is_self, is_deref, true))).collect::<Vec<String>>().join(", ");

                format!("vec![{value}]")
            },
        }
    }
}


impl ToString for AttrValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Usize(v) => v.to_string(),
            Self::List(v) => v.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", "),
        }
    }
}


pub trait AttrValueTrait {
    type Value;

    fn to_code(&self, is_self: bool, is_deref: bool) -> String;
}


impl AttrValueTrait for Option<AttrValue> {
    type Value = AttrValue;

    fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        // macro_rules! to_code_int {
        //     ($branch:expr, $self_arg:expr) => {
        //         if let Some(v) = $branch.as_ref() { format!("Some(({}{}) as usize)", $self_arg, v.to_string()) } else { "None".to_string() }
        //     };
        // }
        
        
        // macro_rules! to_code_int2 {
        //     ($branch:expr, $self_arg:expr, $deref_arg:expr) => {
        //         if let Some(v) = $branch.as_ref() { format!("Some(({}) as usize)", v.to_code2($self_arg, $deref_arg)) } else { "None".to_string() }
        //     };
        // }

        if let Some(value) = self {
            match value {
                Self::Value::String(_v) => return format!("Some({})", value.to_code(is_self, is_deref, false)),
                Self::Value::Usize(_v) => return format!("Some({})", value.to_code(is_self, is_deref, false)),
                Self::Value::List(_) => return format!("Some({})", value.to_code(is_self, is_deref, false)),
            }    
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
    }
}