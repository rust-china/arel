use syn::spanned::Spanned;

pub(crate) fn create_arel_active_model(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let arel_active_model_ident = syn::Ident::new(&format!("ArelActive{}", st.ident.to_string()), st.span());

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    let fields = super::get_fields(input)?;

    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut build_arel_active_model_fields_clauses = vec![];
    let mut build_arel_active_default_init_clauses = vec![];
    for field in fields.iter() {
        let ident = &field.ident;
        let r#type = &field.ty;

        if let Some(inner_type) = super::get_generic_inner_type(r#type, "Option") {
            build_arel_active_model_fields_clauses.push(quote::quote!(
                #ident: arel::ActiveValue<#inner_type>
            ));
        } else {
            build_arel_active_model_fields_clauses.push(quote::quote!(
                #ident: arel::ActiveValue<#r#type>
            ));
        }

        build_arel_active_default_init_clauses.push(quote::quote!(
            #ident: arel::ActiveValue::NotSet
        ));
    }

    let impl_fns = impl_fns(input)?;

    let impl_from_model = impl_from_model(input)?;
    let impl_from_arel_model = impl_from_arel_model(input)?;

    ret_token_stream.extend(quote::quote!(
        #[derive(Clone, Debug, PartialEq)]
        pub struct #arel_active_model_ident #generics {
            pub __persisted__: bool,
            #(pub #build_arel_active_model_fields_clauses),*
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
        impl #impl_generics #arel_active_model_ident #type_generics #where_clause {
            #impl_fns
        }
        #impl_from_model
        #impl_from_arel_model
    ));

    Ok(ret_token_stream)
}

fn impl_from_model(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let model_name_ident = &st.ident;
    let fields = super::get_fields(input)?;
    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut init_clauses = vec![quote::quote!(arel_active_model.__persisted__ = value.__persisted__;)];
    for field in fields.iter() {
        let ident = &field.ident;
        let r#type = &field.ty;
        if let Some(_) = super::get_generic_inner_type(r#type, "Option") {
            init_clauses.push(quote::quote!(
                if let Some(v) = &value.#ident {
                    if value.__persisted__ {
                        arel_active_model.#ident = arel::ActiveValue::Unchanged(v.clone());
                    } else {
                        arel_active_model.#ident = arel::ActiveValue::Changed(v.clone(), std::boxed::Box::new(arel::ActiveValue::NotSet));
                    }
                }
            ));
        } else {
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

fn impl_from_arel_model(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let arel_model_name_ident = syn::Ident::new(&format!("Arel{}", st.ident.to_string()), st.span());
    let fields = super::get_fields(input)?;
    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut init_clauses = vec![quote::quote!(arel_active_model.__persisted__ = value.__persisted__;)];
    for field in fields.iter() {
        let ident = &field.ident;
        // let r#type = &field.ty;
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

fn impl_fns(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let model_ident = &input.st.ident;
    let fields = super::get_fields(input)?;
    let mut ret_token_stream = proc_macro2::TokenStream::new();

    // pub fn to_sql(&self) -> anyhow::Result<crate::Sql>
    {
        let mut update_fields_init_clause = proc_macro2::TokenStream::new();

        let mut insert_fields_init_clause = proc_macro2::TokenStream::new();
        for field in fields.iter() {
            let ident = &field.ident;
            // let r#type = &field.ty;
            update_fields_init_clause.extend(quote::quote!(
                let field_name = stringify!(#ident);
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

            insert_fields_init_clause.extend(quote::quote!(
                match &self.#ident {
                    arel::ActiveValue::Changed(nv, _) => {
                        insert_fields.push(stringify!(#ident));
                        insert_values.push(nv.into());
                    },
                    _ => ()
                }
            ));
        }

        ret_token_stream.extend(quote::quote!(
            pub fn to_sql(&self) -> anyhow::Result<arel::Sql> {
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
                if self.__persisted__ {
                    let mut update_fields: Vec<&'static str> = vec![];
                    let mut update_values: Vec<arel::Value> = vec![];
                    let mut update_where_fields: Vec<&'static str> = vec![];
                    let mut update_where_values: Vec<arel::Value> = vec![];
                    #update_fields_init_clause

                    final_sql.push_str(format!(r#"UPDATE "{}" SET"#, table_name));
                    let len = update_fields.len();
                    for (idx, field) in update_fields.iter().enumerate() {
                        let value = &update_values[idx];
                        let mut sql = arel::Sql::default();
                        sql.push_str_with_prepare_value(format!(r#""{}" = {}"#, field, sql.prepare_symbol()), value.clone());
                        final_sql.push_sql(sql);
                        if idx < len - 1 {
                            final_sql.push_str(", ");
                        }
                    }

                    final_sql.push_str(" WHERE ");
                    let len = update_where_fields.len();
                    for (idx, field) in update_where_fields.iter().enumerate() {
                        let value = &update_where_values[idx];
                        let mut sql = arel::Sql::default();
                        sql.push_str_with_prepare_value(format!(r#""{}" = {}"#, field, sql.prepare_symbol()), value.clone());
                        final_sql.push_sql(sql);
                        if idx < len - 1 {
                            final_sql.push_str(" AND ");
                        }
                    }
                } else {
                    let mut insert_fields: Vec<&'static str> = vec![];
                    let mut insert_values: Vec<arel::Value> = vec![];
                    #insert_fields_init_clause

                    final_sql.push_str(format!(r#"INSERT INTO "{}""#, table_name));

                    let mut field_sql = arel::Sql::new("(");
                    let mut value_sql = arel::Sql::new("(");
                    let len = insert_fields.len();
                    for (idx, field) in insert_fields.iter().enumerate() {
                        let value = &insert_values[idx];
                        field_sql.push_str(format!(r#""{}""#, field));
                        value_sql.push_str_with_prepare_value(value_sql.prepare_symbol(), value.clone());
                        if idx < len - 1 {
                            field_sql.push_str(", ");
                            value_sql.push_str(", ");
                        }
                    }
                    field_sql.push(')');
                    value_sql.push(')');
                    final_sql.push_sql(field_sql).push_str(" VALUES ").push_sql(value_sql);
                }
                Ok(final_sql)
            }
        ));
    }
    // pub async fn save_exec<'a, E>(&mut self) -> anyhow::Result<arel::DatabaseQueryResult> where E: sqlx::Executor<'a, Database = arel::Database>
    {
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
        ret_token_stream.extend(quote::quote!(
            pub async fn save_exec<'a, E>(&mut self, executor: E) -> anyhow::Result<arel::DatabaseQueryResult> where E: sqlx::Executor<'a, Database = arel::Database> {
                match self.to_sql()?.exec(executor).await {
                    Ok(val) => {
                        #set_all_to_unchanged_clause
                        self.__persisted__ = true;
                        Ok(val)
                    },
                    Err(err) => Err(err)
                }
            }
        ));
    }
    // pub fn save(&mut self) -> anyhow::Result<arel::DatabaseQueryResult>
    {
        ret_token_stream.extend(quote::quote!(
            pub async fn save(&mut self) -> anyhow::Result<arel::DatabaseQueryResult> {
                self.save_exec(#model_ident::pool()?).await
            }
        ));
    }

    Ok(ret_token_stream)
}
