// Exemple de function-like macro
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    let expanded = quote! {
        fn answer() -> u32 {
            42
        }
    };

    TokenStream::from(expanded)
}

// Utilisation:
// use my_macro::make_answer;
//
// make_answer!();
//
// fn main() {
//     println!("{}", answer()); // 42
// }
