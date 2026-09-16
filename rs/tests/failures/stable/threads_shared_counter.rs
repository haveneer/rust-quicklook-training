fn main() {
    let mut counter = 0;
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..1_000_000 {
                counter += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..1_000_000 {
                counter += 1;
            }
        });
    });
    println!("counter = {counter}");
}
