use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{FieldAttributes, ContainerAttributes};
use super::parse::AttrValue;


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
        fn_body.push_parsed(format!("let (input, {name}): (&[u8], {rtype}) = {func}(input, cattr_new, fattr_new, {with_args})?;"))?;
        return Ok(());
    }
    else if let Some(func) = &attributes.with {
        fn_body.push_parsed(format!("let (input, {name}): (&[u8], {rtype}) = {func}::decode(input, cattr_new, fattr_new, {with_args})?;"))?;
        return Ok(());
    }

    generate_decode_body2(fn_body, attributes)?;

    // untake
    let untake = if attributes.untake { "_" } else { "input" };

    if let Some(if_expr) = &attributes.if_expr {
        fn_body.push_parsed(format!("let ({untake}, {name}): (&[u8], {rtype}) = if {if_expr} {{ 
            let (input, value) = {crate_name}::decode(input, cattr_new, fattr_new)?;
            (input, Some(value))
        }} else {{ (input, None) }};"))?;
    }
    else {
        fn_body.push_parsed(format!("let ({untake}, {name}): (&[u8], {rtype}) = {crate_name}::decode(input, cattr_new, fattr_new)?;"))?;
    }

    // value expr
    if let Some(value_expr) = &attributes.value_decode {
        fn_body.push_parsed(format!("let {name} = {value_expr};"))?;
    }

    // variable_name
    if let Some(value) = &attributes.variable_name && let AttrValue::List(variable_names) = value {
        fn_body.push_parsed("
            let cattr_new2 = jppe::ContainerAttrModifiers::default();

            if cattr_new.is_none() {{
                cattr_new = Some(&cattr_new2);
            }}
        ")?;

        for variable_name in variable_names {
            let variable_name_str = variable_name.to_string();
            fn_body.push_parsed(format!("
                if let Some(cattr_new) = cattr_new {{
                    cattr_new.variable_name.borrow_mut().insert(\"{variable_name_str}\".to_string(), {variable_name_str}.into());
                }}
            "))?;
        }
    }

    Ok(())
}
