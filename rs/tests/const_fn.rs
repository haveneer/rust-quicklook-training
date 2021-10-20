use rand::prelude::*;

const fn f(x: i64) -> i64 {
    (x << 2) ^ (x >> 2)
}

const VALUE: i64 = f(128);

#[test]
fn main() {
    let mut rng = rand::thread_rng();
    let x = rng.gen::<i64>();
    f(x);
}