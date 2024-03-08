use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{FieldAttributes, ContainerAttributes};


#[inline]
pub fn generate_decode_body2(fn_body: &mut StreamBuilder, attributes: &FieldAttributes) -> Result<()> {
    // offset
    if let Some(offset) = &attributes.offset {
        fn_body.push_parsed(format!("let (input, _) = jppe::input_take(input, ({}) as usize)?;", offset.to_string()))?;
    }

    Ok(())
}


pub fn generate_decode_body(fn_body: &mut StreamBuilder, crate_name: &str, attributes: &FieldAttributes, name: String, rtype: &str, is_enum: bool) -> Result<()> {
    let name = if is_enum { format!("v{name}") } else { name };
    let with_args_default = "".to_string();
    let with_args = attributes.with_args.as_ref().unwrap_or(&with_args_default);

    if let Some(func) = &attributes.with_decode {
        fn_body.push_parsed(format!("let (input, {name}): (&[u8], {rtype}) = {func}(input, Some(&cattr), Some(&fattr), {with_args})?;"))?;
        return Ok(());
    }
    else if let Some(func) = &attributes.with {
        fn_body.push_parsed(format!("let (input, {name}): (&[u8], {rtype}) = {func}::decode(input, Some(&cattr), Some(&fattr), {with_args})?;"))?;
        return Ok(());
    }

    generate_decode_body2(fn_body, attributes)?;

    // untake
    if attributes.untake {
        fn_body.push_parsed(format!("let (_, {name}): (&[u8], {rtype}) = {crate_name}::decode(input, Some(&cattr), Some(&fattr))?;"))?;
    }
    else {
        fn_body.push_parsed(format!("let (input, {name}): (&[u8], {rtype}) = {crate_name}::decode(input, Some(&cattr), Some(&fattr))?;"))?;
    }

    // value expr
    if let Some(value_expr) = &attributes.value_decode {
        fn_body.push_parsed(format!("let {name} = {value_expr};"))?;
    }

    Ok(())
}
