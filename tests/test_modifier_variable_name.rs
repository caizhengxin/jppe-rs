
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct VariableExample {
    pub cmd: u8,
    #[jppe(variable_name="length")]
    pub length: u8,
    pub body: VariableExampleBody,
    #[jppe(branch="cmd")]
    pub enum_body: VariableExampleEnumBody,
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jppe(get_variable_name="length")]
pub struct VariableExampleBody {
    #[jppe(length="length")]
    value: String,
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jppe(get_variable_name="length")]
#[repr(u8)]
pub enum VariableExampleEnumBody {
    Read {
        #[jppe(length="length")]
        value: String,
    } = 1,
    Write {
        #[jppe(length="length")]
        value: String,
    } = 2,
    #[jppe(branch_default)]
    Unknown,
}


#[cfg(test)]
mod tests {
    use jppe::ByteDecode;

    use super::*;

    #[test]
    fn test_modifier_variable_name() {
        let input = b"\x01\x02abcd";

        let (input, value) = VariableExample::decode(input, None, None).unwrap();
        assert_eq!(value, VariableExample {
            cmd: 1,
            length: 2,
            body: VariableExampleBody { value: "ab".to_string() },
            enum_body: VariableExampleEnumBody::Read { value: "cd".to_string() },
        });
        assert_eq!(input.is_empty(), true);

        assert_eq!(jppe::encode(value), b"\x01\x02abcd");
    }
}