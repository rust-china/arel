mod arel_active_model;
mod arel_model;
mod arel_trait;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;

pub fn create_arel(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = crate::Input {
        args: if args.is_empty() {
            None
        } else {
            Some(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse(args).unwrap())
        },
        st: syn::parse_macro_input!(input as syn::DeriveInput),
    };

    match do_expand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => {
            let mut ret_token_stream = e.to_compile_error();
            ret_token_stream.extend(input.st.to_token_stream());
            ret_token_stream.into()
        }
    }
}

fn do_expand(input: &crate::Input) -> syn::Result<proc_macro2::TokenStream> {
    let st = &input.st;
    let model_name_ident = &input.st.ident;

    let mut model_fields = vec![];
    for field in crate::get_fields(input)?.iter() {
        let mut new_field = field.clone();
        new_field.attrs = vec![];

        // arel(rename="x")
        if let Some(rename) = crate::get_path_value(input, Some(&field), "rename", None)? {
            new_field.attrs.push(syn::parse_quote! {
                #[sqlx(rename = #rename)]
            });
        }
        model_fields.push(new_field);
    }

    let arel_trait_impl_table_name = arel_trait::impl_table_name(input)?;
    let arel_trait_impl_primary_key_or_primary_keys = arel_trait::impl_primary_key_or_primary_keys(input)?;

    let generics = &st.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let arel_model = arel_model::create_arel_model(input)?;
    let arel_active_model = arel_active_model::create_arel_active_model(input)?;
    Ok(quote::quote!(

        #[derive(Clone, Debug, Default, PartialEq, sqlx::FromRow)]
        pub struct #model_name_ident #generics {
            #[sqlx(default)]
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

        #arel_model
        #arel_active_model
    ))
}
