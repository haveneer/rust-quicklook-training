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
    let mut v: Vec<_> = (1..16).map(|_| rng.next() % 100).collect();

    let mid: usize = rng.next() % v.len();
    let (first_part, second_part) = v.split_at_mut(mid);

    std::thread::scope(|scope| {
        // Requires Rust 1.63+
        scope.spawn(|| {
            first_part.sort();
        });
        scope.spawn(|| {
            second_part.sort_by(|a, b| b.cmp(a));
        });
    });

    println!("v up and down at {mid}: {v:?}");
}

#[test]
fn test() {
    main()
}
