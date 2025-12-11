use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};

// TODO: add example with deserailize_with: https://stackoverflow.com/a/46755370/12430075
//   see also DeserializeOwned: https://users.rust-lang.org/t/serde-generics-implement-deserialize-with-lifetime/45683/2

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
struct Container {
    theme: Theme,
}

#[derive(Debug, PartialEq, Default, Serialize)]
enum Theme {
    #[default]
    Default,
    Custom(String),
}

impl<'de> de::Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Theme, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeOptionVisitor;

        impl<'de> Visitor<'de> for ThemeOptionVisitor {
            type Value = Theme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`default` or <theme-file-path>")
            }

            fn visit_str<E>(self, value: &str) -> Result<Theme, E>
            where
                E: de::Error,
            {
                match value {
                    "default" => Ok(Theme::Default),
                    str => Ok(Theme::Custom(str.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ThemeOptionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let c = Container {
            theme: Theme::default(),
        };

        println!("{}", serde_json::to_string(&c).unwrap());
    }

    #[test]
    fn test_custom_deserialize_default() {
        let text = r#"{"theme": "default" }"#;
        assert_eq!(
            serde_json::from_str::<Container>(&text).unwrap(),
            Container {
                theme: Theme::Default
            }
        );
    }

    #[test]
    fn test_custom_deserialize_other() {
        let text = r#"{"theme": "other" }"#;
        assert_eq!(
            serde_json::from_str::<Container>(text).unwrap(),
            Container {
                theme: Theme::Custom("other".to_string())
            }
        );
    }
}
