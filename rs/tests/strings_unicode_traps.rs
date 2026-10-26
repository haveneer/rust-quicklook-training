fn suspicious(c: char) -> bool {
    // Bidi controls (Trojan Source), zero-width chars
    matches!(c, '\u{202A}'..='\u{202E}'
              | '\u{2066}'..='\u{2069}'
              | '\u{200B}'..='\u{200D}')
}

fn main() {
    // Homoglyphs: same look, different code points
    let user = "\u{430}dmin"; // Cyrillic 'а', not Latin 'a'
    assert_ne!(user, "admin");
    let odd = user.chars().find(|c| !c.is_ascii()).unwrap();
    assert_eq!(odd.escape_unicode().to_string(), "\\u{430}");

    // Bidi override: shown as "invoice_exe.pdf", is a .exe
    let file = "invoice_\u{202E}fdp.exe";
    assert!(file.ends_with(".exe"));
    assert!(file.chars().any(suspicious));

    // Width: {:<4} pads by chars, but a CJK char takes
    // 2 columns: "日本  |" is 6 columns wide, not 4
    assert_eq!(format!("{:<4}|", "日本"), "日本  |");

    // UTF-16 (Windows, JS): 😀 needs a surrogate pair
    assert_eq!("é😀".encode_utf16().count(), 3);
    // A lone surrogate (half of a pair) is not text
    assert!(String::from_utf16(&[0xD800]).is_err());
    assert_eq!(String::from_utf16_lossy(&[0xD800]), "\u{FFFD}");
}

#[test]
fn test() {
    main()
}
