fn main() {
    let mut v = [1, 2, 3];
    let first = &mut v[0];
    let last = &mut v[2];
    *first += *last;
}
