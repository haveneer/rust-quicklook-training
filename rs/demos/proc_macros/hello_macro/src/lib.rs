use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse l'input en AST
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Génère le code
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
