pub struct ItemInput {
    pub args: Option<syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>>,
    pub input: syn::Item,
}

impl ItemInput {
    pub fn ident(&self) -> syn::Result<&syn::Ident> {
        match &self.input {
            syn::Item::Struct(st) => Ok(&st.ident),
            syn::Item::Enum(en) => Ok(&en.ident),
            _ => Err(syn::Error::new_spanned(&self.input, "Define Type Only Support: Struct, Enum")),
        }
    }
    pub fn generics(&self) -> syn::Result<&syn::Generics> {
        match &self.input {
            syn::Item::Struct(st) => Ok(&st.generics),
            syn::Item::Enum(en) => Ok(&en.generics),
            _ => Err(syn::Error::new_spanned(&self.input, "Define Type Only Support: Struct, Enum")),
        }
    }
    pub fn vis(&self) -> syn::Result<&syn::Visibility> {
        match &self.input {
            syn::Item::Struct(st) => Ok(&st.vis),
            syn::Item::Enum(en) => Ok(&en.vis),
            _ => Err(syn::Error::new_spanned(&self.input, "Define Type Only Support: Struct, Enum")),
        }
    }
    pub fn struct_fields(&self) -> syn::Result<&syn::Fields> {
        match &self.input {
            syn::Item::Struct(st) => Ok(&st.fields),
            _ => Err(syn::Error::new_spanned(&self.input, "Must Call on Struct")),
        }
    }
    #[allow(dead_code)]
    pub fn enum_variants(&self) -> syn::Result<&syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>> {
        match &self.input {
            syn::Item::Enum(en) => Ok(&en.variants),
            _ => Err(syn::Error::new_spanned(&self.input, "Must Call on Enum")),
        }
    }
    pub fn get_args_path_value(&self, root_attr_paths: Vec<&str>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
        let metas = match &self.args {
            Some(metas) => metas.iter().map(|f| f).collect::<Vec<&syn::Meta>>(),
            None => return Ok(None),
        };
        Self::get_path_value_from_metas(metas, root_attr_paths, attr_path, allowed_path_names)
    }
}

impl ItemInput {
    pub fn get_field_path_value(field: &syn::Field, root_attr_paths: Vec<&str>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
        let metas = field.attrs.iter().map(|f| &f.meta).collect::<Vec<&syn::Meta>>();
        Self::get_path_value_from_metas(metas, root_attr_paths, attr_path, allowed_path_names)
    }
    pub fn get_path_value_from_metas(metas: Vec<&syn::Meta>, root_attr_paths: Vec<&str>, attr_path: &str, allowed_path_names: Option<Vec<&str>>) -> syn::Result<Option<(String, Option<syn::Lit>)>> {
        for meta in metas {
            match super::get_path_value_from_meta(meta, root_attr_paths.clone(), attr_path, allowed_path_names.clone())? {
                Some(v) => return Ok(Some(v)),
                None => continue,
            }
        }
        Ok(None)
    }
}
