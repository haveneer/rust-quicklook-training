use validation_macros::{validated_regex, validated_sql};

#[test]
fn test_valid_regex() {
    let re = validated_regex!(r"^\d{3}-\d{4}$");
    assert!(re.is_match("123-4567"));
    assert!(!re.is_match("abc-defg"));
}

#[test]
fn test_valid_sql() {
    let sql = validated_sql!("SELECT id, name FROM users WHERE id = ?");
    assert_eq!(sql, "SELECT id, name FROM users WHERE id = ?");
}

// Ces tests ne compilent pas (commentés pour la démo) :
// #[test]
// fn test_invalid_regex() {
//     let re = validated_regex!(r"[invalid");  // ❌ Erreur de compilation
// }

// #[test]
// fn test_invalid_sql() {
//     let sql = validated_sql!("not a valid sql");  // ❌ Erreur de compilation
// }
