use hashbrown::HashMap;
use rand::{thread_rng, Rng};

fn main() {
    let mut map: HashMap<&str, Vec<_>> = HashMap::new();
    map.insert("a", vec![0u8; 10]);
    map.insert("b", vec![0u8; 10]);

    if let [Some(val_a), Some(val_b)] = map.get_many_mut(["a", "b"]) {
        std::thread::scope(|scope| {
            // Requires Rust 1.63+
            scope.spawn(|| {
                thread_rng().fill(val_a.as_mut_slice());
                val_a.sort();
            });
            scope.spawn(|| {
                thread_rng().fill(val_b.as_mut_slice());
                val_b.sort_by(|a, b| b.cmp(a));
            });
        });
    }
    println!("map[a]: {:?}", map.get("a").unwrap());
    println!("map[b]: {:?}", map.get("b").unwrap());
}

#[test]
fn test() {
    main()
}
