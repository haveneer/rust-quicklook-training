// "  port = 8080  # default" -> ("port", 8080); every &str borrows `line`
fn parse_setting(line: &str) -> Option<(&str, u16)> {
    let code = line.split_once('#').map_or(line, |(code, _comment)| code);
    let (key, value) = code.split_once('=')?;
    Some((key.trim(), value.trim().parse().ok()?))
}

fn main() {
    assert_eq!(
        parse_setting("  port = 8080  # default"),
        Some(("port", 8080))
    );
    assert_eq!(parse_setting("# nothing here"), None);
    assert_eq!(parse_setting("port = eighty"), None); // parse() failed

    // strip_prefix / strip_suffix: an Option, rather than slicing by hand
    assert_eq!("v1.2.3".strip_prefix('v'), Some("1.2.3"));
    assert_eq!("report.csv".strip_suffix(".csv"), Some("report"));

    // rsplit_once cuts at the *last* separator
    assert_eq!("a/b/c.txt".rsplit_once('/'), Some(("a/b", "c.txt")));

    // split(' ') keeps the empty fields, split_whitespace drops them
    let row = "a  b   c";
    assert_eq!(row.split(' ').count(), 6);
    assert_eq!(row.split_whitespace().collect::<Vec<_>>(), ["a", "b", "c"]);

    // Searching gives byte indices
    assert_eq!("key=value".find('='), Some(3));
    assert_eq!("l1\nl2\r\nl3".lines().count(), 3); // \n or \r\n
}

#[test]
fn test() {
    main()
}
