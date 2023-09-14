// mod derive_input;
mod item_input;

// pub use derive_input::DeriveInput;
pub use item_input::ItemInput;

use syn::{parse::Parser, spanned::Spanned};

fn get_path_value_from_meta(meta: &syn::Meta, root_attr_paths: Vec<&str>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
    match root_attr_paths.split_first() {
        Some((root_attr_path, sub_root_attr_paths)) => {
            let sub_root_attr_paths: Vec<&str> = sub_root_attr_paths.into_iter().map(|v| *v).collect();
            match meta {
                syn::Meta::List(list) => {
                    for segment in list.path.segments.iter() {
                        if segment.ident == root_attr_path {
                            let nested_metas = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse2(list.tokens.clone()).unwrap();
                            for meta in nested_metas.iter() {
                                match get_path_value_from_meta(meta, sub_root_attr_paths.clone(), attr_path, allowed_path_names.clone())? {
                                    Some(v) => return Ok(Some(v)),
                                    None => continue,
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        None => match meta {
            syn::Meta::NameValue(kv) => {
                if kv.path.is_ident(attr_path) {
                    match &kv.value {
                        syn::Expr::Lit(expr) => match &expr.lit {
                            syn::Lit::Str(ref str) => {
                                return Ok(Some((str.value().to_string(), Some(expr.lit.clone()))));
                            }
                            syn::Lit::Int(ref int) => {
                                return Ok(Some((int.base10_digits().to_string(), Some(expr.lit.clone()))));
                            }
                            syn::Lit::Char(char) => {
                                return Ok(Some((char.value().to_string(), Some(expr.lit.clone()))));
                            }
                            syn::Lit::Bool(bool) => {
                                return Ok(Some((bool.value().to_string(), Some(expr.lit.clone()))));
                            }
                            syn::Lit::Float(float) => {
                                return Ok(Some((float.base10_digits().to_string(), Some(expr.lit.clone()))));
                            }
                            _ => (),
                        },
                        _ => return Ok(Some(("".to_string(), None))),
                    }
                }
                if let Some(ref allowed_path_names) = allowed_path_names {
                    match kv.path.get_ident() {
                        Some(kv_path_ident) => {
                            let kv_path_name = kv_path_ident.to_string();
                            if allowed_path_names.iter().find(|allowed_name| *allowed_name == &kv_path_name).is_none() {
                                return Err(syn::Error::new_spanned(&meta, format!(r#"expected `[macro]({} = "...")`"#, allowed_path_names.join("|"))));
                            }
                        }
                        _ => (),
                    }
                }
            }
            syn::Meta::Path(path) => {
                for segment in path.segments.iter() {
                    if segment.ident == attr_path {
                        return Ok(Some(("true".to_string(), Some(syn::Lit::Bool(syn::LitBool { value: true, span: path.span() })))));
                    }
                }
            }
            _ => (),
        },
    }
    Ok(None)
}
