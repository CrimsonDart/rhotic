use proc_macro2::{TokenStream, TokenTree, Delimiter};
use quote::quote;






pub fn impl_text_stage(attr: TokenStream, item: TokenStream) -> TokenStream {

    for token in item.clone().into_iter() {
        println!("{token}");
    }

    let mut token_iter = item.clone().into_iter();

    let mut pub_struct = false;

    if let Some(t) = token_iter.next() {
        if t.to_string() == "pub" {
            pub_struct = true;

            if let Some(n) = token_iter.next() {
                if n.to_string() == "struct" {
                    println!("struct")
                } else {
                    return quote!(compile_error!("The \"impl_text_stage\" macro only works on structs."));
                }
            } else {
                return quote!(compile_error!("Token Stream not long enough!"));
            }
        } else if t.to_string() == "struct" {
            println!("struct")
        } else {
            return quote!(compile_error!("The \"impl_text_stage\" macro only works on structs."));
        }
    } else {
        return quote!(compile_error!("Token Stream not long enough!"));
    }

    let struct_name = if let Some(TokenTree::Ident(i)) = token_iter.next() {
        i
    } else {
        return quote!(compile_error!("Token Stream not long enough!"));
    };

    let inner = if let Some(TokenTree::Group(p)) = token_iter.next() {

        if p.delimiter() != Delimiter::Brace {
            return quote!(compile_error!("The \"impl_text_stage\" macro can't work for Unit and Tuple Structs."));
        } else {
            println!("{{");
        }
        p.stream()
    } else {
        return quote!(compile_error!("Token Stream not long enough!"));
    };

    let mut appendix = quote!(
        pub page: crate::buffer::text_buffer::Page,
        pub cursor_x: usize,
        pub cursor_y: usize,
    );

    appendix.extend(inner.into_iter());

    let mut out = quote!();

    if pub_struct {
        out.extend(quote!(pub).into_iter());
    }

    out.extend(
        quote!(
            struct #struct_name {
                #appendix
            }
        ).into_iter()
    );

    println!("{out}");

    out
}






#[cfg(test)]
mod test {
    use quote::quote;

    use super::impl_text_stage;

    #[test]
    fn print_out() {
        impl_text_stage(quote!(), quote!(
            pub struct Struct {
                pub value: usize
            }
        ));
        panic!();
    }
}
