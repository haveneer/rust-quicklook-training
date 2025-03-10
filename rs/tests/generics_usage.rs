fn foo<T>(arg: T) {}

fn main() {
    foo(1);
    foo(0.577);
    foo("Hello");
}

#[test]
fn test_foo() {
    main()
}

#[test]
#[rustfmt::skip]
fn generics_bad_usage() {
    let t = trybuild::TestCases::new();

    let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail(format!("tests/failures/{version_path}/generics_bad_usage.rs"));
    t.compile_fail(format!("tests/failures/{version_path}/generics_with_unsatisfied_bounds.rs"));
}
