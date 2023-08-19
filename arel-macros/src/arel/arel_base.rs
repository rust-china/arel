pub(crate) fn impl_table_name(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();

    if let Some(table_name) = super::get_path_value(input, None, "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn table_name() -> std::borrow::Cow<'static, str>
            where
                Self: Sized,
            {
                std::borrow::Cow::Borrowed(stringify!(#table_name))
            }
        ));
    }

    Ok(ret_token_stream)
}
