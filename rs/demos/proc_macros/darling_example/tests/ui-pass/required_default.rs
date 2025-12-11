use darling_example::Config;

// This should compile (but emit a warning): `required` together with `default`.
#[derive(Config)]
struct WarnConfig {
    #[config(required, default = 1)]
    value: i32,
}

fn main() {
    // Touch generated API to ensure expansion happens fully
    let _names = WarnConfig::config_field_names();
}
