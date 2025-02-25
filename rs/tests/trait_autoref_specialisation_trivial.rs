struct Value;

impl Value {
    fn print(&self) {
        println!("it works!");
    }
}

fn main() {
    let v = Value;
    v.print();
}

#[cfg(test)]
fn test() {
    main()
}
