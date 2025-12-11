use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

// This example demonstrates using `darling` to parse container and field attributes
// for a derive macro, with error aggregation, compile-time warnings, and an
// optional debug mode printing parsed metadata and generated code.

use darling::{ast, util, FromDeriveInput, FromField};

// The fields are mainly named as they are from parse_macro_input!(input as DeriveInput)
// + custom attributes (e.g. `env_prefix`)
#[derive(FromDeriveInput)]
#[darling(attributes(config), supports(struct_named))]
struct Container {
    /// Identifier of the struct we derive on
    ident: syn::Ident,

    /// Parsed fields (ignore the variant-level data because we only support named structs)
    data: ast::Data<util::Ignored, Field>,

    /// Optional prefix to use (container-level option): `#[config(env_prefix = "APP_")]`
    #[darling(default)]
    env_prefix: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(config))]
struct Field {
    /// Field identifier (present for named fields)
    ident: Option<syn::Ident>,
    /// Field type
    ty: syn::Type,

    /// Rename the logical name (e.g. to map to an external source)
    #[darling(default)]
    rename: Option<String>,

    /// Default value expression: `#[config(default = 42)]` or `#[config(default = "expr()")]`
    #[darling(default)]
    default: Option<syn::Expr>,

    /// Field is required (no default allowed); demonstration of boolean flags
    #[darling(default)]
    required: bool,

    /// Mark field as deprecated and emit a compile-time warning: `#[config(deprecated = "use X instead")]`
    #[darling(default)]
    deprecated: Option<String>,

    /// Validation example: min/max constraints on numeric fields
    #[darling(default)]
    min: Option<i64>,
    #[darling(default)]
    max: Option<i64>,
}

#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Let `darling` parse our input. If parsing fails (unknown options, wrong shape,
    // etc.), we turn the error(s) into compile errors with spans.
    let container = match Container::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Aggregate validation errors manually to show how to report several issues at once.
    let mut acc = darling::Error::accumulator();

    let fields: Vec<&Field> = match &container.data {
        ast::Data::Struct(fields) => fields.iter().collect(),
        _ => unreachable!("supports(struct_named) ensured by darling"),
    };

    for f in &fields {
        // 1) Validate min/max constraints
        if let (Some(min), Some(max)) = (f.min, f.max) {
            if min > max {
                acc.push(
                    darling::Error::custom("`min` must be <= `max`")
                        .with_span(&f.ident.as_ref().unwrap_or(&container.ident)),
                );
            }
        }

        // 2) required + default don't make sense together -> produce a soft warning.
        // We'll implement warnings by generating deprecated items that we reference,
        // so we only record the info here.
    }

    if let Err(errs) = acc.finish() {
        return errs.write_errors().into();
    }

    // Build output: demonstrate generating a helper method exposing the effective field names
    // (taking `rename` into account) and force compile-time warnings for `deprecated` fields.
    let ident = &container.ident;

    // Effective names slice
    let effective_names: Vec<String> = fields
        .iter()
        .map(|f| {
            if let Some(r) = &f.rename {
                r.clone()
            } else {
                f.ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "<unnamed>".to_string())
            }
        })
        .collect();

    // Prepare reusable token lists for names used in two places below
    let name_lits: Vec<proc_macro2::TokenStream> =
        effective_names.iter().map(|s| quote! { #s }).collect();
    let name_lits_a = name_lits.clone();

    // Prepare uppercased names for env keys and an uppercased container-level prefix if any
    let upper_names: Vec<String> = effective_names
        .iter()
        .map(|s| s.to_ascii_uppercase())
        .collect();
    let upper_name_lits: Vec<proc_macro2::TokenStream> =
        upper_names.iter().map(|s| quote! { #s }).collect();

    let upper_prefix = container
        .env_prefix
        .as_deref()
        .unwrap_or("")
        .to_ascii_uppercase();
    let prefix_lit = syn::LitStr::new(&upper_prefix, proc_macro2::Span::call_site());

    // Create compile-time deprecation warnings: for each deprecated field, declare a
    // #[deprecated] const and reference it from an associated const. Referencing a
    // deprecated item triggers a warning on stable Rust.
    let mut deprecated_consts = Vec::new();
    let mut deprecated_uses = Vec::new();
    for f in &fields {
        if let (Some(field_ident), Some(msg)) = (&f.ident, &f.deprecated) {
            let const_ident =
                format_ident!("__DARLING_EXAMPLE_DEPRECATED_{}_{}", ident, field_ident);
            deprecated_consts.push(quote! {
                #[allow(dead_code)]
                #[deprecated(note = #msg)]
                const #const_ident: () = ();
            });
            // Reference it to actually emit the warning during compilation.
            deprecated_uses.push(quote! {
                #[allow(deprecated, unreachable_code, dead_code)]
                { let _ = #const_ident; }
            });
        }
        // Also warn if `required` and `default` are both set.
        if let (Some(field_ident), true) = (&f.ident, f.required && f.default.is_some()) {
            let msg = format!(
                "`{}` is marked `required` but also has `default`; `default` will be ignored",
                field_ident
            );
            let const_ident =
                format_ident!("__DARLING_EXAMPLE_REQ_DEFAULT_{}_{}", ident, field_ident);
            deprecated_consts.push(quote! {
                #[allow(dead_code)]
                #[deprecated(note = #msg)]
                const #const_ident: () = ();
            });
            deprecated_uses.push(quote! {
                #[allow(deprecated, unreachable_code, dead_code)]
                { let _ = #const_ident; }
            });
        }
    }

    // Debug print: when feature `debug` is enabled on this proc-macro crate,
    // print parsed metadata and the generated tokens. This writes during macro expansion.
    if cfg!(feature = "debug") {
        let field_summaries: Vec<String> = fields
            .iter()
            .map(|f| format!(
                "{}: ty={:?}, rename={:?}, default={:?}, required={}, deprecated={:?}, min={:?}, max={:?}",
                f.ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "<unnamed>".to_string()),
                f.ty,
                f.rename,
                f.default.as_ref().map(|e| quote!{#e}.to_string()),
                f.required,
                f.deprecated,
                f.min,
                f.max
            ))
            .collect();
        eprintln!(
            "[darling_example::Config] {} fields: {}",
            ident,
            field_summaries.join(" | ")
        );
    }

    // Build field initialization code for `from_env()`
    let mut init_fields = Vec::<proc_macro2::TokenStream>::new();
    for (idx, f) in fields.iter().enumerate() {
        let field_ident = f
            .ident
            .as_ref()
            .expect("named fields guaranteed by darling supports(struct_named)");
        let ty = &f.ty;

        // Env key for this field: UPPER(prefix) + UPPER(effective_name)
        let key_str = format!("{}{}", upper_prefix, upper_names[idx]);
        let key_lit = syn::LitStr::new(&key_str, proc_macro2::Span::call_site());

        // Is this field a String? If so, we don't parse.
        let is_string = match &f.ty {
            syn::Type::Path(tp) => tp
                .path
                .segments
                .last()
                .map(|seg| seg.ident == "String")
                .unwrap_or(false),
            _ => false,
        };

        // Default expression if provided
        let default_expr = f.default.as_ref();
        let required = f.required;

        // Min/Max constraints (checked at runtime if present)
        let min_check = if let Some(min) = f.min {
            let min_val = min;
            Some(quote! {
                let __v_num = v as i128;
                if __v_num < #min_val as i128 {
                    return Err(format!("{} is below min {} (got {})", #key_lit, #min_val, v));
                }
            })
        } else {
            None
        };

        let max_check = if let Some(max) = f.max {
            let max_val = max;
            Some(quote! {
                let __v_num = v as i128;
                if __v_num > #max_val as i128 {
                    return Err(format!("{} exceeds max {} (got {})", #key_lit, #max_val, v));
                }
            })
        } else {
            None
        };

        let parse_value = if is_string {
            quote! { s }
        } else {
            quote! { s.parse::<#ty>().map_err(|e| format!("Failed to parse {} for {}: {}", #key_lit, stringify!(#field_ident), e))? }
        };

        let value_code = if let Some(def) = default_expr {
            quote! {
                match ::std::env::var(#key_lit) {
                    Ok(s) => {
                        let v: #ty = { #parse_value };
                        { #min_check #max_check }
                        v
                    }
                    Err(_) => { #def }
                }
            }
        } else if required {
            quote! {
                match ::std::env::var(#key_lit) {
                    Ok(s) => {
                        let v: #ty = { #parse_value };
                        { #min_check #max_check }
                        v
                    }
                    Err(_) => return Err(format!("Missing required env var {}", #key_lit)),
                }
            }
        } else {
            quote! {
                match ::std::env::var(#key_lit) {
                    Ok(s) => {
                        let v: #ty = { #parse_value };
                        { #min_check #max_check }
                        v
                    }
                    Err(_) => <#ty as ::core::default::Default>::default(),
                }
            }
        };

        init_fields.push(quote! { #field_ident: { #value_code } });
    }

    let expanded = quote! {
        // Emit deprecated consts at module level
        #(#deprecated_consts)*

        impl #ident {
            /// Example generated method: exposes the effective field names after `rename`.
            pub fn config_field_names() -> &'static [&'static str] {
                &[ #(#name_lits_a),* ]
            }

            /// Example generated method: exposes environment keys using the optional prefix.
            pub fn config_env_keys() -> &'static [&'static str] {
                &[ #( concat!(#prefix_lit, #upper_name_lits) ),* ]
            }

            /// Initialize the configuration from the process environment.
            ///
            /// Each field is read from an uppercased env key built from the container-level
            /// `env_prefix` (also uppercased) plus the field's logical name (uppercased).
            /// - If the variable is set, its value is parsed into the field type (`String` uses the raw value).
            /// - If not set and a `default = <expr>` is provided, the default expression is used.
            /// - If not set and the field is `required`, an error is returned.
            /// - Otherwise, `Default::default()` is used.
            pub fn from_env() -> ::core::result::Result<Self, ::std::string::String> {
                let instance = Self {
                    #(#init_fields),*
                };
                Ok(instance)
            }

            // Force warnings for deprecated fields at compile time by referencing
            // the deprecated consts in a never-used associated const.
            const __DARLING_EXAMPLE_TRIGGER_WARNINGS: () = {
                #(#deprecated_uses)*
            };
        }
    };

    if cfg!(feature = "debug") {
        eprintln!(
            "[darling_example::Config] generated for {} =>\n{}",
            ident,
            expanded.to_string()
        );
    }

    TokenStream::from(expanded)
}
