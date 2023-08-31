pub struct DeriveInput {
    pub args: Option<syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>>,
    /**
     * input: syn::parse_macro_input!(input as syn::DeriveInput)
     * */
    pub input: syn::DeriveInput,
}

impl DeriveInput {
    pub fn fields(&self) -> syn::Result<&syn::punctuated::Punctuated<syn::Field, syn::Token![,]>> {
        if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) = &self.input.data
        {
            Ok(named)
        } else {
            Err(syn::Error::new_spanned(&self.input, "Must call on struct"))
        }
    }
}

impl DeriveInput {
    pub fn get_field_path_value(field: &syn::Field, root_attr_path: &str, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
        let metas = field.attrs.iter().map(|f| &f.meta).collect::<Vec<&syn::Meta>>();
        Self::get_path_value_from_metas(metas, root_attr_path, attr_path, allowed_path_names)
    }
    pub fn get_path_value_from_metas(metas: Vec<&syn::Meta>, root_attr_path: &str, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
        for meta in metas {
            match super::get_path_value_from_meta(meta, Some(root_attr_path), attr_path, allowed_path_names.clone())? {
                Some(v) => return Ok(Some(v)),
                None => continue,
            }
        }
        Ok(None)
    }
}
