use proc_macro::TokenStream;
use quote::ToTokens;

pub fn create_arel(args: TokenStream, input: TokenStream) -> TokenStream {
    // AttributeArgs 及为 Vec<NestedMeta>类型的语法树节点
    // eprintln!("1-----");
    let args = if args.is_empty() { None } else { Some(syn::parse_macro_input!(args as syn::MetaList)) };

    // eprintln!("2-----");
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    // eprintln!("3-----");

    match do_expand(args.as_ref(), &derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => {
            let mut ret_token_stream = e.to_compile_error();
            ret_token_stream.extend(derive_input.to_token_stream());
            ret_token_stream.into()
        }
    }
}

fn do_expand(_args: Option<&syn::MetaList>, input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let model_name_ident = &input.ident;
    // let model_name = model_name_ident.to_string();
    // let arel_model_name = format!("Arel{}", model_name);

    Ok(quote::quote!(

        #[derive(Clone, Debug, Default, sqlx::FromRow)]
        #input

        impl arel::ArelBase for #model_name_ident {}
        impl arel::ArelRecord for #model_name_ident {}
    ))
}
