mod arel_base;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parser, spanned::Spanned};

pub(crate) struct Input {
    args: Option<syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>>,
    st: syn::DeriveInput,
}

pub fn create_arel(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = Input {
        args: if args.is_empty() {
            None
        } else {
            Some(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse(args).unwrap())
        },
        st: syn::parse_macro_input!(input as syn::DeriveInput),
    };

    match do_expand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => {
            let mut ret_token_stream = e.to_compile_error();
            ret_token_stream.extend(input.st.to_token_stream());
            ret_token_stream.into()
        }
    }
}

fn do_expand(input: &Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let model_name_ident = &input.st.ident;

    let arel_base_impl_table_name = arel_base::impl_table_name(input)?;

    let (impl_generics, type_generics, where_clause) = st.generics.split_for_impl();
    Ok(quote::quote!(

        #[derive(Clone, Debug, Default, PartialEq, sqlx::FromRow)]
        #st

        impl #impl_generics arel::ArelBase for #model_name_ident #type_generics #where_clause {
            #arel_base_impl_table_name
        }
        impl #impl_generics arel::ArelRecord for #model_name_ident #type_generics #where_clause {}
    ))
}

fn get_path_value(input: &Input, field: Option<&syn::Field>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
    let metas = match field {
        Some(field) => field.attrs.iter().map(|f| &f.meta).collect::<Vec<&syn::Meta>>(),
        None => match &input.args {
            Some(metas) => metas.iter().map(|f| f).collect::<Vec<&syn::Meta>>(),
            None => return Ok(None),
        },
    };

    for meta in metas {
        match meta {
            syn::Meta::NameValue(kv) => {
                if kv.path.is_ident(attr_path) {
                    match &kv.value {
                        syn::Expr::Lit(expr) => {
                            if let syn::Lit::Str(ref ident_str) = expr.lit {
                                return Ok(Some(syn::Ident::new(ident_str.value().as_str(), field.span())));
                            }
                        }
                        _ => (),
                    }
                }
                if let Some(ref allowed_path_names) = allowed_path_names {
                    match kv.path.get_ident() {
                        Some(kv_path_ident) => {
                            let kv_path_name = kv_path_ident.to_string();
                            if allowed_path_names.iter().find(|allowed_name| *allowed_name == &kv_path_name).is_none() {
                                return Err(syn::Error::new_spanned(&meta, format!(r#"expected `arel({} = "...")`"#, allowed_path_names.join("|"))));
                            }
                        }
                        _ => (),
                    }
                }
            }
            syn::Meta::List(list) => {
                if let Some(p) = list.path.segments.first() {
                    if p.ident == "arel" {
                        let nested_metas = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse2(list.tokens.clone()).unwrap();
                        for nested_meta in nested_metas.iter() {
                            if let syn::Meta::NameValue(kv) = nested_meta {
                                if kv.path.is_ident(attr_path) {
                                    match &kv.value {
                                        syn::Expr::Lit(expr) => {
                                            if let syn::Lit::Str(ref ident_str) = expr.lit {
                                                return Ok(Some(syn::Ident::new(ident_str.value().as_str(), field.span())));
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                if let Some(ref allowed_path_names) = allowed_path_names {
                                    match kv.path.get_ident() {
                                        Some(kv_path_ident) => {
                                            let kv_path_name = kv_path_ident.to_string();
                                            if allowed_path_names.iter().find(|allowed_name| *allowed_name == &kv_path_name).is_none() {
                                                return Err(syn::Error::new_spanned(&list, format!(r#"expected `arel({} = "...")`"#, allowed_path_names.join("|"))));
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
    Ok(None)
}
