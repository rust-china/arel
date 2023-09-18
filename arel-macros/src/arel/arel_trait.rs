// fn _table_name() -> String;
pub(crate) fn impl_table_name(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();
    if let Some((table_name, _)) = input.get_args_path_value(vec![], "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn _table_name() -> String {
                #table_name.into()
            }
        ));
    }
    // if ret_token_stream.is_empty() {
    //     Err(syn::Error::new_spanned(&input.input, r#"Please set arel(table_name = "xxx")"#))
    // } else {
    //     Ok(ret_token_stream)
    // }
    Ok(ret_token_stream)
}
// fn primary_keys() -> Vec<&'static str>;
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
            fn primary_keys() -> Vec<&'static str> {
                vec![#(#primary_keys),*]
            }
        ))
    }

    Ok(ret_token_stream)
}

// fn primary_values(&self) -> Vec<arel::Value>;
pub(crate) fn impl_primary_values(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut sub_clauses = vec![];

    let fields = input.struct_fields()?;
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
        sub_clauses.push(quote::quote!(
            if Self::primary_keys().contains(&#field_name) {
                primary_values.push(self.#ident.clone().into());
            }
        ));
    }

    Ok(quote::quote!(
        fn primary_values(&self) -> Vec<arel::Value> {
            let mut primary_values = vec![];
            #(#sub_clauses)*
            primary_values
        }
    ))
}
