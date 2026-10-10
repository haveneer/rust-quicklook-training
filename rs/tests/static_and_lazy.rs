// Global data with a non-const initialization, in std since Rust 1.80 (LazyLock)
// and Rust 1.70 (OnceLock); previously provided by the lazy_static and once_cell crates
use std::collections::HashMap;
use std::sync::{LazyLock, OnceLock};
use std::thread;

struct ComplexStructure {
    field: String,
    all_want_you_want: HashMap<u32, String>,
}

impl ComplexStructure {
    pub fn new() -> Self {
        println!("initializing COMPLEX_STRUCTURE");
        Self {
            field: "Hello".to_owned(),
            all_want_you_want: (1..=4)
                .map(|k| (k, char::from(b'A' + k as u8 - 1).to_string()))
                .collect(),
        }
    }
}

// LazyLock: the initializer is given at the declaration, run on first access
static COMPLEX_STRUCTURE: LazyLock<ComplexStructure> = LazyLock::new(ComplexStructure::new);

// OnceLock: the value is chosen at runtime, by whoever comes first
static CONFIG: OnceLock<String> = OnceLock::new();

fn main() {
    println!("{}", COMPLEX_STRUCTURE.field); // initialization happens here
    let mut entries: Vec<_> = COMPLEX_STRUCTURE.all_want_you_want.iter().collect();
    entries.sort();
    entries.iter().for_each(|(k, v)| println!("{k} : {v}"));

    thread::scope(|s| {
        for id in 0..4 {
            s.spawn(move || {
                // Only one closure runs; the other threads wait, then read the value
                let config = CONFIG.get_or_init(|| format!("set by thread {id}"));
                println!("thread {id} reads: {config}");
            });
        }
    });
    assert!(CONFIG.set("too late".into()).is_err()); // already initialized
}

#[test]
fn test() {
    main()
}
