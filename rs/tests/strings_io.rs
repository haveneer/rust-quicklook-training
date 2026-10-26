use std::fmt::Write as _;
use std::io::BufRead;

fn main() {
    let log = "GET /\nget /admin\nPOST /login\n";
    let mut reader = log.as_bytes(); // &[u8] is a BufRead

    // One buffer for all lines (lines(): one String each)
    let mut line = String::new();
    let mut report = String::with_capacity(64);
    let mut n = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        let (verb, path) = line.split_once(' ').unwrap();
        let path = path.trim_end(); // drop the '\n'

        // In place, no String from to_lowercase()
        if verb.eq_ignore_ascii_case("get") {
            n += 1;
            // Appends to report; format! would allocate
            writeln!(report, "{n:>3} {path}").unwrap();
        }
        line.clear(); // read_line appends: empty it first
    }
    assert_eq!(report, "  1 /\n  2 /admin\n");
}

#[test]
fn test() {
    main()
}
