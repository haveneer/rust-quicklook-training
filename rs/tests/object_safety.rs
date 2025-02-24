
#[test]
#[rustfmt::skip]
fn object_safety_failures() {
    let t = trybuild::TestCases::new();

    let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail(format!("tests/failures/{version_path}/object_safety.rs"));
}
