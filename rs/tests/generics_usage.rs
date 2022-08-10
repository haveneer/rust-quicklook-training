fn foo<T>(arg: T) {}

fn main() {
    foo(1);
    foo(3.14);
    foo("Hello");
}

#[test]
fn test_foo() {
    main()
}

#[test]
fn generics_bad_usage() {
    let t = trybuild::TestCases::new();

    let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail(format!("tests/failures/{version_path}/generics_bad_usage.rs"));
    t.compile_fail(format!("tests/failures/{version_path}/generics_with_unsatisfied_bounds.rs"));
}
