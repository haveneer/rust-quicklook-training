fn main() {
    // One visible "é", two encodings (NFC and NFD)
    let nfc = "caf\u{E9}"; // é precomposed
    let nfd = "cafe\u{301}"; // e + combining acute accent
    assert_ne!(nfc, nfd); // == compares bytes
    assert_eq!((nfc.len(), nfc.chars().count()), (5, 4));
    assert_eq!((nfd.len(), nfd.chars().count()), (6, 5));
    // Reversed chars: the accent now leads, on nothing
    let rev: String = nfd.chars().rev().collect();
    assert!(rev.starts_with('\u{301}'));
    // One grapheme: three emojis joined by ZWJ (U+200D)
    let family = "👨\u{200D}👩\u{200D}👧";
    assert_eq!((family.len(), family.chars().count()), (18, 5));

    // Case mapping is not one char to one char
    assert_eq!("straße".to_uppercase(), "STRASSE");
    // char::to_uppercase is an iterator: 'ß' gives 2 chars
    assert_eq!('ß'.to_uppercase().count(), 2);
    // Locale-independent conversion (Turkish would want "ı")
    assert_eq!("I".to_lowercase(), "i");
    // *_ascii_* functions ignore non-ASCII letters
    assert!("ÉTÉ".eq_ignore_ascii_case("ÉtÉ"));
    assert!(!"ÉTÉ".eq_ignore_ascii_case("été"));

    // Order = code points, not a dictionary
    let mut words = vec!["zèbre", "Ève", "arbre"];
    words.sort();
    assert_eq!(words, ["arbre", "zèbre", "Ève"]);

    // Unicode predicates are wider than ASCII ones
    assert!('٣'.is_numeric()); // ARABIC-INDIC DIGIT THREE
    assert!(!'٣'.is_ascii_digit());
    assert!("٣".parse::<u32>().is_err());
}

#[test]
fn test() {
    main()
}
