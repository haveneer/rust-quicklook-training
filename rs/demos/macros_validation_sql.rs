// Exemple de validation SQL à la compilation
// Ceci est un exemple "no-run" - nécessite proc-macro = true dans Cargo.toml

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

// Validation de requête SQL à la compilation
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let sql = input.to_string();

    // Parse et valide le SQL
    match parse_sql(&sql) {
        Ok(_) => {}
        Err(e) => {
            return syn::Error::new(
                Span::call_site(),
                format!("Invalid SQL: {}", e),
            )
            .to_compile_error()
            .into();
        }
    }

    // Génère du code typé
    quote! {
        PreparedStatement::new(#sql)
    }
    .into()
}

fn parse_sql(sql: &str) -> Result<(), String> {
    // Implémentation du parsing SQL
    Ok(())
}

struct PreparedStatement;
