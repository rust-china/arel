pub(crate) fn impl_table_name(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();

    if let Some((table_name, _)) = input.get_args_path_value(vec!["arel"], "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn _table_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(#table_name)
            }
        ));
    }

    Ok(ret_token_stream)
}

pub(crate) fn impl_primary_key_or_primary_keys(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
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

    if primary_keys.len() == 1 {
        let primary_key = &primary_keys[0];
        ret_token_stream.extend(quote::quote!(
            fn _primary_key() -> std::option::Option<std::borrow::Cow<'static, str>> {
                std::option::Option::Some(std::borrow::Cow::Borrowed(#primary_key))
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

pub(crate) fn impl_table_column_name(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut get_column_name_clauses = vec![];
    for field in fields.iter() {
        let ident = &field.ident;

        let field_name = {
            if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                rename
            } else {
                match ident {
                    Some(ident) => ident.to_string().trim_start_matches("r#").to_string(),
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")),
                }
            }
        };

        get_column_name_clauses.push(quote::quote!(if struct_key.as_ref() == stringify!(#ident) {
            return std::option::Option::Some(std::borrow::Cow::Borrowed(#field_name))
        }));
    }

    Ok(quote::quote!(
        fn _table_column_name<K: AsRef<str>>(struct_key: K) -> Option<std::borrow::Cow<'static, str>> {
            #(#get_column_name_clauses)*
            None
        }
    ))
}
