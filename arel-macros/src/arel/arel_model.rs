pub(crate) fn create_arel_model(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    let fields = input.struct_fields()?;

    let generics = input.generics()?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut build_arel_model_fields_clauses = vec![];
    for field in fields.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        let r#type = &field.ty;
        // arel(rename="x")
        // if let Some(rename) = crate::get_path_value(input, Some(&field), "rename", None)? {
        //     new_field.attrs.push(syn::parse_quote! {
        //         #[sqlx(rename = #rename)]
        //     });
        // }
        if let Some(inner_type) = crate::get_generic_inner_type(r#type, "Option") {
            new_field.ty = syn::parse_quote! { std::option::Option<#inner_type> };
        } else {
            new_field.ty = syn::parse_quote! { std::option::Option<#r#type> };
        }
        build_arel_model_fields_clauses.push(quote::quote!(
            #new_field
        ));
    }

    let arel_model_impl_trait_sqlx_from_row = impl_trait_sqlx_from_row(input)?;

    let vis = input.vis()?;
    ret_token_stream.extend(quote::quote!(
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #arel_model_ident #generics {
            pub __persisted__: bool,
            #(#build_arel_model_fields_clauses),*
        }
        impl #impl_generics arel::traits::ArelPersisted for #arel_model_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }
        #arel_model_impl_trait_sqlx_from_row
    ));

    Ok(ret_token_stream)
}

fn impl_trait_sqlx_from_row(input: &crate::ItemInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = input.ident()?;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", struct_ident.to_string()), struct_ident.span());
    let fields = input.struct_fields()?;

    let mut build_assign_clauses = vec![];
    for field in fields.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        let ident = &field.ident;
        let r#type = &field.ty;
        let option_type: syn::Type = {
            if let Some(inner_type) = crate::get_generic_inner_type(r#type, "Option") {
                syn::parse_quote! { std::option::Option<#inner_type> }
            } else {
                syn::parse_quote! { std::option::Option<#r#type> }
            }
        };
        // arel(rename="x")
        if let Some((rename, _)) = crate::ItemInput::get_field_path_value(field, "arel", "rename", None)? {
            build_assign_clauses.push(quote::quote!(
                // user.#ident = row.try_get::<#option_type, _>(#rename).unwrap_or_default();
                user.#ident = <#option_type as arel::ArelAttributeFromRow>::from_row(&row, #rename).unwrap_or_default();
            ));
        } else {
            build_assign_clauses.push(quote::quote!(
                // user.#ident = row.try_get::<#option_type, _>(stringify!(#ident)).unwrap_or_default();
                user.#ident = <#option_type as arel::ArelAttributeFromRow>::from_row(&row, stringify!(#ident)).unwrap_or_default();
            ));
        }
    }

    let mut generics = input.generics()?.clone();
    generics.params.push(syn::parse_quote!('_r));
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, type_generics, where_clause) = input.generics()?.split_for_impl();
    Ok(quote::quote!(
        impl #impl_generics arel::sqlx::FromRow<'_r, arel::DatabaseRow> for #arel_model_ident #type_generics #where_clause  {
            fn from_row(row: &'_r arel::DatabaseRow) -> arel::sqlx::Result<Self, arel::sqlx::Error> {
                let mut user = Self::default();
                #(#build_assign_clauses)*
                Ok(user)
            }
        }
    ))
}
