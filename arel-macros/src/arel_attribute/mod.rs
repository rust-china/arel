use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;

pub fn create_arel_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
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
        _ => return Err(syn::Error::new_spanned(&input.input, "arel_attribute only allow use on enum type")),
    }

    let model_name_ident = input.ident()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut type_to_value_clauses = vec![];
    let mut value_to_type_clauses = vec![];
    let mut enum_variants = vec![];
    for variant in input.enum_variants()? {
        let metas = variant.attrs.iter().map(|attr| &attr.meta).collect::<Vec<&syn::Meta>>();
        let mut new_variant = variant.clone();
        new_variant.attrs = vec![];

        if let Some((_, lit)) = crate::ItemInput::get_path_value_from_metas(metas, "arel_attribute", "value", None)? {
            if let Some(lit) = lit {
                match &lit {
                    syn::Lit::Int(int) => {
                        let value = int.base10_parse::<i32>()?;
                        type_to_value_clauses.push(quote::quote!(
                            #model_name_ident #type_generics::#new_variant => #value.into()
                        ));
                        value_to_type_clauses.push(quote::quote!(
                           #value => Self::#new_variant
                        ));
                    }
                    syn::Lit::Str(str) => {
                        let value = str.value();
                        type_to_value_clauses.push(quote::quote!(
                            #model_name_ident #type_generics::#new_variant => #value.into()
                        ));
                        value_to_type_clauses.push(quote::quote!(
                           #value => Self::#new_variant
                        ));
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(&input.input, "arel_attribute value type not support"));
                    }
                }
            }
        }
        enum_variants.push(new_variant);
    }

    value_to_type_clauses.push(quote::quote!(
        v @ _ => {
            return Err(arel::sqlx::Error::Decode(format!("value: {} can not decode", v).into()));
        }
    ));

    let vis = input.vis()?;
    Ok(quote::quote!(
            #[derive(Clone, Debug, PartialEq)]
            #vis enum #model_name_ident #generics {
                #(#enum_variants),*
            }

            impl #impl_generics arel::ArelAttributeFromRow for #model_name_ident #type_generics #where_clause {
                fn from_row<'r, I>(row: &'r arel::DatabaseRow, index: I) -> arel::sqlx::Result<Self, sqlx::Error>
                where
                    Self: Sized,
                    I: arel::sqlx::ColumnIndex<arel::DatabaseRow>,
                {
                    let value = row.try_get(index)?;
                    let ret = match value {
                        #(#value_to_type_clauses),*
                    };
                    Ok(ret)
                }
            }
            impl #impl_generics From<#model_name_ident #type_generics> for arel::Value {
                fn from(value: #model_name_ident #type_generics) -> Self {
                    match value {
                        #(#type_to_value_clauses),*
                    }
                }
            }
    ))
}
