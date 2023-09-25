pub(crate) mod inputs;

pub(crate) mod arel;
pub(crate) mod arel_enum;
pub(crate) use inputs::ItemInput;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}

#[proc_macro_attribute]
pub fn arel_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    arel_enum::create_arel_enum(args, input)
}
