use syn::spanned::Spanned;

pub(crate) fn create_arel_model(input: &super::Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let arel_model_ident = syn::Ident::new(&format!("Arel{}", st.ident.to_string()), st.span());

    let mut ret_token_stream = proc_macro2::TokenStream::new();
    let fields = super::get_fields(input)?;

    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut build_arel_model_fields_clauses = vec![];
    for field in fields.iter() {
        let ident = &field.ident;
        let r#type = &field.ty;
        if let Some(inner_type) = super::get_generic_inner_type(r#type, "Option") {
            build_arel_model_fields_clauses.push(quote::quote!(
                #ident: std::option::Option<#inner_type>
            ));
        } else {
            build_arel_model_fields_clauses.push(quote::quote!(
                #ident: std::option::Option<#r#type>
            ));
        }
    }
    ret_token_stream.extend(quote::quote!(
        #[derive(Clone, Debug, Default, PartialEq, sqlx::FromRow)]
        pub struct #arel_model_ident #generics {
            #[sqlx(default)]
            pub __persisted__: bool,
            #(pub #build_arel_model_fields_clauses),*
        }
        impl #impl_generics arel::traits::ArelPersisted for #arel_model_ident #type_generics #where_clause {
            fn set_persisted(&mut self, persisted: bool) {
                self.__persisted__ = persisted;
            }
            fn persited(&self) -> bool {
                self.__persisted__
            }
        }
    ));

    Ok(ret_token_stream)
}
