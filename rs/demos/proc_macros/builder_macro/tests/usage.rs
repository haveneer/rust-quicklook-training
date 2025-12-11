use builder_macro::Builder;

#[derive(Builder, Debug)]
struct User {
    name: String,
    age: u32,
    #[builder(default)]
    email: String,
}

#[test]
fn test_builder_complete() {
    let user = User::builder()
        .name("Alice".to_string())
        .age(30)
        .email("alice@example.com".to_string())
        .build()
        .unwrap();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
    assert_eq!(user.email, "alice@example.com");
}

#[test]
fn test_builder_optional_field() {
    let user = User::builder()
        .name("Bob".to_string())
        .age(25)
        .build()
        .unwrap();

    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, 25);
    assert!(user.email.is_empty());
}

#[test]
fn test_builder_missing_field() {
    let result = User::builder().name("Charlie".to_string()).build();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("age is missing"));
}
