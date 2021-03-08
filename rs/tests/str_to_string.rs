#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // more info: https://doc.rust-lang.org/nightly/book/ch08-02-strings.html
        let _ = "hello".to_owned(); // "I have a borrowed object and I want an owned version"
        let _ = "hello".to_string(); // "I want the textual representation of something"
        let _ = String::from("hello");
        let _ = format!("hello"); // expensive textual representation of something in a particular representation
        let _: String = "hello".into(); // require known result type : "I want a generic type conversion"
    }
}
