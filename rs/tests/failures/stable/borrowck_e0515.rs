fn prefix(text: &str) -> &str {
    let upper = text.to_uppercase();
    &upper[..3]
}

fn main() {
    println!("{}", prefix("naca0012"));
}
