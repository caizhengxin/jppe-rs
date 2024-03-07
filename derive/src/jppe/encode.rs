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


#[inline]
pub fn generate_encode_body(fn_body: &mut StreamBuilder, attributes: &FieldAttributes, crate_name: &str, field: &String, is_self: bool) -> Result<()> {
    let der_arg = if is_self {"&"} else {""};
    let self_arg = if is_self {"self."} else {""};

    generate_encode_body2(fn_body, attributes, is_self)?;
    fn_body.push_parsed(format!("{crate_name}::encode({der_arg}{self_arg}{field}, input, Some(&cattr), Some(&fattr));"))?;    

    if attributes.untake {
        // fn_body.push_parsed(format!("let {field} = {self_arg}{field};"))?;

        // if let Some(value_expr) = &attributes.value {
        //     fn_body.push_parsed(format!("let {field} = {value_expr};"))?;
        // }
        // else if let Some(value_expr) = &attributes.value_encode {
        //     fn_body.push_parsed(format!("let {field} = {value_expr};"))?;
        // }
    }
    else {
        // let mut status = false;

        // fn_body.push_parsed(format!("let {field} = {self_arg}{field};"))?;

        // if let Some(value_expr) = &attributes.value {
        //     fn_body.push_parsed(format!("let {field} = {value_expr};"))?;
        //     status = true;
        // }
        // else if let Some(value_expr) = &attributes.value_encode {
        //     fn_body.push_parsed(format!("let {field} = {value_expr};"))?;
        //     status = true;
        // }

        // generate_encode_body2(fn_body, attributes, is_self)?;

        // if status {
        //     fn_body.push_parsed(format!("{crate_name}::encode({der_arg}{field}, input, Some(&cattr), Some(&fattr));"))?;
        // }
        // else {
        //     fn_body.push_parsed(format!("{crate_name}::encode({der_arg}{self_arg}{field}, input, Some(&cattr), Some(&fattr));"))?;    
        // }
    }

    // if !attributes.untake {
    //     // generate_encode_body(fn_body, &attributes, true)?;
    //     generate_encode_body2(fn_body, attributes, is_self)?;
    //     fn_body.push_parsed(format!("{crate_name}::encode({self_arg}{field}, input, Some(&cattr), Some(&fattr));"))?;

    //     // variant_body.push_parsed(format!("{crate_name}::encode({ident}, input, Some(&cattr), Some(&fattr));"))?;
    // }

    Ok(())
}
