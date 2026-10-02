fn consume(_v: Vec<i32>) {}

fn main() {
    let v = vec![1, 2, 3];
    let r = &v;
    consume(v);
    println!("{r:?}");
}
