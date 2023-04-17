#![allow(unused_variables)]

fn main() {
    let b;
    {
        let a = 1;
        b = &a;
    }
    let c = *b;
}
