// fn primary_values(&self) -> Vec<arel::Value>;
pub(crate) fn impl_primary_values(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut sub_clauses = vec![];

    let fields = input.struct_fields()?;
    for field in fields.iter() {
        let ident = &field.ident;
        let field_name = {
            if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                rename
            } else {
                match ident {
                    Some(ident) => ident.to_string().trim_start_matches("r#").to_string(),
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")),
                }
            }
        };
        sub_clauses.push(quote::quote!(
            if Self::Model::primary_keys().contains(&#field_name) {
                let value = match &self.#ident {
                    arel::ActiveValue::Changed(nv, ov) => {
                        match ov.as_ref() {
                            arel::ActiveValue::Unchanged(v) => {
                                Some(v.into())
                            },
                            _ => None
                        }
                    },
                    arel::ActiveValue::Unchanged(v) => {
                        Some(v.into())
                    }
                    _ => None
                };
                if let Some(value) = value {
                    primary_values.push(value);
                }
            }
        ));
    }

    Ok(quote::quote!(
        fn primary_values(&self) -> Vec<arel::Value> {
            let mut primary_values = vec![];
            #(#sub_clauses)*
            primary_values
        }
    ))
}

// async fn insert_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = crate::db::Database>;
pub(crate) fn impl_insert_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut insert_init_clause = proc_macro2::TokenStream::new();
    let mut set_primary_id_clause = proc_macro2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        let r#type = &field.ty;
        let field_name = {
            if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                rename
            } else {
                match ident {
                    Some(ident) => ident.to_string().trim_start_matches("r#").to_string(),
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")),
                }
            }
        };
        insert_init_clause.extend(quote::quote!(
            let field_name = #field_name;
            match &self.#ident {
                arel::ActiveValue::Changed(nv, _) => {
                    insert_fields.push(field_name);
                    insert_values.push(nv.into());
                },
                _ => ()
            }
        ));
        if field_name == "id" {
            set_primary_id_clause.extend(quote::quote!(
                if let Some(last_insert_id) = val.last_insert_id() {
                    self.#ident = arel::ActiveValue::Unchanged((last_insert_id as #r#type).into());
                }
            ));
        }
    }

    let mut set_all_to_unchanged_clause = proc_macro2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        // let r#type = &field.ty;
        set_all_to_unchanged_clause.extend(quote::quote!(
            if let arel::ActiveValue::Changed(nv, ov) = &self.#ident {
                self.#ident = arel::ActiveValue::Unchanged(nv.clone());
            }
        ));
    }

    Ok(quote::quote!(
        async fn insert_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            let mut insert_fields: Vec<&'static str> = vec![];
            let mut insert_values: Vec<arel::Value> = vec![];
            #insert_init_clause

            if let Some(insert_sql) = arel::statements::insert::Insert::<Self::Model>::new(insert_fields, insert_values).to_sql()? {
                let mut model: Self::Model = insert_sql.fetch_one_as_exec(executor).await?;
                model.set_persisted(true);
                *self = model.into();
                Ok(())
                // match insert_sql.fetch_one_as(executor).await {
                //     Ok(val) => {
                //         #set_primary_id_clause
                //         #set_all_to_unchanged_clause
                //         self.set_persisted(true);
                //         return Ok(val)
                //     },
                //     Err(err) => Err(err)
                // }
            } else {
                Err(arel::Error::Message("sql error".to_string()))
            }
        }
    ))
}

// async fn update_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = crate::db::Database>;
pub(crate) fn impl_update_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut update_init_clause = proc_macro2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        // let r#type = &field.ty;
        let field_name = {
            if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                rename
            } else {
                match ident {
                    Some(ident) => ident.to_string().trim_start_matches("r#").to_string(),
                    _ => return Err(syn::Error::new_spanned(field, "Field name can not Blank!")),
                }
            }
        };
        update_init_clause.extend(quote::quote!(
            let field_name = #field_name;
            match &self.#ident {
                arel::ActiveValue::Changed(nv, _) => {
                    update_fields.push(field_name);
                    update_values.push(nv.into());
                },
                _ => ()
            }
        ));
    }

    let mut set_all_to_unchanged_clause = proc_macro2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        // let r#type = &field.ty;
        set_all_to_unchanged_clause.extend(quote::quote!(
            if let arel::ActiveValue::Changed(nv, ov) = &self.#ident {
                self.#ident = arel::ActiveValue::Unchanged(nv.clone());
            }
        ));
    }

    Ok(quote::quote!(
        async fn update_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            let mut update_fields: Vec<&'static str> = vec![];
            let mut update_values: Vec<arel::Value> = vec![];
            #update_init_clause

            if let Some(update_sql) =arel::statements::update::Update::<Self::Model>::new(update_fields, update_values, Self::Model::primary_keys().clone(), self.primary_values().clone()).to_sql()? {
                let mut model: Self::Model = update_sql.fetch_one_as_exec(executor).await?;
                model.set_persisted(true);
                *self = model.into();
                Ok(())
                // match update_sql.exec(executor).await {
                //     Ok(val) => {
                //         if val.rows_affected() > 0 {
                //             #set_all_to_unchanged_clause
                //             self.set_persisted(true);
                //             return Ok(())
                //         }
                //         Err(arel::Error::Message("update self failed with rows_affected == 0".to_string()))
                //     },
                //     Err(err) => Err(err)
                // }
            } else {
                Err(arel::Error::Message("sql error".to_string()))
            }
        }
    ))
}

// async fn destroy_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = crate::db::Database>;
pub(crate) fn impl_destroy_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut set_all_to_changed_clause = proc_macro2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        // let r#type = &field.ty;
        set_all_to_changed_clause.extend(quote::quote!(
            match &mut self.#ident {
                arel::ActiveValue::Changed(nv, ov) => {
                    *ov = std::boxed::Box::new(arel::ActiveValue::NotSet);
                },
                arel::ActiveValue::Unchanged(v) => {
                    self.#ident = arel::ActiveValue::Changed(v.clone(), std::boxed::Box::new(arel::ActiveValue::NotSet));
                },
                _ => ()
            }
        ));
    }
    Ok(quote::quote!(
        async fn destroy_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            if let Some(destroy_sql) = arel::statements::delete::Delete::<Self::Model>::new(Self::Model::primary_keys().clone(), self.primary_values().clone()).to_sql()? {
                match destroy_sql.exec(executor).await {
                    Ok(val) => {
                        if val.rows_affected() > 0 {
                            #set_all_to_changed_clause
                            self.set_persisted(false);
                            return Ok(());
                        }
                        Err(arel::Error::Message("destroy self failed with rows_affected == 0".to_string()))
                    }
                    Err(err) => Err(err),
                }
            } else {
                Err(arel::Error::Message("sql error".to_string()))
            }
        }
    ))
}

pub fn impl_from_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());

    let fields = input.struct_fields()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut init_clauses = vec![];
    for field in fields.iter() {
        let ident = &field.ident;

        init_clauses.push(quote::quote!(
            arel_model.#ident = {
                if model.persited() {
                    arel::ActiveValue::Unchanged(model.#ident.clone().into())
                } else {
                    arel::ActiveValue::Changed(model.#ident.clone().into(), std::boxed::Box::new(arel::ActiveValue::NotSet))
                }
            };
        ));
    }

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    ret_token_stream.extend(quote::quote!(
        impl #impl_generics From<#struct_ident #type_generics> for #arel_model_ident #type_generics #where_clause {
            fn from(model: #struct_ident #type_generics) -> #arel_model_ident #type_generics {
                let mut arel_model = Self::default();
                arel_model.set_persisted(model.persited());
                #(#init_clauses)*
                arel_model
            }
        }
    ));

    Ok(ret_token_stream)
}

pub fn new_field_type(field: &syn::Field) -> Option<syn::Type> {
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
