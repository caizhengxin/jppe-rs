use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{FieldAttributes, ContainerAttributes};


#[inline]
pub fn generate_encode_body2(fn_body: &mut StreamBuilder, attributes: &FieldAttributes, is_self: bool) -> Result<()> {
    // offset and full
    if let Some(offset) = &attributes.offset {
        let mut full = "0x00".to_string();

        if let Some(full_tmp) = &attributes.full {
            full = full_tmp.to_string();
        }

        fn_body.push_parsed(format!("
            for i in 0..{} {{ input.push({full}); }}
        ", offset.to_code2(is_self, false)))?;
    }

    Ok(())
}


pub fn generate_encode_body(fn_body: &mut StreamBuilder, attributes: &FieldAttributes, is_self: bool) -> Result<()> {
    generate_encode_body2(fn_body, attributes, is_self)?;

    Ok(())
}
