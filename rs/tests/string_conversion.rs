fn is_strong_string(password: String) -> bool {
    // only for String
    password.len() > 6
}

fn is_strong_str(password: &str) -> bool {
    // only for str
    password.len() > 6
}

fn is_strong_asref<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 6 // as_ref returns a ref (&str)
}

fn is_strong_into<T: Into<String>>(password: T) -> bool {
    password.into().len() > 6 // into returns a new object (String; no ref)
}

#[test]
fn test() {
    assert!(is_strong_string(String::from("password")));
    // assert!(is_strong_string("password")); // error:  expected struct `String`, found `&str`
    // assert!(is_strong_str(String::from("password"))); // error:  expected `&str`, found struct `String`
    assert!(is_strong_str("password"));
    assert!(is_strong_asref(String::from("password")));
    assert!(is_strong_asref("password"));
    assert!(is_strong_into(String::from("password")));
    assert!(is_strong_into("password"));
}
