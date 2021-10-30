extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let signature = function.sig;
    let func_name = &signature.ident.to_string();
    let body = function.block;

    quote!(
        #signature {
            println!("> {}", #func_name);
            #body
        }
    )
    .into()
}
