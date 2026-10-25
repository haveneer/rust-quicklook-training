use std::io::Read;
use std::str;

fn main() {
    let mut raw = "café ".as_bytes().to_vec();
    raw.push(0xFF); // 0xFF is never valid UTF-8
    let bytes: &[u8] = &raw;

    // from_utf8 checks once, then gives a &str (no copy)
    let err = str::from_utf8(bytes).unwrap_err();
    assert_eq!(err.valid_up_to(), 6); // "café " is valid
    assert_eq!(str::from_utf8(&bytes[..6]), Ok("café "));

    // from_utf8_lossy: Cow, borrowed if valid, owned with U+FFFD otherwise
    assert_eq!(String::from_utf8_lossy(bytes), "café \u{FFFD}");

    // read_to_string fails on invalid UTF-8 (a file behaves the same way)
    let mut text = String::new();
    let mut reader = bytes; // &[u8] implements Read, like a File
    let e = reader.read_to_string(&mut text).unwrap_err();
    assert_eq!(e.kind(), std::io::ErrorKind::InvalidData);

    // Byte indices, not characters
    let s = "café";
    assert_eq!((s.len(), s.chars().count()), (5, 4));
    assert!(!s.is_char_boundary(4)); // inside 'é'
    assert_eq!(s.get(0..4), None); // &s[0..4] would panic
    assert_eq!(s.char_indices().last(), Some((3, 'é')));
}

#[test]
fn test() {
    main()
}
