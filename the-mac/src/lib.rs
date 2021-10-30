extern crate proc_macro;

use proc_macro::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::{Brace, Group},
    Signature,
};

#[proc_macro_attribute]
pub fn log(_args: TokenStream, input: TokenStream) -> TokenStream {
    let signature = get_signature(input.clone());
    let signature = parse_macro_input!(signature as Signature);
    dbg!(signature.ident);
    quote!(
        struct Dummy {}
    )
    .into()
}

fn get_signature(input: TokenStream) -> TokenStream {
    input
        .into_iter()
        .take_while(|i| match i {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => false,
            _ => true,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
