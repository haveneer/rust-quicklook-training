use memory_layout_derive::MemoryLayout;

#[derive(MemoryLayout, Debug, Default)]
struct Small {
    a: u8,
    b: u16,
    c: u32,
}

#[derive(MemoryLayout, Debug, Default)]
struct WithStrings {
    s1: String,
    s2: Option<String>,
    n: u64,
}

#[test]
fn test_print_layout_runs_small() {
    // Ensure the generated function exists and runs without panicking.
    Small::print_layout();
}

#[test]
fn test_print_layout_runs_with_strings() {
    // Exercise different field types to cover offset/size/type-name code paths
    WithStrings::print_layout();
}
