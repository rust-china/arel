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

    Ok(quote::quote!(
        #arel_trail
        #model_sqlx_from_row
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

        if let Some(new_type) = new_field_type(&field) {
            new_field.ty = syn::parse_quote! { #new_type };
        } else {
            let old_ty = &field.ty;
            new_field.ty = syn::parse_quote! { arel::ActiveValue<#old_ty> };
        }
        model_fields.push(new_field);
    }

    let arel_trait_impl_table_name = arel_trait::impl_table_name(input)?;
    let arel_trait_impl_primary_keys = arel_trait::impl_primary_keys(input)?;
    let arel_trait_impl_primary_values = arel_trait::impl_primary_values(input)?;
    let arel_trait_impl_assign = arel_trait::impl_assign(input)?;
    let arel_trait_impl_is_dirty = arel_trait::impl_is_dirty(input)?;
    let arel_trait_impl_insert_with_exec = arel_trait::impl_insert_with_exec(input)?;
    let arel_trait_impl_update_with_exec = arel_trait::impl_update_with_exec(input)?;

    let arel_trait_impl_increment_with_exec = arel_trait::impl_increment_with_exec(input)?;
    let arel_trait_impl_destroy_with_exec = arel_trait::impl_destroy_with_exec(input)?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let vis = input.vis()?;
    Ok(quote::quote!(
        #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        #vis struct #struct_ident #generics {
            #[serde(default,skip_serializing)]
            pub __persisted__: bool,
            #(#model_fields),*
        }

        impl #impl_generics arel::ArelPersisted for #struct_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }

        #[arel::async_trait::async_trait]
        impl #impl_generics arel::SuperArel for #struct_ident #type_generics #where_clause {
            // fn _table_name() -> String;
            #arel_trait_impl_table_name
            // fn primary_keys() -> Vec<&'static str>;
            #arel_trait_impl_primary_keys
            // fn primary_values(&self) -> Vec<arel::Value>;
            #arel_trait_impl_primary_values
            // fn assign(&mut self, other: &Self) -> &mut Self;
            #arel_trait_impl_assign
            // fn is_dirty(&self) -> bool;
            #arel_trait_impl_is_dirty
            // async fn insert_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_trait_impl_insert_with_exec
            // async fn update_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_trait_impl_update_with_exec
            // async fn increment_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = arel::db::Database>
            #arel_trait_impl_increment_with_exec
            // async fn destroy_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = arel::db::Database>;
            #arel_trait_impl_destroy_with_exec
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
            let value = <#r#type as arel::ArelAttributeFromRow>::from_row(&row, #field_name)?;
            model.#ident = arel::ActiveValue::Unchanged(value.into());
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
                model.set_persisted(true);
                #(#build_assign_clauses)*
                Ok(model)
            }
        }
    ))
}

fn new_field_type(field: &syn::Field) -> Option<syn::Type> {
    let r#type = &field.ty;
    let type_str: String = quote::quote!(#r#type).to_string().split_whitespace().collect();

    if !regex::Regex::new(r"Vec").unwrap().is_match(&type_str) {
        if regex::Regex::new(r"bool").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueBool> });
        } else if regex::Regex::new(r"i8").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueTinyInt> });
        } else if regex::Regex::new(r"i16").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueSmallInt> });
        } else if regex::Regex::new(r"i32").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueInt> });
        } else if regex::Regex::new(r"i64").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueBigInt> });
        } else if regex::Regex::new(r"u8").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueTinyUnsigned> });
        } else if regex::Regex::new(r"u16").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueSmallUnsigned> });
        } else if regex::Regex::new(r"u32").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueUnsigned> });
        } else if regex::Regex::new(r"u64").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueBigUnsigned> });
        } else if regex::Regex::new(r"f32").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueFloat> });
        } else if regex::Regex::new(r"f64").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueDouble> });
        } else if regex::Regex::new(r"String").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueString> });
        } else if regex::Regex::new(r"Byptes").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueBytes> });
        } else if regex::Regex::new(r"serde_json::Value").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueJson> });
        } else if regex::Regex::new(r"chrono::DateTime").unwrap().is_match(&type_str) && regex::Regex::new(r"chrono::FixedOffset").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ValueChronoTimestamp> });
        } else if regex::Regex::new(r"chrono::NaiveDateTime").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::NaiveDateTime> });
        } else if regex::Regex::new(r"chrono::NaiveDate").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::NaiveDate> });
        } else if regex::Regex::new(r"chrono::ChronoTime").unwrap().is_match(&type_str) {
            return Some(syn::parse_quote! { arel::ActiveValue<arel::sub_value::ChronoTime> });
        }
    }
    return None;
}
