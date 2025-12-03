// Exemple de génération automatique du pattern Builder
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Utilisation:
// #[derive(Builder)]
// struct User {
//     name: String,
//     age: u32,
//     #[builder(default)]
//     email: Option<String>,
// }
//
// Génère automatiquement:
// let user = User::builder()
//     .name("Alice".to_string())
//     .age(30)
//     .email(Some("alice@example.com".into()))
//     .build();

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = extract_fields(&input);

    // Génère la struct Builder
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });

    // Génère les setters
    let setters = fields.iter().map(|f| generate_setter(f));

    let expanded = quote! {
        struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(self) -> Result<#name> {
                // ...
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_fields(input: &DeriveInput) -> Vec<Field> {
    vec![]
}

fn generate_setter(field: &Field) -> proc_macro2::TokenStream {
    quote! {}
}

struct Field;
