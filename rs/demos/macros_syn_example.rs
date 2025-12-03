// Exemple complet d'utilisation de syn
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SynExample)]
pub fn syn_example(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Accès aux informations
    let struct_name = &ast.ident;
    let generics = &ast.generics;
    let fields = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Only structs"),
    };

    // Génération de code basée sur ces informations
    // ...

    TokenStream::new()
}
