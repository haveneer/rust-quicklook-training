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
