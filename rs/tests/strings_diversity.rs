use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn main() {
    let name = "données.log";

    // &str -> the others: free views (Path, OsStr), or a checked copy (CString)
    let path: &Path = Path::new(name);
    let os: &OsStr = path.as_os_str();
    let c: CString = CString::new(name).expect("no NUL byte inside");
    assert!(CString::new("a\0b").is_err()); // a C string cannot hold a NUL
    let literal: &CStr = c"littéral C";

    // ... and back: it may fail, or lose information
    assert_eq!(os.to_str(), Some(name)); // Option: OS strings may not be UTF-8
    let shown: Cow<str> = path.to_string_lossy(); // U+FFFD if needed
    assert_eq!(c.to_str(), Ok(name)); // Result: C bytes may not be UTF-8
    println!("{} {shown} {literal:?}", path.display());

    // Owned <-> borrowed, within a family
    let owned: PathBuf = path.with_extension("gz"); // allocates
    let view: &Path = &owned; // Deref, free
    let text: String = name.to_owned(); // &str -> String, allocates
    assert_eq!(view, Path::new("données.gz"));

    // Immutable and shared: no capacity, one allocation
    let shared: Arc<str> = Arc::from(text);
    let boxed: Box<str> = "figé".into();
    println!("{shared} {boxed}");
}

#[test]
fn test() {
    main()
}
