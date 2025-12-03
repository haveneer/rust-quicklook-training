use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;

#[proc_macro]
pub fn validated_regex(input: TokenStream) -> TokenStream {
    let regex_lit = syn::parse_macro_input!(input as LitStr);
    let regex_str = regex_lit.value();

    // Validation à la compilation
    if let Err(e) = regex::Regex::new(&regex_str) {
        let error = format!("Invalid regex: {}", e);
        return syn::Error::new(Span::call_site(), error)
            .to_compile_error()
            .into();
    }

    let expanded = quote! {
        regex::Regex::new(#regex_str).unwrap()
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn validated_sql(input: TokenStream) -> TokenStream {
    let sql_lit = syn::parse_macro_input!(input as LitStr);
    let sql_str = sql_lit.value();

    // Validation basique du SQL (on vérifie juste qu'il contient SELECT/INSERT/UPDATE/DELETE)
    let sql_upper = sql_str.to_uppercase();
    let valid = sql_upper.contains("SELECT")
        || sql_upper.contains("INSERT")
        || sql_upper.contains("UPDATE")
        || sql_upper.contains("DELETE");

    if !valid {
        let error = "Invalid SQL: must contain SELECT, INSERT, UPDATE, or DELETE";
        return syn::Error::new(Span::call_site(), error)
            .to_compile_error()
            .into();
    }

    let expanded = quote! {
        #sql_str
    };

    TokenStream::from(expanded)
}
