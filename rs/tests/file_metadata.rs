use std::fs;

#[test]
fn main() {
    let metadata = std::fs::metadata("Cargo.toml").unwrap();
    println!("is dir? {:?}", metadata.is_dir());
    println!("is file? {:?}", metadata.is_file());

    let entries = fs::read_dir(".").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let meta = fs::metadata(&path).unwrap();
        let prefix = if meta.is_dir() { "(dir)" } else { "     " };
        match path.to_str() {
            Some(s) => println!("{} {}", prefix, s),
            None => println!("{} {:?} (invalid utf-8)", prefix, path),
        }
    }
}
