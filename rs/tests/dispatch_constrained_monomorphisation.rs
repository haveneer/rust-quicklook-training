// Exact equivalent of dispatch_generic_function
//      if #[inline(never)] is not enabled on internal_common_function
// Max number of vowels in an alphabetic word
fn generic_function<T: AsRef<str>>(t: T) -> usize {
    t.as_ref()
        .split_whitespace()
        .filter(|w| w.chars().all(char::is_alphabetic))
        .map(|w| w.chars().filter(|c| "aeiouy".contains(*c)).count())
        .max()
        .unwrap_or(0)
}

fn dispatch_generic_function<T: AsRef<str>>(t: T) -> usize {
    internal_common_function(t.as_ref())
}

#[inline(never)]
fn internal_common_function(s: &str) -> usize {
    s.split_whitespace()
        .filter(|w| w.chars().all(char::is_alphabetic))
        .map(|w| w.chars().filter(|c| "aeiouy".contains(*c)).count())
        .max()
        .unwrap_or(0)
}

fn main() {
    use std::{borrow::Cow, hint::black_box, rc::Rc};
    let s = "hello wonderful 🌍 world";

    // Full monomorphisation: the whole iterator chain is copied for each T
    black_box(generic_function(s));
    black_box(generic_function(String::from(s)));
    black_box(generic_function(Rc::<str>::from(s)));
    black_box(generic_function(Box::<str>::from(s)));
    black_box(generic_function(Cow::from(s)));

    // Partial monomorphisation: only the wrapper is copied for each T
    black_box(dispatch_generic_function(s));
    black_box(dispatch_generic_function(String::from(s)));
    black_box(dispatch_generic_function(Rc::<str>::from(s)));
    black_box(dispatch_generic_function(Box::<str>::from(s)));
    black_box(dispatch_generic_function(Cow::from(s)));
}

#[test]
fn test() {
    main()
}
