// fn _table_name() -> String;
pub(crate) fn impl_table_name(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret_token_stream = proc_macro2::TokenStream::new();
    if let Some((table_name, _)) = input.get_args_path_value(vec![], "table_name", None)? {
        ret_token_stream.extend(quote::quote!(
            fn _table_name() -> String {
                #table_name.into()
            }
        ));
    }
    // if ret_token_stream.is_empty() {
    //     Err(syn::Error::new_spanned(&input.input, r#"Please set arel(table_name = "xxx")"#))
    // } else {
    //     Ok(ret_token_stream)
    // }
    Ok(ret_token_stream)
}
// fn primary_keys() -> Vec<&'static str>;
pub(crate) fn impl_primary_keys(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;
    let mut primary_keys: Vec<String> = vec![];
    for field in fields.iter() {
        if let Some(_) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "primary_key", None)? {
            if let Some(field_ident) = &field.ident {
                if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, vec!["arel"], "rename", None)? {
                    primary_keys.push(rename);
                } else {
                    primary_keys.push(field_ident.to_string().trim_start_matches("r#").to_string());
                }
            }
        }
    }

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    if primary_keys.len() > 0 {
        ret_token_stream.extend(quote::quote!(
            fn primary_keys() -> Vec<&'static str> {
                vec![#(#primary_keys),*]
            }
        ))
    }

    Ok(ret_token_stream)
}

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
            if Self::primary_keys().contains(&#field_name) {
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

// fn assign(&mut self, other: &Self) -> &mut Self;
pub(crate) fn impl_assign(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut assign_fields_clause = vec![];
    for field in fields.iter() {
        let ident = &field.ident;
        assign_fields_clause.push(quote::quote!(
            match &other.#ident {
                arel::ActiveValue::Changed(nv, _) => {
                    self.#ident.set(nv.clone());
                }
                arel::ActiveValue::Unchanged(v) => {
                    self.#ident.set(v.clone());
                }
                arel::ActiveValue::NotSet => ()
            }
        ));
    }

    Ok(quote::quote!(
        fn assign(&mut self, other: &Self) -> &mut Self {
            #(#assign_fields_clause)*
            self
        }
    ))
}

// async fn insert_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = crate::db::Database>;
pub(crate) fn impl_insert_with_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = input.struct_fields()?;

    let mut insert_init_clause = proc_macro2::TokenStream::new();
    // let mut set_primary_id_clause = proc_macro2::TokenStream::new();
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
        // if field_name == "id" {
        //     set_primary_id_clause.extend(quote::quote!(
        //         if let Some(last_insert_id) = val.last_insert_id() {
        //             self.#ident = arel::ActiveValue::Unchanged((last_insert_id as #r#type).into());
        //         }
        //     ));
        // }
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
        async fn insert_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            let mut insert_fields: Vec<&'static str> = vec![];
            let mut insert_values: Vec<arel::Value> = vec![];
            #insert_init_clause

            if let Some(insert_sql) = arel::statements::insert::Insert::<Self>::new(insert_fields, insert_values).to_sql()? {
                *self = insert_sql.fetch_one_as_with_exec(executor).await?;
                Ok(())
                // match insert_sql.fetch_one_with_exec(executor).await {
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

// async fn update_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = arel::db::Database>;
pub(crate) fn impl_update_with_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
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
        async fn update_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            let mut update_fields: Vec<&'static str> = vec![];
            let mut update_values: Vec<arel::Value> = vec![];
            #update_init_clause

            if let Some(update_sql) =arel::statements::update::Update::<Self>::new(update_fields, update_values, Self::primary_keys().clone(), self.primary_values().clone()).to_sql()? {
                *self = update_sql.fetch_one_as_with_exec(executor).await?;
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

// async fn increment_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> arel::Result<()> where E: sqlx::Executor<'a, Database = arel::db::Database>
pub(crate) fn impl_increment_with_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
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
            if update_field == stringify!(ident).to_string().trim_start_matches("r#").to_string() {
                update_field = field_name.to_string();
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
        async fn increment_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            let mut update_field = key.to_string().trim_start_matches("r#").to_string();
            #update_init_clause

            if let Some(increment_sql) = arel::statements::increment::Increment::<Self>::new(update_field, step, Self::primary_keys().clone(), self.primary_values().clone()).to_sql()? {
                *self = increment_sql.fetch_one_as_with_exec(executor).await?;
                Ok(())
            } else {
                Err(arel::Error::Message("sql error".to_string()))
            }
        }
    ))
}

// async fn destroy_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()> where E: arel::sqlx::Executor<'a, Database = crate::db::Database>;
pub(crate) fn impl_destroy_with_exec(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
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
        async fn destroy_with_exec<'a, E>(&mut self, executor: E) -> arel::Result<()>
        where
            E: arel::sqlx::Executor<'a, Database = arel::db::Database>,
        {
            if let Some(destroy_sql) = arel::statements::delete::Delete::<Self>::new(Self::primary_keys().clone(), self.primary_values().clone()).to_sql()? {
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
