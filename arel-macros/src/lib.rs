pub(crate) mod arel;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}
