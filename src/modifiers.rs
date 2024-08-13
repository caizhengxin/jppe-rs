use crate::std::*;
use crate::ByteOrder;


#[derive(Debug, Clone)]
pub enum ModifierValue {
    Bool(bool),
    Usize(usize),
    String(String),
    Vecu8(Vec<u8>),
}


#[derive(Debug, Default, Clone)]
pub struct ContainerAttrModifiers {
    pub byteorder: Option<ByteOrder>,
    pub expr: Option<String>,

    // cache variable
    pub variable_name: RefCell<BTreeMap<String, usize>>,
}


#[derive(Debug, Default, Clone)]
pub struct FieldAttrModifiers<'a> {
    // byte stream offset
    pub offset: Option<usize>,
    // string/stream/.. length
    pub length: Option<usize>,
    // list/vec/.. count
    pub count: Option<usize>,
    pub try_count: Option<usize>,
    // byte stream byteorder
    pub byteorder: Option<ByteOrder>,
    // Unmoved byte stream
    pub untake: bool,
    pub linend: bool,
    pub linend_value: Option<&'a [u8]>,
    pub bits: Option<usize>,
    pub bits_start: bool,
    pub byte_count: Option<usize>,

    // key value
    pub key: Option<&'a [u8]>,
    pub split: Option<&'a [u8]>,

    // branch
    pub branch: Option<usize>,
    // pub branch_byte: Option<u8>,
    // pub branch_byteorder: Option<ByteOrder>,
    // pub branch_func: Option<String>,
    // pub branch_enum: Option<String>,

    pub expr: Option<String>,
}


#[inline]
pub fn get_byteorder(cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) -> crate::ByteOrder {
    if let Some(value) = fattr {
        if let Some(byteorder) = value.byteorder {
            return byteorder;
        }
    }

    if let Some(value) = cattr {
        if let Some(byteorder) = value.byteorder {
            return byteorder;
        }
    }

    crate::ByteOrder::Be
}