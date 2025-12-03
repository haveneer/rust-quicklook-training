// Exemple complet d'utilisation de quote
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(QuoteExample)]
pub fn quote_example(_input: TokenStream) -> TokenStream {
    let name = /* ... */;
    let fields = /* ... */;

    let output = quote! {
        impl #name {
            pub fn new(
                #(#fields: #field_types),*
            ) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    TokenStream::from(output)
}
