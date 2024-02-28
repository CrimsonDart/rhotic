

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
