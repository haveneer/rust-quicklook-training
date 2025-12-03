// Exemple de macro attribute pour mesurer le temps d'exécution
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("{} took {:?}", stringify!(#fn_name), duration);
            result
        }
    };

    TokenStream::from(expanded)
}

// Utilisation (dans un autre fichier):
// use timing_macro::timing;
//
// #[timing]
// fn expensive_computation() -> u64 {
//     std::thread::sleep(std::time::Duration::from_millis(100));
//     42
// }
