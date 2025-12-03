use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream};

struct Route {
    method: Ident,
    path: LitStr,
    handler: Ident,
}

impl Parse for Route {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method: Ident = input.parse()?;
        let path: LitStr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let handler: Ident = input.parse()?;
        Ok(Route { method, path, handler })
    }
}

struct Routes {
    routes: Vec<Route>,
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut routes = Vec::new();
        while !input.is_empty() {
            routes.push(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Routes { routes })
    }
}

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let routes_input = parse_macro_input!(input as Routes);

    let methods = routes_input.routes.iter().map(|r| &r.method);
    let paths = routes_input.routes.iter().map(|r| &r.path);
    let handlers = routes_input.routes.iter().map(|r| &r.handler);

    let expanded = quote! {
        pub fn setup_routes() -> Vec<(String, String, fn() -> String)> {
            vec![
                #(
                    (
                        stringify!(#methods).to_string(),
                        #paths.to_string(),
                        #handlers as fn() -> String
                    )
                ),*
            ]
        }
    };

    TokenStream::from(expanded)
}
