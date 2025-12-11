use syn_quote_examples::*;

#[test]
fn test_show_tokens() {
    // show_tokens affiche les tokens lors de la compilation
    show_tokens!(1 + 2);
}

// Test parse_example
#[derive(ParseExample)]
struct TestParse {
    field1: i32,
}

// Test inspect_fields
#[derive(InspectFields)]
struct TestInspect {
    name: String,
    age: u32,
}

// Test auto_display
#[derive(AutoDisplay)]
struct TestDisplay {
    value: i32,
}

#[test]
fn test_auto_display() {
    let t = TestDisplay { value: 42 };
    assert_eq!(format!("{}", t), "TestDisplay");
}

// Test syn_example
#[derive(SynExample)]
struct TestSyn {
    field_a: String,
    field_b: i32,
}

// Test auto_new
#[derive(AutoNew)]
struct User {
    name: String,
    age: u32,
    #[default] // with a custom attribute
    is_admin: bool,
}

#[test]
fn test_auto_new() {
    let user = User::new("Alice".to_string(), 30);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
    assert!(!user.is_admin);
}
