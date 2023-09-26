use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;

pub fn create_arel_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = crate::ItemInput {
        args: if args.is_empty() {
            None
        } else {
            Some(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse(args).unwrap())
        },
        input: syn::parse_macro_input!(input as syn::Item),
    };

    match do_expand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => {
            let mut ret_token_stream = e.to_compile_error();
            ret_token_stream.extend(input.input.to_token_stream());
            ret_token_stream.into()
        }
    }
}

fn do_expand(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    match &input.input {
        syn::Item::Enum(_) => (),
        _ => return Err(syn::Error::new_spanned(&input.input, "arel_enum only allow use on enum type")),
    }
    let enum_struct_impl = impl_enum_struct(input)?;
    let arel_attribute_from_row_impl = impl_trait_arel_attribute_from_row(input)?;
    let from_self_to_value_impl = impl_from_self_to_value(input)?;
    let default_impl = impl_default(input)?;
    Ok(quote::quote!(
        #enum_struct_impl
        #arel_attribute_from_row_impl
        #from_self_to_value_impl
        #default_impl
    ))
}

fn impl_enum_struct(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut enum_variants = vec![];
    for variant in input.enum_variants()? {
        let mut new_variant = variant.clone();
        new_variant.attrs = vec![];
        enum_variants.push(new_variant);
    }

    let enum_ident = input.ident()?;
    let generics = input.generics()?;

    let vis = input.vis()?;
    Ok(quote::quote!(
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #vis enum #enum_ident #generics {
            #(#enum_variants),*
        }
    ))
}

fn impl_trait_arel_attribute_from_row(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = input.ident()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut value_to_type_clauses = vec![];
    for variant in input.enum_variants()? {
        let metas = variant.attrs.iter().map(|attr| &attr.meta).collect::<Vec<&syn::Meta>>();
        let mut new_variant = variant.clone();
        new_variant.attrs = vec![];
        if let Some((_, lit)) = crate::ItemInput::get_path_value_from_metas(metas, vec!["arel_enum"], "value", None)? {
            if let Some(lit) = lit {
                match &lit {
                    syn::Lit::Int(int) => {
                        let value = int.base10_parse::<i32>()?;
                        value_to_type_clauses.push(quote::quote!(
                           #value => Self::#new_variant
                        ));
                    }
                    syn::Lit::Str(str) => {
                        let value = str.value();
                        value_to_type_clauses.push(quote::quote!(
                           #value => Self::#new_variant
                        ));
                    }
                    syn::Lit::Bool(bool) => {
                        let value = bool.value();
                        value_to_type_clauses.push(quote::quote!(
                           #value => Self::#new_variant
                        ));
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(&input.input, "arel_enum value type not support"));
                    }
                }
            }
        }
    }
    value_to_type_clauses.push(quote::quote!(
        v @ _ => {
            return Err(arel::sqlx::Error::Decode(format!("value: {} can not decode", v).into()));
        }
    ));
    Ok(quote::quote!(
        impl #impl_generics arel::ArelAttributeFromRow for #enum_ident #type_generics #where_clause {
            fn from_row<'r, I>(row: &'r arel::db::DatabaseRow, index: I) -> arel::sqlx::Result<Self, arel::sqlx::Error>
            where
                Self: Sized,
                I: arel::sqlx::ColumnIndex<arel::db::DatabaseRow>,
            {
                let value = row.try_get(index)?;
                let ret = match value {
                    #(#value_to_type_clauses),*
                };
                Ok(ret)
            }
        }
    ))
}

fn impl_from_self_to_value(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = input.ident()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut type_to_value_clauses = vec![];
    for variant in input.enum_variants()? {
        let metas = variant.attrs.iter().map(|attr| &attr.meta).collect::<Vec<&syn::Meta>>();
        let mut new_variant = variant.clone();
        new_variant.attrs = vec![];

        if let Some((_, lit)) = crate::ItemInput::get_path_value_from_metas(metas, vec!["arel_enum"], "value", None)? {
            if let Some(lit) = lit {
                match &lit {
                    syn::Lit::Int(int) => {
                        let value = int.base10_parse::<i32>()?;
                        type_to_value_clauses.push(quote::quote!(
                            #enum_ident #type_generics::#new_variant => #value.into()
                        ));
                    }
                    syn::Lit::Str(str) => {
                        let value = str.value();
                        type_to_value_clauses.push(quote::quote!(
                            #enum_ident #type_generics::#new_variant => #value.into()
                        ));
                    }
                    syn::Lit::Bool(bool) => {
                        let value = bool.value();
                        type_to_value_clauses.push(quote::quote!(
                            #enum_ident #type_generics::#new_variant => #value.into()
                        ));
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(&input.input, "arel_enum value type not support"));
                    }
                }
            }
        }
    }

    Ok(quote::quote!(
        impl #impl_generics From<#enum_ident #type_generics> for arel::Value #where_clause {
            fn from(value: #enum_ident #type_generics) -> Self {
                match value {
                    #(#type_to_value_clauses),*
                }
            }
        }
    ))
}

fn impl_default(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = input.ident()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut default_token_stream = proc_macro2::TokenStream::new();
    let mut default_count = 0;
    for variant in input.enum_variants()? {
        let mut new_variant = variant.clone();
        new_variant.attrs = vec![];
        let metas = variant.attrs.iter().map(|attr| &attr.meta).collect::<Vec<&syn::Meta>>();
        if let Some((_, lit)) = crate::ItemInput::get_path_value_from_metas(metas, vec!["arel_enum"], "default", None)? {
            if let Some(lit) = lit {
                match &lit {
                    syn::Lit::Bool(bool) => {
                        let value = bool.value();
                        if value == false {
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            default_count += 1;
            default_token_stream.extend(quote::quote!(
                Self::#new_variant
            ));
        }
    }

    match default_count {
        1 => Ok(quote::quote!(
            impl #impl_generics Default for #enum_ident #type_generics #where_clause {
                fn default() -> Self {
                    #default_token_stream
                }
            }
        )),
        0 => Err(syn::Error::new_spanned(&input.input, "arel_enum: must set a default value.")),
        _ => Err(syn::Error::new_spanned(&input.input, "arel_enum set default support only one value.")),
    }
}
