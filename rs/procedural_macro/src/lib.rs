// More info:
// * https://doc.rust-lang.org/reference/procedural-macros.html
// * https://github.com/dtolnay/syn

// #![feature(proc_macro_quote)] // unstable feature

// extern crate proc_macro;
// extern crate syn;

use proc_macro::TokenStream;

#[proc_macro] // functions tagged with `#[proc_macro]` must currently reside in the root of the crate
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer1() -> u32 { 42 }".parse().unwrap() // parse from string :/
}

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(CountFields)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Dig into input data
    let count = 
    match input.data {
        Data::Enum(ref data) => data.variants.len(),
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.len(),
            Fields::Unnamed(_) => unimplemented!("Fields::Unnamed"),
            Fields::Unit => unimplemented!("Fields::Unit"),
        },
        Data::Union(_) => unimplemented!(),
    };
    println!("{} fields or variants found", count);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics #name #ty_generics #where_clause {
            fn count_field(&self) -> usize {
                #count
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
