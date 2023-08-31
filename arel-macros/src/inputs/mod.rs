// mod derive_input;
mod item_input;

// pub use derive_input::DeriveInput;
pub use item_input::ItemInput;

use syn::parse::Parser;

fn get_path_value_from_meta(meta: &syn::Meta, root_attr_path: Option<&str>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
    match meta {
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
                            return Err(syn::Error::new_spanned(
                                &meta,
                                format!(r#"expected `macro({:?})({} = "...")`"#, root_attr_path, allowed_path_names.join("|")),
                            ));
                        }
                    }
                    _ => (),
                }
            }
        }
        syn::Meta::List(list) => {
            if let Some(p) = list.path.segments.first() {
                let should_continue = {
                    if let Some(root_attr_path) = root_attr_path {
                        if p.ident == root_attr_path {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                };
                if should_continue {
                    let nested_metas = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse2(list.tokens.clone()).unwrap();
                    for meta in nested_metas.iter() {
                        match get_path_value_from_meta(meta, None, attr_path, allowed_path_names.clone())? {
                            Some(v) => return Ok(Some(v)),
                            None => continue,
                        }
                    }
                }
            }
        }
        _ => (),
    }
    Ok(None)
}
