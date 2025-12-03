use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder only works with named fields"),
        },
        _ => panic!("Builder only works on structs"),
    };

    // Génère les champs du builder (tous en Option)
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });

    // Génère les setters
    let setters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    // Génère les champs pour la méthode build
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.ok_or(concat!("Field ", stringify!(#name), " is missing"))?
        }
    });

    // Génère les champs pour new()
    let init_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: None }
    });

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#init_fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
