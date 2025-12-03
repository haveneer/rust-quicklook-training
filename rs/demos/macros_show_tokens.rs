// Exemple de TokenStream - affiche les tokens reçus
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;

#[proc_macro]
pub fn show_tokens(input: TokenStream) -> TokenStream {
    println!("Tokens: {}", input);
    input
}

// Utilisation:
// show_tokens!(1 + 2)
// Affiche: "1 + 2"
