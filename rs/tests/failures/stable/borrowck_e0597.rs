fn main() {
    let best;
    {
        let name = String::from("naca0012");
        best = &name;
    }
    println!("{best}");
}
