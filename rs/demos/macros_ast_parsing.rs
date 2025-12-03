// Exemple de parsing avec syn
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // ast.ident : nom de la struct
    // ast.data : champs de la struct
    // ast.attrs : attributs
    // ...

    TokenStream::new()
}
