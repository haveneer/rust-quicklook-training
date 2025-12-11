darling_example — procedural macro demo using `darling`

This crate showcases how to build attribute/derive macros using the `darling` crate
for ergonomic parsing, as well as how to manage friendly compile-time errors,
issue warnings, and enable a debug mode during macro development.

What it provides
- `#[derive(Config)]` on a named struct, with a `#[config(...)]` container attribute
  and `#[config(...)]` field attributes.
- Aggregated validation errors with useful spans using `darling::Error`.
- Compile-time warnings on stable Rust by generating and referencing `#[deprecated]`
  items (works without nightly diagnostics).
- A small "debug mode" that prints parsed metadata and generated code when the
  proc-macro crate is built with feature `debug`.
  ```shell
  cargo build -p darling_example --tests --features darling_example/debug
  ```

Usage
```
use darling_example::Config;

#[derive(Config, Debug)]
#[config(env_prefix = "APP_")]
struct AppConfig {
    #[config(rename = "APP_NAME")] // rename the logical name
    name: String,

    #[config(min = 1024, max = 65535)] // validate numeric range
    port: u16,

    #[config(default = false)] // provide a default expression
    debug: bool,

    #[config(deprecated = "`old_feature` is deprecated, use `debug` instead")] // emit a warning
    old_feature: bool,
}

fn main() {
    // Generated helper methods to show how parsed attributes can be used
    assert_eq!(AppConfig::config_field_names(), &["APP_NAME", "port", "debug", "old_feature"]);
    // Env keys are always UPPERCASED (including the prefix)
    assert_eq!(AppConfig::config_env_keys(), &["APP_APP_NAME", "APP_PORT", "APP_DEBUG", "APP_OLD_FEATURE"]);

    // New: build from environment variables using those keys
    // APP_APP_NAME=MyApp APP_PORT=8080 APP_DEBUG=true
    let _cfg = AppConfig::from_env().unwrap();
}
```

Supported attributes
- On the container (struct):
  - `env_prefix = "PREFIX_"` — optional prefix used when generating `config_env_keys()`
    and when looking up environment variables in `from_env()`. The effective
    env keys are always uppercased (prefix + field logical name).
- On fields:
  - `rename = "NAME"` — logical name override.
  - `default = <expr>` — default value expression (stored as a `syn::Expr` in the macro input).
  - `required` — marks a field as required (demo only).
  - `deprecated = "message"` — emits a compile-time warning on usage.
  - `min = <i64>`, `max = <i64>` — example numeric constraints validated by the macro.

Errors and warnings
- Multiple validation issues (e.g., `min > max` on several fields) are aggregated
  using `darling::Error::accumulator()` and returned together as compile errors.
- Warnings are emitted on stable by generating a `#[deprecated]` constant and
  referencing it from a hidden associated const, ensuring the compiler produces
  a warning without requiring nightly `proc_macro_diagnostic`.

Debug feature
Enable the `debug` feature on this proc-macro crate to print parsed metadata
and the generated code to stderr during expansion, e.g.:
```
cargo test -p darling_example --features darling_example/debug
```

Tests and [trybuild](https://crates.io/crates/trybuild) UI cases
- `tests/usage.rs` shows normal usage and asserts the generated helper methods.
- `tests/compile.rs` runs `trybuild` UI tests:
  - `tests/ui-fail/min_gt_max.rs` — fails with a custom error when `min > max`.
  - `tests/ui-fail/wrong_shape.rs` — fails if `Config` is used on an enum (unsupported shape).
  - `tests/ui-fail/unknown_key.rs` — fails on unknown attribute keys parsed by `darling`.
  - `tests/ui-pass/required_default.rs` — compiles successfully but produces a warning
    (required + default). This is not asserted by trybuild; it’s for demonstration.

Tip: When authoring or updating UI tests, run first with:
```
TRYBUILD=overwrite cargo test -p darling_example
```
This will write `.stderr` files next to your `.rs` tests. Then run tests again
without `TRYBUILD=overwrite` to ensure diagnostics match.
