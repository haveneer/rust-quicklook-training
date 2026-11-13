// Custom serde (de)serialization: when `#[derive]` is not enough.
// Expected JSON: {"theme": "default"} or {"theme": "<theme-file-path>"}
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug, PartialEq, Default)]
enum Theme {
    #[default]
    Default,
    Custom(String),
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ThemeVisitor;

        impl Visitor<'_> for ThemeVisitor {
            type Value = Theme;

            // Used in error messages
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("`default` or <theme-file-path>")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Theme, E> {
                Ok(match value {
                    "default" => Theme::Default,
                    path => Theme::Custom(path.to_string()),
                })
            }
        }

        deserializer.deserialize_str(ThemeVisitor)
    }
}

// Symmetric format: a derived Serialize would write {"Custom":"..."}
impl Serialize for Theme {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Theme::Default => serializer.serialize_str("default"),
            Theme::Custom(path) => serializer.serialize_str(path),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
struct Config {
    theme: Theme,
}

fn main() {
    for json in [
        r#"{"theme":"default"}"#,
        r#"{"theme":"dark.toml"}"#,
        r#"{"theme":42}"#,
    ] {
        match serde_json::from_str::<Config>(json) {
            Ok(config) => {
                let back = serde_json::to_string(&config).unwrap();
                println!("{json:22} => {config:?} => {back}");
            }
            Err(error) => println!("{json:22} => error: {error}"),
        }
    }
}

#[test]
fn test() {
    main()
}

// Lighter: go through a type serde knows
mod via_string {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(from = "String", into = "String")]
    pub enum Theme {
        Default,
        Custom(String),
    }

    impl From<String> for Theme {
        fn from(s: String) -> Self {
            if s == "default" {
                Theme::Default
            } else {
                Theme::Custom(s)
            }
        }
    }

    impl From<Theme> for String {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Default => "default".to_string(),
                Theme::Custom(path) => path,
            }
        }
    }

    #[test]
    fn via_string() {
        for json in [r#""default""#, r#""dark.toml""#, "42"] {
            match serde_json::from_str::<Theme>(json) {
                Ok(theme) => {
                    let back = serde_json::to_string(&theme).unwrap();
                    println!("{json:12} => {theme:?} => {back}");
                }
                Err(error) => println!("{json:12} => error: {error}"),
            }
        }
    }
}

// Checks only (not shown on slides)

#[test]
fn custom_deserialize() {
    let config: Config = serde_json::from_str(r#"{"theme": "dark.toml"}"#).unwrap();
    assert_eq!(config.theme, Theme::Custom("dark.toml".to_string()));

    let config: Config = serde_json::from_str(r#"{"theme": "default"}"#).unwrap();
    assert_eq!(config, Config::default());

    let error = serde_json::from_str::<Config>(r#"{"theme": 42}"#).unwrap_err();
    assert_eq!(
        error.to_string(),
        "invalid type: integer `42`, expected `default` or <theme-file-path> at line 1 column 12"
    );
}

#[test]
fn custom_serialize_roundtrip() {
    let config = Config {
        theme: Theme::Custom("dark.toml".to_string()),
    };
    let json = serde_json::to_string(&config).unwrap();
    assert_eq!(json, r#"{"theme":"dark.toml"}"#);
    assert_eq!(serde_json::from_str::<Config>(&json).unwrap(), config);
}

#[test]
fn via_string_roundtrip() {
    use via_string::Theme;
    let theme: Theme = serde_json::from_str(r#""default""#).unwrap();
    assert_eq!(theme, Theme::Default);
    let json = serde_json::to_string(&Theme::Custom("dark.toml".to_string())).unwrap();
    assert_eq!(json, r#""dark.toml""#);
    let error = serde_json::from_str::<Theme>("42").unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("invalid type: integer `42`, expected a string")
    );
}
