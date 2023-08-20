pub(crate) fn impl_table_name(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();

    if let Some(table_name) = super::get_path_value(input, None, "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn _table_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!(#table_name))
            }
        ));
    }

    Ok(ret_token_stream)
}

pub(crate) fn impl_primary_key_or_primary_keys(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let fields = super::get_fields(input)?;

    let mut primary_keys: Vec<String> = vec![];
    for field in fields.iter() {
        if let Some(_) = super::get_path_value(input, Some(field), "primary_key", None)? {
            if let Some(field_ident) = &field.ident {
                primary_keys.push(field_ident.to_string());
            }
        }
    }
    let mut ret_token_stream = proc_macro2::TokenStream::new();

    if primary_keys.len() == 1 {
        let primary_key = &primary_keys[0];
        ret_token_stream.extend(quote::quote!(
            fn _primary_key() -> std::option::Option<std::borrow::Cow<'static, str>> {
                std::option::Option::Some(#primary_key.into())
            }
        ));
    } else if primary_keys.len() > 1 {
        ret_token_stream.extend(quote::quote!(
            fn _primary_keys() -> Option<Vec<std::borrow::Cow<'static, str>>> {
                std::option::Option::Some(vec![
                    #(#primary_keys.into()),*
                ])
            }
        ));
    }

    Ok(ret_token_stream)
}
