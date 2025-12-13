use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(MemoryLayout)]
pub fn derive_memory_layout(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Ensure it's a struct with named fields
    let fields_named = match input.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(named) => named,
            _ => {
                return quote! {
                    compile_error!("#[derive(MemoryLayout)] only supports structs with named fields");
                }
                .into();
            }
        },
        _ => {
            return quote! {
                compile_error!("#[derive(MemoryLayout)] only supports structs");
            }
            .into();
        }
    };

    // Build arrays of field data
    let mut field_names = Vec::new();
    let mut field_idents = Vec::new();
    let mut field_types = Vec::new();

    for (idx, f) in fields_named.named.iter().enumerate() {
        let ident = f.ident.clone().expect("named field ident");
        let ty = &f.ty;
        let name_str = ident.to_string();
        field_names.push(name_str);
        field_idents.push(ident);
        field_types.push(quote! { #ty });
        let _ = idx; // original index used later in generation
    }

    // Generate an internal struct to help sorting
    let field_info_ident = format_ident!("__FieldInfoFor{}", name);

    let orig_indices: Vec<usize> = (0..field_idents.len()).collect();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn print_layout() {
                use core::mem::{size_of, align_of};
                use core::mem::offset_of;

                #[allow(non_camel_case_types)]
                struct #field_info_ident {
                    name: &'static str,
                    offset: usize,
                    size: usize,
                    ty_name: &'static str,
                    orig_index: usize,
                }

                println!("Memory layout for {}:", stringify!(#name));
                let total_size = size_of::<Self>();
                let align = align_of::<Self>();
                println!("  Total size: {} bytes", total_size);
                println!("  Alignment: {} bytes", align);
                println!("");

                let mut fields: ::std::vec::Vec<#field_info_ident> = ::std::vec![
                    #( #field_info_ident {
                        name: #field_names,
                        offset: offset_of!(Self, #field_idents),
                        size: size_of::<#field_types>(),
                        ty_name: ::core::any::type_name::<#field_types>(),
                        orig_index: #orig_indices,
                    } ),*
                ];

                // Sort by offset
                fields.sort_by_key(|f| f.offset);

                let mut prev_end = 0usize;
                for f in &fields {
                    if f.offset > prev_end {
                        let padding = f.offset - prev_end;
                        println!("  {:20} @ {:3}  ({:3} bytes)", "[padding]", prev_end, padding);
                    }
                    // Align the name to a fixed width similar to the example
                    println!(
                        "  {:20} @ offset={:3}  ({:3} bytes)  [orig: {:2}]  type={}",
                        f.name,
                        f.offset,
                        f.size,
                        f.orig_index + 1,
                        f.ty_name,
                    );
                    prev_end = f.offset + f.size;
                }

                if total_size > prev_end {
                    let padding = total_size - prev_end;
                    println!("  {:20} @ offset={:3}  ({:3} bytes)", "[padding]", prev_end, padding);
                }
            }
        }
    };

    TokenStream::from(expanded)
}
