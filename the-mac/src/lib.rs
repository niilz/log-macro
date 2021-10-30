extern crate proc_macro;

use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Signature};

#[proc_macro_attribute]
pub fn log(_args: TokenStream, input: TokenStream) -> TokenStream {
    let signature = input
        .clone()
        .into_iter()
        .take_while(|i| !is_brace(i))
        .collect();
    let signature = parse_macro_input!(signature as Signature);

    let mut body = input.into_iter().skip_while(|i| !is_brace(i));
    let body = body.next().unwrap().into();
    let body = parse_macro_input!(body as proc_macro2::Group);

    quote!(
        #signature {
            println!("function starts");
            #body
        }
    )
    .into()
}

fn is_brace(item: &TokenTree) -> bool {
    match item {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
