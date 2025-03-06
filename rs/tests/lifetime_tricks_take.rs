use std::collections::HashMap;

struct MyRng {
    state: usize,
}

impl MyRng {
    fn new() -> Self {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        // On mélange un peu la valeur de l'heure pour éviter qu'elle ne soit trop prévisible
        let seed = now.as_nanos() as usize ^ 0x5D588B65;
        Self { state: seed }
    }

    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

fn main() {
    let mut rng = MyRng::new();
    let mut map: HashMap<&str, Vec<_>> = HashMap::new();
    map.insert("a", (1..16).map(|_| rng.next() % 100).collect());
    map.insert("b", (1..16).map(|_| rng.next() % 100).collect());

    let val_a_addr = map.get("a").unwrap().as_ptr();
    let mut val_a = std::mem::take(map.get_mut("a").unwrap()); // move (no alloc, no copy)
    let val_b_addr = map.get("b").unwrap().as_ptr();
    let mut val_b = std::mem::take(map.get_mut("b").unwrap()); // move (no alloc, no copy)

    std::thread::scope(|scope| {
        // Requires Rust 1.63+
        scope.spawn(|| {
            val_a.sort();
        });
        scope.spawn(|| {
            val_b.sort_by(|a, b| b.cmp(a));
        });
    });

    let _ = std::mem::replace(map.get_mut("a").unwrap(), val_a); // move back (no alloc, no copy)
    let _ = std::mem::replace(map.get_mut("b").unwrap(), val_b); // move back (no alloc, no copy)

    assert_eq!(map.get("a").unwrap().as_ptr(), val_a_addr);
    println!("map[a]: {:?}", map.get("a").unwrap());
    assert_eq!(map.get("b").unwrap().as_ptr(), val_b_addr);
    println!("map[b]: {:?}", map.get("b").unwrap());
}

#[test]
fn test() {
    main()
}
