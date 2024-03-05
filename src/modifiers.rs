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
}


#[derive(Debug, Default, Clone)]
pub struct FieldAttrModifiers {
    // byte stream offset
    pub offset: Option<usize>,
    // string/stream/.. length
    pub length: Option<usize>,
    // list/vec/.. count
    pub count: Option<usize>,
    // byte stream byteorder
    pub byteorder: Option<ByteOrder>,
    // Unmoved byte stream
    pub untake: bool,
    pub linend: bool,
    pub linend_value: Option<Vec<Vec<u8>>>,

    // key value
    pub key: Option<Vec<u8>>,
    pub split: Option<Vec<Vec<u8>>>,

    // branch
    pub branch: Option<usize>,
    // pub branch_byte: Option<u8>,
    // pub branch_byteorder: Option<ByteOrder>,
    // pub branch_func: Option<String>,
    // pub branch_enum: Option<String>,

    pub expr: Option<String>,
}
