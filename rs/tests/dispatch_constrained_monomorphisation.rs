// Exact equivalent of dispatch_generic_function
//      if #[inline(never)] is not enabled on internal_common_function
fn generic_function<T: AsRef<str>>(t: T) -> usize {
    t.as_ref().chars().fold(0, |acc, c| acc + c.len_utf8())
}

fn dispatch_generic_function<T: AsRef<str>>(t: T) -> usize {
    internal_common_function(t.as_ref())
}

#[inline(never)]
fn internal_common_function(s: &str) -> usize {
    s.chars().fold(0, |acc, c| acc + c.len_utf8())
}

fn main() {
    // Full monomorphisation (at each call)
    std::hint::black_box(generic_function(String::from("hello 🌍!")));
    std::hint::black_box(generic_function("hello 🌍!"));

    // Partial monomorphisation (only for top level function, not on common part)
    std::hint::black_box(dispatch_generic_function(String::from("hello 🌍!")));
    std::hint::black_box(dispatch_generic_function("hello 🌍!"));
}

#[test]
fn test() {
    main()
}
