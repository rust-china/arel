pub(crate) mod inputs;

pub(crate) mod arel;
pub(crate) mod arel_attribute;

pub(crate) use inputs::ItemInput;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}

#[proc_macro_attribute]
pub fn arel_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    arel_attribute::create_arel_attribute(args, input)
}

// ============= common =============

/**
 * input.st: syn::parse_macro_input!(input as syn::DeriveInput)
 * */
// pub(crate) type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;
// fn get_fields(input: &Input) -> syn::Result<&StructFields> {
//     if let syn::Data::Struct(syn::DataStruct {
//         fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
//         ..
//     }) = &input.st.data
//     {
//         Ok(named)
//     } else {
//         Err(syn::Error::new_spanned(&input.st, "Must Define on Struct, Not on Enum"))
//     }
// }

fn get_generic_inner_type<'a>(r#type: &'a syn::Type, outer_ident_name: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = r#type {
        if let Some(seg) = segments.last() {
            if seg.ident.to_string() == outer_ident_name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

// PhantomData<M> => "M"
// pub(crate) fn get_phantom_data_generic_type_name(field: &syn::Field) -> syn::Result<Option<String>> {
//     if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = &field.ty {
//         if let Some(segment) = segments.last() {
//             if segment.ident == "PhantomData" {
//                 if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
//                     if let Some(syn::GenericArgument::Type(syn::Type::Path(type_path))) = args.first() {
//                         if let Some(syn::PathSegment { ident, .. }) = type_path.path.segments.first() {
//                             return Ok(Some(ident.to_string()));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     Ok(None)
// }
