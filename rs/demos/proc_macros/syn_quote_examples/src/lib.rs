use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// 1. TokenStream : affiche les tokens reçus
#[proc_macro]
pub fn show_tokens(input: TokenStream) -> TokenStream {
    println!("Tokens: {}", input);
    input
}

// 2. Parsing avec syn : parse basique
#[proc_macro_derive(ParseExample)]
pub fn parse_example(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // ast.ident : nom de la struct
    // ast.data : champs de la struct
    // ast.attrs : attributs

    println!("Parsed struct: {}", ast.ident);

    TokenStream::new()
}

// 3. Inspection de l'AST : inspecte les champs d'une struct
#[proc_macro_derive(InspectFields)]
pub fn inspect_fields(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    #[allow(clippy::single_match)]
    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let name = &field.ident;
                    let ty = &field.ty;
                    println!("{}: {}", quote!(#name), quote!(#ty));
                }
            }
            _ => {}
        },
        _ => {}
    }

    TokenStream::new()
}

// 4. Génération avec quote : génère impl Display
#[proc_macro_derive(AutoDisplay)]
pub fn auto_display(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}

// 5. Exemple complet avec syn : accès aux informations de la struct
#[proc_macro_derive(SynExample)]
pub fn syn_example(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Accès aux informations
    let struct_name = &ast.ident;
    let generics = &ast.generics;
    let fields = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Only structs supported"),
    };

    println!("Struct: {}", struct_name);
    println!("Generics: {}", quote!(#generics));
    println!("Fields count: {}", fields.len());

    TokenStream::new()
}

// 6. Exemple avec quote : génère un constructeur new()
#[proc_macro_derive(AutoNew)]
pub fn auto_new(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let field_types = fields.iter().map(|f| &f.ty);
    let field_assigns = fields.iter().map(|f| &f.ident);

    let output = quote! {
        impl #name {
            pub fn new(
                #(#field_names: #field_types),*
            ) -> Self {
                Self {
                    #(#field_assigns),*
                }
            }
        }
    };

    TokenStream::from(output)
}
