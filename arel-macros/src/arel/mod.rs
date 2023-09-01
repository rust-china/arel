mod arel_active_model;
mod arel_model;
mod arel_trait;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;

pub fn create_arel(args: TokenStream, input: TokenStream) -> TokenStream {
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
        syn::Item::Struct(_) => (),
        _ => return Err(syn::Error::new_spanned(&input.input, "arel only allow use on struct type")),
    }

    let model_name_ident = input.ident()?;
    let mut model_fields = vec![];
    for field in input.struct_fields()?.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];
        model_fields.push(new_field);
    }

    let arel_trait_impl_table_name = arel_trait::impl_table_name(input)?;
    let arel_trait_impl_primary_key_or_primary_keys = arel_trait::impl_primary_key_or_primary_keys(input)?;
    let model_impl_trait_sqlx_from_row = impl_trait_sqlx_from_row(input)?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let arel_model = arel_model::create_arel_model(input)?;
    let arel_active_model = arel_active_model::create_arel_active_model(input)?;

    let vis = input.vis()?;
    Ok(quote::quote!(
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #model_name_ident #generics {
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
        #model_impl_trait_sqlx_from_row

        #arel_model
        #arel_active_model
    ))
}

fn impl_trait_sqlx_from_row(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let model_ident = input.ident()?;
    let fields = input.struct_fields()?;

    let mut build_assign_clauses = vec![];
    for field in fields.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        let ident = &field.ident;
        let r#type = &field.ty;

        let field_name = {
            // arel(rename="x")
            if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                rename
            } else {
                match ident {
                    Some(ident) => ident.to_string().trim_start_matches("r#").to_string(),
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")),
                }
            }
        };
        build_assign_clauses.push(quote::quote!(
            // user.#ident = row.try_get::<#r#type, _>(#rename).unwrap_or_default();
            user.#ident = <#r#type as arel::ArelAttributeFromRow>::from_row(&row, #field_name)?;
        ));
    }

    let mut generics = input.generics()?.clone();
    generics.params.push(syn::parse_quote!('_r));
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, type_generics, where_clause) = input.generics()?.split_for_impl();
    Ok(quote::quote!(
        impl #impl_generics arel::sqlx::FromRow<'_r, arel::DatabaseRow> for #model_ident #type_generics #where_clause  {
            fn from_row(row: &'_r arel::DatabaseRow) -> arel::sqlx::Result<Self, arel::sqlx::Error> {
                let mut user = Self::default();
                #(#build_assign_clauses)*
                Ok(user)
            }
        }
    ))
}
