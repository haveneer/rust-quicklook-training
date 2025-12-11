#![allow(clippy::bool_assert_comparison)]

use darling_example::Config;

#[derive(Config, Debug, Default)]
#[config(env_prefix = "APP_")]
struct AppConfig {
    /// renamed logical name
    #[config(rename = "name")]
    app_name: String,

    /// value with min/max constraints
    #[config(min = 1024, max = 65535)]
    port: u16,

    /// defaulted field
    #[config(default = false)]
    debug: bool,

    /// deprecated usage should emit a compile-time warning
    #[config(deprecated = "`old_feature` is deprecated, use `debug` instead")]
    old_feature: bool,
}

#[test]
fn derives_and_exposes_names() {
    let names = AppConfig::config_field_names();
    assert_eq!(names, &["name", "port", "debug", "old_feature"]);
    let env_keys = AppConfig::config_env_keys();
    assert_eq!(
        env_keys,
        &["APP_NAME", "APP_PORT", "APP_DEBUG", "APP_OLD_FEATURE"]
    );
}

#[test]
fn from_env_builds() {
    std::env::set_var("APP_NAME", "MyApp");
    std::env::set_var("APP_PORT", "8080");
    std::env::set_var("APP_DEBUG", "true");
    std::env::remove_var("APP_OLD_FEATURE"); // -> defaults to false

    let cfg = AppConfig::from_env().expect("from_env should succeed");
    assert_eq!(cfg.app_name, "MyApp");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.debug, true);
    assert_eq!(cfg.old_feature, false);
}
