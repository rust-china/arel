pub(crate) fn create_arel_active_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_active_model_ident = syn::Ident::new(&format!("ArelActive{}", struct_ident.to_string()), struct_ident.span());

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    let fields = input.struct_fields()?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut build_arel_active_default_init_clauses = vec![];
    let mut model_fields = vec![];
    for field in fields.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        let ident = &field.ident;
        let r#type = &field.ty;

        new_field.vis = syn::parse_quote! { pub };
        new_field.ty = syn::parse_quote! { arel::ActiveValue<#r#type> };
        model_fields.push(new_field);

        build_arel_active_default_init_clauses.push(quote::quote!(
            #ident: arel::ActiveValue::NotSet
        ));
    }

    let impl_from_model = impl_from_model(input)?;
    let impl_from_arel_model = impl_from_arel_model(input)?;

    let impl_trait_to_insert_sql = impl_trait_to_insert_sql(input)?;
    let impl_trait_to_update_sql = impl_trait_to_update_sql(input)?;
    let impl_trait_to_destroy_sql = impl_trait_to_destroy_sql(input)?;
    let impl_trait_save_exec = impl_trait_save_exec(input)?;
    let impl_trait_destroy_exec = impl_trait_destroy_exec(input)?;

    let vis = input.vis()?;
    ret_token_stream.extend(quote::quote!(
        #[derive(Clone, Debug, PartialEq)]
        #vis struct #arel_active_model_ident #generics {
            pub __persisted__: bool,
            #(#model_fields),*
        }
        impl #impl_generics Default for #arel_active_model_ident #type_generics #where_clause {
            fn default() -> Self {
                Self {
                    __persisted__: false,
                    #(#build_arel_active_default_init_clauses),*
                }
            }
        }
        impl #impl_generics arel::traits::ArelPersisted for #arel_active_model_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }
        #impl_from_model
        #impl_from_arel_model

        #[arel::async_trait::async_trait]
        impl #impl_generics arel::ArelActiveModel for #arel_active_model_ident #type_generics #where_clause {
            type Model = #struct_ident #type_generics;

            #impl_trait_to_insert_sql
            #impl_trait_to_update_sql
            #impl_trait_to_destroy_sql
            #impl_trait_save_exec
            async fn save(&mut self) -> arel::anyhow::Result<arel::DatabaseQueryResult> {
                self.save_exec(Self::Model::pool()?).await
            }
            #impl_trait_destroy_exec
            async fn destroy(&mut self) -> anyhow::Result<arel::DatabaseQueryResult> {
                self.destroy_exec(Self::Model::pool()?).await
            }
        }
    ));

    Ok(ret_token_stream)
}

fn impl_from_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let model_name_ident = struct_ident;
    let fields = input.struct_fields()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut init_clauses = vec![quote::quote!(arel_active_model.__persisted__ = value.__persisted__;)];
    for field in fields.iter() {
        let ident = &field.ident;

        init_clauses.push(quote::quote!(
            arel_active_model.#ident = {
                if value.__persisted__ {
                    arel::ActiveValue::Unchanged(value.#ident.clone())
                } else {
                    arel::ActiveValue::Changed(value.#ident.clone(), std::boxed::Box::new(arel::ActiveValue::NotSet))
                }
            };
        ));
    }

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    ret_token_stream.extend(quote::quote!(
        impl #impl_generics From<#model_name_ident #type_generics> for ArelActiveUser #type_generics #where_clause {
            fn from(value: #model_name_ident #type_generics) -> ArelActiveUser #type_generics {
                let mut arel_active_model = Self::default();
                #(#init_clauses)*
                arel_active_model
            }
        }
    ));

    Ok(ret_token_stream)
}

fn impl_from_arel_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_model_name_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());
    let fields = input.struct_fields()?;
    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut init_clauses = vec![quote::quote!(arel_active_model.__persisted__ = value.__persisted__;)];
    for field in fields.iter() {
        let ident = &field.ident;
        let r#type = &field.ty;
        if let Some(_) = crate::get_generic_inner_type(r#type, "Option") {
            init_clauses.push(quote::quote!(
                if value.__persisted__ {
                    arel_active_model.#ident = arel::ActiveValue::Unchanged(value.#ident.clone());
                } else {
                    arel_active_model.#ident = arel::ActiveValue::Changed(value.#ident.clone(), std::boxed::Box::new(arel::ActiveValue::NotSet));
                }
            ));
        } else {
            init_clauses.push(quote::quote!(
                if let Some(v) = &value.#ident {
                    if value.__persisted__ {
                        arel_active_model.#ident = arel::ActiveValue::Unchanged(v.clone());
                    } else {
                        arel_active_model.#ident = arel::ActiveValue::Changed(v.clone(), std::boxed::Box::new(arel::ActiveValue::NotSet));
                    }
                }
            ));
        }
    }

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    ret_token_stream.extend(quote::quote!(
        impl #impl_generics From<#arel_model_name_ident #type_generics> for ArelActiveUser #type_generics #where_clause {
            fn from(value: #arel_model_name_ident #type_generics) -> ArelActiveUser #type_generics {
                let mut arel_active_model = Self::default();
                #(#init_clauses)*
                arel_active_model
            }
        }
    ));

    Ok(ret_token_stream)
}

fn impl_trait_to_insert_sql(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let model_ident = struct_ident;
    let fields = input.struct_fields()?;

    let mut insert_fields_init_clause = proc_macro2::TokenStream::new();
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

        insert_fields_init_clause.extend(quote::quote!(
            let field_name = #field_name;
            match &self.#ident {
                arel::ActiveValue::Changed(nv, _) => {
                    insert_fields.push(field_name);
                    insert_values.push(nv.into());
                },
                _ => ()
            }
        ));
    }

    Ok(quote::quote!(
        fn to_insert_sql(&self) -> arel::anyhow::Result<arel::Sql> {
            let primary_keys = {
                if let Some(keys) = #model_ident::primary_keys() {
                    keys
                } else if let Some(key) = #model_ident::primary_key() {
                    vec![key]
                } else {
                    vec![]
                }
            };
            if primary_keys.len() == 0 {
                return Err(anyhow::anyhow!("primary key/keys MUST SET"));
            }

            let table_name = #model_ident::table_name();
            let mut final_sql = arel::Sql::new("");

            let mut insert_fields: Vec<&'static str> = vec![];
            let mut insert_values: Vec<arel::Value> = vec![];
            #insert_fields_init_clause

            if let Some(sql) = arel::statements::insert::Insert::<#model_ident>::new(insert_fields, insert_values).to_sql() {
                final_sql = sql;
            }
            Ok(final_sql)
        }
    ))
}

fn impl_trait_to_update_sql(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let model_ident = struct_ident;
    let fields = input.struct_fields()?;

    let mut update_fields_init_clause = proc_macro2::TokenStream::new();
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

        update_fields_init_clause.extend(quote::quote!(
            let field_name = #field_name;
            match &self.#ident {
                arel::ActiveValue::Changed(nv, ov) => {
                    update_fields.push(field_name);
                    update_values.push(nv.into());
                    if primary_keys.contains(&field_name.into()) {
                        match ov.as_ref() {
                            arel::ActiveValue::Unchanged(v) => {
                                update_where_fields.push(field_name);
                                update_where_values.push(v.into());
                            },
                            _ => ()
                        }

                    }
                },
                arel::ActiveValue::Unchanged(v) => {
                    if primary_keys.contains(&field_name.into()) {
                        update_where_fields.push(field_name);
                        update_where_values.push(v.into());
                    }
                }
                _ => ()
            }
        ));
    }

    Ok(quote::quote!(
        fn to_update_sql(&self) -> arel::anyhow::Result<arel::Sql> {
            let primary_keys = {
                if let Some(keys) = #model_ident::primary_keys() {
                    keys
                } else if let Some(key) = #model_ident::primary_key() {
                    vec![key]
                } else {
                    vec![]
                }
            };
            if primary_keys.len() == 0 {
                return Err(anyhow::anyhow!("primary key/keys MUST SET"));
            }

            let table_name = #model_ident::table_name();
            let mut final_sql = arel::Sql::new("");

            let mut update_fields: Vec<&'static str> = vec![];
            let mut update_values: Vec<arel::Value> = vec![];
            let mut update_where_fields: Vec<&'static str> = vec![];
            let mut update_where_values: Vec<arel::Value> = vec![];
            #update_fields_init_clause

            if update_where_fields.len() == 0 {
                return Err(anyhow::anyhow!("Update where statement is blank!"));
            }
            if let Some(sql) = arel::statements::update::Update::<#model_ident>::new(update_fields, update_values, update_where_fields, update_where_values).to_sql() {
                final_sql = sql;
            }

            Ok(final_sql)
        }
    ))
}

fn impl_trait_to_destroy_sql(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let model_ident = struct_ident;
    let fields = input.struct_fields()?;

    let mut delete_fields_init_clause = proc_macro2::TokenStream::new();
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

        delete_fields_init_clause.extend(quote::quote!(
            let field_name = #field_name;
            match &self.#ident {
                arel::ActiveValue::Changed(nv, ov) => {
                    if primary_keys.contains(&field_name.into()) {
                        match ov.as_ref() {
                            arel::ActiveValue::Unchanged(v) => {
                                delete_where_fields.push(field_name);
                                delete_where_values.push(v.into());
                            },
                            _ => ()
                        }

                    }
                },
                arel::ActiveValue::Unchanged(v) => {
                    if primary_keys.contains(&field_name.into()) {
                        delete_where_fields.push(field_name);
                        delete_where_values.push(v.into());
                    }
                }
                _ => ()
            }
        ));
    }

    Ok(quote::quote!(
        fn to_destroy_sql(&self) -> arel::anyhow::Result<arel::Sql> {
            let primary_keys = {
                if let Some(keys) = #model_ident::primary_keys() {
                    keys
                } else if let Some(key) = #model_ident::primary_key() {
                    vec![key]
                } else {
                    vec![]
                }
            };
            if primary_keys.len() == 0 {
                return Err(anyhow::anyhow!("Primary key/keys MUST SET"));
            }

            let table_name = #model_ident::table_name();
            let mut final_sql = arel::Sql::new("");
            if self.__persisted__ {
                let mut delete_where_fields: Vec<&'static str> = vec![];
                let mut delete_where_values: Vec<arel::Value> = vec![];
                #delete_fields_init_clause

                if delete_where_fields.len() == 0 {
                    return Err(anyhow::anyhow!("Update where statement is blank!"));
                }
                if let Some(sql) = arel::statements::delete::Delete::<#model_ident>::new(delete_where_fields, delete_where_values).to_sql() {
                    final_sql = sql;
                }
            } else {
                return Err(anyhow::anyhow!("Model is Not Persisted"));
            }
            Ok(final_sql)
        }
    ))
}

fn impl_trait_save_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

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
        async fn save_exec<'a, E>(&mut self, executor: E) -> arel::anyhow::Result<arel::DatabaseQueryResult> where E: arel::sqlx::Executor<'a, Database = arel::Database> {
            let save_sql = {
                if self.__persisted__ {
                    self.to_update_sql()?
                } else {
                    self.to_insert_sql()?
                }
            };
            match save_sql.exec(executor).await {
                Ok(val) => {
                    #set_all_to_unchanged_clause
                    self.__persisted__ = true;
                    Ok(val)
                },
                Err(err) => Err(err)
            }
        }
    ))
}

fn impl_trait_destroy_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
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
        async fn destroy_exec<'a, E>(&mut self, executor: E) -> arel::anyhow::Result<arel::DatabaseQueryResult> where E: arel::sqlx::Executor<'a, Database = arel::Database> {
            match self.to_destroy_sql()?.exec(executor).await {
                Ok(val) => {
                    #set_all_to_changed_clause
                    self.__persisted__ = false;
                    Ok(val)
                },
                Err(err) => Err(err)
            }
        }
    ))
}
