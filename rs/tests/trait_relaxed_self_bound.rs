use std::fmt::Debug;

// Définition d'un trait que nous allons implémenter
trait Printable {
    fn print(&self);
}

// Implémentation générique du trait pour tout type qui implémente `Debug`
impl<T: ?Sized + Debug> Printable for T {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let x = 42;
    let s: &str = "Hello, Rust!";
    let dyn_debug: &dyn Debug = &x;

    x.print(); // Fonctionne pour un type Sized
    s.print(); // Fonctionne pour str (qui est !Sized)
    dyn_debug.print(); // Fonctionne pour un trait objet (dyn Debug)
}

#[test]
fn test() {
    main()
}
