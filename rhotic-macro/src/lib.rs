use proc_macro::TokenStream;

mod text_stage_impl;

#[proc_macro_attribute]
pub fn text_and_render(attr: TokenStream, item: TokenStream) -> TokenStream {
    text_stage_impl::impl_text_stage(attr.into(), item.into()).into()
}
