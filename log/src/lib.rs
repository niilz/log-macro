extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn log(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let signature = function.sig;
    let func_name = &signature.ident.to_string();
    let mut body = function.block;

    // Figure out if the function is returning a type
    // If so: store the last statement separately
    let return_statement = match signature.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, _) => match body.stmts.pop() {
            Some(stmt) => quote!(#stmt),
            None => unreachable!("There must be a statement if there is a return_type"),
        },
    };

    quote!(
        #signature {
            println!("> {}", #func_name);
            #body
            println!("< {}", #func_name);

            #return_statement
        }
    )
    .into()
}
