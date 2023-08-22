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
                    arel_active_model.#ident = arel::ActiveValue::Unchanged(v.clone());
                }
            ));
        } else {
            init_clauses.push(quote::quote!(
                arel_active_model.#ident = arel::ActiveValue::Unchanged(value.#ident.clone());
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
                arel_active_model.#ident = arel::ActiveValue::Unchanged(v.clone());
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
