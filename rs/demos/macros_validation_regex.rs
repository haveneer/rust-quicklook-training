// Exemple de validation de regex à la compilation
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let regex_str = input.to_string();

    // Validation à la compilation
    if let Err(e) = regex::Regex::new(&regex_str) {
        panic!("Invalid regex: {}", e);
    }

    let expanded = quote! {
        regex::Regex::new(#regex_str).unwrap()
    };

    TokenStream::from(expanded)
}

// Utilisation:
// let re = regex!(r"^\d{3}-\d{4}$");
// ✅ Compile
// let bad = regex!(r"[invalid");
// ❌ Erreur à la compilation
