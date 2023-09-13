// fn _table_name() -> &'static str;
pub(crate) fn impl_table_name(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();
    if let Some((table_name, _)) = input.get_args_path_value(vec![], "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn _table_name() -> &'static str {
                #table_name
            }
        ));
    }
    if ret_token_stream.is_empty() {
        Err(syn::Error::new_spanned(&input.input, r#"Please set arel(table_name = "xxx")"#))
    } else {
        Ok(ret_token_stream)
    }
}
// fn _primary_keys() -> Vec<&'static str>;
pub(crate) fn impl_primary_keys(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;
    let mut primary_keys: Vec<String> = vec![];
    for field in fields.iter() {
        if let Some(_) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "primary_key", None)? {
            if let Some(field_ident) = &field.ident {
                if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                    primary_keys.push(rename);
                } else {
                    primary_keys.push(field_ident.to_string().trim_start_matches("r#").to_string());
                }
            }
        }
    }

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    if primary_keys.len() > 0 {
        ret_token_stream.extend(quote::quote!(
            fn _primary_keys() -> Vec<&'static str> {
                vec![#(#primary_keys),*]
            }
        ))
    }

    Ok(ret_token_stream)
}
