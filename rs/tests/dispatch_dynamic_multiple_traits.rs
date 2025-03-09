trait Trait1 {
    fn f1(&self);
}

trait Trait2 {
    fn f2(&self);
}

trait Traits: Trait1 + Trait2 {}
impl<T: Trait1 + Trait2> Traits for T {}

struct A;
impl Trait1 for A {
    fn f1(&self) {}
}

impl Trait2 for A {
    fn f2(&self) {}
}

#[test]
fn main() {
    // let valeurs: Vec<Box<dyn Trait1 + Trait2>> = vec![Box::new(A {})]; // Not valid if not markers
    let valeurs: Vec<Box<dyn Traits>> = vec![Box::new(A {})];

    for valeur in valeurs {
        valeur.f1();
        valeur.f2();
    }
}
