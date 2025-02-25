/// flexible_to_string! macro use a "autoref-based stable specialization" to specialize display for various types
///
/// The main specialization is to use Display as much as possible and to fallback on Debug

pub(crate) struct Match<'a, T>(pub(crate) &'a T);

// The number of & should be equal to the number of specialized cases + 1
#[macro_export]
macro_rules! flexible_to_string {
    ($e:expr) => {
        (&&&&$crate::Match(&$e)).flexible_to_string()
    };
}

// First case: LogDisplayToString (to manage all the custom display for logging)

pub(crate) trait LogDisplayToString {
    fn flexible_to_string(&self) -> String;
}

impl<'a, T: std::fmt::Display> LogDisplayToString for &&&Match<'a, Option<T>> {
    fn flexible_to_string(&self) -> String {
        match self.0 {
            None => "None".into(),
            Some(t) => format!("Some({t})"),
        }
    }
}

impl<'a> LogDisplayToString for &&&Match<'a, chrono::DateTime<chrono::Utc>> {
    fn flexible_to_string(&self) -> String {
        format!(
            "Timestamp({})",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
        )
    }
}

// Second case: all types what implement Display

pub(crate) trait DisplayToString {
    fn flexible_to_string(&self) -> String;
}

impl<'a, T: std::fmt::Display> DisplayToString for &&Match<'a, T> {
    fn flexible_to_string(&self) -> String {
        format!("{}", self.0)
    }
}

// Third case: all types what implement Display

pub(crate) trait DebugToString {
    fn flexible_to_string(&self) -> String;
}

impl<'a, T: std::fmt::Debug> DebugToString for &Match<'a, T> {
    fn flexible_to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

// Default case: any other type

// pub(super) trait AnyToString {
//     fn flexible_to_string(&self) -> String;
// }
//
// impl<'a, T> AnyToString for Match<'a, T> {
//     fn flexible_to_string(&self) -> String {
//         todo!() // to debug edge case
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flexible_to_string;
    use chrono::{DateTime, Utc};
    use std::str::FromStr;

    struct CustomTypeWithDebug;

    impl std::fmt::Debug for CustomTypeWithDebug {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Debug(CustomTypeWithDebug)")
        }
    }

    #[test]
    fn test_with_debug_only() {
        assert_eq!(
            flexible_to_string!(CustomTypeWithDebug),
            "Debug(CustomTypeWithDebug)"
        )
    }

    #[derive(Clone, Copy)]
    struct CustomTypeWithDisplay;

    impl std::fmt::Display for CustomTypeWithDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Display(CustomTypeWithDisplay)")
        }
    }

    #[test]
    fn test_with_display_only() {
        assert_eq!(
            flexible_to_string!(CustomTypeWithDisplay),
            "Display(CustomTypeWithDisplay)"
        )
    }

    struct CustomTypeWithDebugAndDisplay;

    impl std::fmt::Debug for CustomTypeWithDebugAndDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Debug(CustomTypeWithDebugAndDisplay)")
        }
    }

    impl std::fmt::Display for CustomTypeWithDebugAndDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Display(CustomTypeWithDebugAndDisplay)")
        }
    }

    #[test]
    fn test_with_debug_and_display() {
        assert_eq!(
            flexible_to_string!(CustomTypeWithDebugAndDisplay),
            "Display(CustomTypeWithDebugAndDisplay)"
        )
    }

    #[test]
    fn test_with_option_of_display() {
        assert_eq!(
            flexible_to_string!(Some(CustomTypeWithDisplay)),
            "Some(Display(CustomTypeWithDisplay))"
        )
    }

    #[test]
    fn test_with_block_number_or_timestamp_as_timestamp() {
        assert_eq!(
            flexible_to_string!(DateTime::<Utc>::from_str("2024-03-02T01:00:00Z").unwrap()),
            "Timestamp(2024-03-02T01:00:00Z)"
        )
    }
}
