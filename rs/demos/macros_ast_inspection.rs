// Exemple d'inspection de l'AST
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(InspectFields)]
pub fn inspect_fields(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match &ast.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    for field in &fields.named {
                        let name = &field.ident;
                        let ty = &field.ty;
                        println!("{:?}: {:?}", name, ty);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    TokenStream::new()
}
