mod arel_active_model;
mod arel_model;
mod arel_trait;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;

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

    let mut model_fields = vec![];
    for field in get_fields(input)?.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        // arel(rename="x")
        if let Some(rename) = get_path_value(input, Some(&field), "rename", None)? {
            new_field.attrs.push(syn::parse_quote! {
                #[sqlx(rename = #rename)]
            });
        }
        model_fields.push(new_field);
    }

    let arel_trait_impl_table_name = arel_trait::impl_table_name(input)?;
    let arel_trait_impl_primary_key_or_primary_keys = arel_trait::impl_primary_key_or_primary_keys(input)?;

    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let arel_model = arel_model::create_arel_model(input)?;
    let arel_active_model = arel_active_model::create_arel_active_model(input)?;
    Ok(quote::quote!(

        #[derive(Clone, Debug, Default, PartialEq, sqlx::FromRow)]
        pub struct #model_name_ident #generics {
            #[sqlx(default)]
            pub __persisted__: bool,
            #(#model_fields),*
        }

        impl #impl_generics arel::SuperArel for #model_name_ident #type_generics #where_clause {
            #arel_trait_impl_table_name
            #arel_trait_impl_primary_key_or_primary_keys
        }
        impl #impl_generics arel::traits::ArelPersisted for #model_name_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }

        #arel_model
        #arel_active_model
    ))
}

fn get_path_value(input: &Input, field: Option<&syn::Field>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<String>> {
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
