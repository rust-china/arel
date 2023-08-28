pub(crate) mod arel;

use proc_macro::TokenStream;
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}

// ============= common =============
pub(crate) struct Input {
    args: Option<syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>>,
    st: syn::DeriveInput,
}

pub(crate) fn get_path_value(input: &Input, field: Option<&syn::Field>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<String>> {
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
                                return Ok(Some(ident_str.value().to_string()));
                            }
                        }
                        _ => return Ok(Some("".to_string())),
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
                            match nested_meta {
                                syn::Meta::NameValue(kv) => {
                                    if kv.path.is_ident(attr_path) {
                                        match &kv.value {
                                            syn::Expr::Lit(expr) => {
                                                if let syn::Lit::Str(ref ident_str) = expr.lit {
                                                    return Ok(Some(ident_str.value().to_string()));
                                                }
                                            }
                                            _ => return Ok(Some("".to_string())),
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
                                syn::Meta::Path(path) => {
                                    for path_segment in &path.segments {
                                        if path_segment.ident == attr_path {
                                            return Ok(Some("".to_string()));
                                        }
                                    }
                                }
                                _ => (),
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

pub(crate) type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;
fn get_fields(input: &Input) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &input.st.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new_spanned(&input.st, "Must Define on Struct, Not on Enum"))
    }
}

fn get_generic_inner_type<'a>(r#type: &'a syn::Type, outer_ident_name: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = r#type {
        if let Some(seg) = segments.last() {
            if seg.ident.to_string() == outer_ident_name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

// PhantomData<M> => "M"
// pub(crate) fn get_phantom_data_generic_type_name(field: &syn::Field) -> syn::Result<Option<String>> {
//     if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = &field.ty {
//         if let Some(segment) = segments.last() {
//             if segment.ident == "PhantomData" {
//                 if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
//                     if let Some(syn::GenericArgument::Type(syn::Type::Path(type_path))) = args.first() {
//                         if let Some(syn::PathSegment { ident, .. }) = type_path.path.segments.first() {
//                             return Ok(Some(ident.to_string()));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     Ok(None)
// }
