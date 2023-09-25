mod arel_model_trait;
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
    let arel_trail = do_expand_arel(input)?;
    let model_sqlx_from_row = do_expand_model_sqlx_from_row(input)?;
    let arel_model_trait = do_expand_arel_model(input)?;
    let arel_model_sqlx_from_row = do_expand_arel_model_sqlx_from_row(input)?;

    Ok(quote::quote!(
        #arel_trail
        #model_sqlx_from_row
        #arel_model_trait
        #arel_model_sqlx_from_row
    ))
}

fn do_expand_arel(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    match &input.input {
        syn::Item::Struct(_) => (),
        _ => return Err(syn::Error::new_spanned(&input.input, "arel only allow use on struct type")),
    }

    let struct_ident = input.ident()?;
    let mut model_fields = vec![];
    for field in input.struct_fields()?.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];
        model_fields.push(new_field);
    }

    let arel_trait_impl_table_name = arel_trait::impl_table_name(input)?;
    let arel_trait_impl_primary_keys = arel_trait::impl_primary_keys(input)?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let vis = input.vis()?;
    Ok(quote::quote!(
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #struct_ident #generics {
            pub __persisted__: bool,
            #(#model_fields),*
        }

        impl #impl_generics arel::SuperArel for #struct_ident #type_generics #where_clause {
            // fn _table_name() -> String;
            #arel_trait_impl_table_name
            // fn primary_keys() -> Vec<&'static str>;
            #arel_trait_impl_primary_keys
        }

        impl #impl_generics arel::ArelPersisted for #struct_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }
    ))
}

// impl<'r> arel::sqlx::FromRow<'r, arel::db::DatabaseRow> for User {
//     fn from_row(row: &'r arel::db::DatabaseRow) -> Result<Self, sqlx::Error> {
//         let mut model = Self::default();
//         model.id = <i32 as arel::ArelAttributeFromRow>::from_row(row, "id")?;
//         model.name = <String as arel::ArelAttributeFromRow>::from_row(row, "name")?;
//         model.age = <Option<i32> as arel::ArelAttributeFromRow>::from_row(row, "age")?;
//         model.gender = <Gender as arel::ArelAttributeFromRow>::from_row(row, "gender")?;
//         model.r#type = <String as arel::ArelAttributeFromRow>::from_row(row, "type")?;
//         model.address = <Option<String> as arel::ArelAttributeFromRow>::from_row(row, "address")?;
//         model.expired_at = <Option<chrono::DateTime<chrono::FixedOffset>> as arel::ArelAttributeFromRow>::from_row(row, "expired_at")?;
//         Ok(model)
//     }
// }
fn do_expand_model_sqlx_from_row(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
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
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")), //不可达
                }
            }
        };
        build_assign_clauses.push(quote::quote!(
            model.#ident = <#r#type as arel::ArelAttributeFromRow>::from_row(&row, #field_name)?;
        ));
    }

    let mut generics = input.generics()?.clone();
    generics.params.push(syn::parse_quote!('_r));
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, type_generics, where_clause) = input.generics()?.split_for_impl();
    Ok(quote::quote!(
        impl #impl_generics arel::sqlx::FromRow<'_r, arel::db::DatabaseRow> for #struct_ident #type_generics #where_clause  {
            fn from_row(row: &'_r arel::db::DatabaseRow) -> arel::sqlx::Result<Self, arel::sqlx::Error> {
                let mut model = Self::default();
                #(#build_assign_clauses)*
                Ok(model)
            }
        }
    ))
}

fn do_expand_arel_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    match &input.input {
        syn::Item::Struct(_) => (),
        _ => return Err(syn::Error::new_spanned(&input.input, "arel only allow use on struct type")),
    }

    let struct_ident = input.ident()?;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());

    let mut model_fields = vec![];
    for field in input.struct_fields()?.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        if let Some(new_type) = arel_model_trait::new_field_type(&field) {
            new_field.ty = syn::parse_quote! { #new_type };
        } else {
            let old_ty = &field.ty;
            new_field.ty = syn::parse_quote! { arel::ActiveValue<#old_ty> };
        }
        model_fields.push(new_field);
    }

    let arel_model_trait_impl_primary_values = arel_model_trait::impl_primary_values(input)?;
    let arel_model_trait_impl_insert_exec = arel_model_trait::impl_insert_exec(input)?;
    let arel_model_trait_impl_update_exec = arel_model_trait::impl_update_exec(input)?;
    let arel_model_trait_from_model = arel_model_trait::impl_from_model(input)?;
    let arel_model_trait_impl_destroy_exec = arel_model_trait::impl_destroy_exec(input)?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let vis = input.vis()?;
    Ok(quote::quote!(
        #[derive(Clone, Debug, PartialEq, Default)]
        #vis struct #arel_model_ident #generics {
            pub __persisted__: bool,
            #(#model_fields),*
        }

        #[arel::async_trait::async_trait]
        impl #impl_generics arel::ArelModel for #arel_model_ident #type_generics #where_clause {
            type Model = #struct_ident #type_generics;
            // fn primary_values(&self) -> Vec<arel::Value>;
            #arel_model_trait_impl_primary_values
            // async fn insert_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_model_trait_impl_insert_exec
            // async fn update_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_model_trait_impl_update_exec
            // async fn save(&mut self) -> arel::Result<()>;
            async fn save(&mut self) -> arel::Result<()> {
                self.save_exec(Self::Model::pool()?).await
            }
            // async fn destroy_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_model_trait_impl_destroy_exec
            // async fn destroy(&mut self) -> arel::Result<()>;
            async fn destroy(&mut self) -> arel::Result<()> {
                self.destroy_exec(Self::Model::pool()?).await
            }
        }

        impl #impl_generics arel::ArelPersisted for #arel_model_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }

        #arel_model_trait_from_model
    ))
}

fn do_expand_arel_model_sqlx_from_row(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());

    let mut generics = input.generics()?.clone();
    generics.params.push(syn::parse_quote!('_r));
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, type_generics, where_clause) = input.generics()?.split_for_impl();
    Ok(quote::quote!(
        impl #impl_generics arel::sqlx::FromRow<'_r, arel::db::DatabaseRow> for #arel_model_ident #type_generics #where_clause  {
            fn from_row(row: &'_r arel::db::DatabaseRow) -> arel::sqlx::Result<Self, arel::sqlx::Error> {
                let mut model = #struct_ident #type_generics::from_row(row)?;
                model.set_persisted(true);
                return Ok(model.into())
            }
        }
    ))
}
