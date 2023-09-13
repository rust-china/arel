pub(crate) mod inputs;

pub(crate) mod arel;
pub(crate) use inputs::ItemInput;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}
