#[test]
fn box_basics() {
    // Box<T> alloue T sur le tas ; la Box elle-même (sur la pile) n'est qu'un pointeur.
    let boxed: Box<i32> = Box::new(42);
    assert_eq!(*boxed, 42);

    // Taille fixe et petite quelle que soit la taille de T (juste un pointeur)
    assert_eq!(
        std::mem::size_of::<Box<[i32; 1000]>>(),
        std::mem::size_of::<usize>()
    );

    // Déplacer une Box est bon marché : seul le pointeur est copié, pas les données
    let moved = boxed;
    assert_eq!(*moved, 42);
}

#[test]
fn box_recursive_type() {
    // Box<T> permet de définir des types récursifs : sans lui, la taille de List serait infinie.
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    fn sum(list: &List) -> i32 {
        match list {
            Cons(value, rest) => value + sum(rest),
            Nil => 0,
        }
    }

    assert_eq!(sum(&list), 6);
}

#[test]
fn box_dyn_trait() {
    // Box<dyn Trait> : pointeur possédé vers une valeur de type inconnu à la compilation
    // (dispatch dynamique), utile pour stocker des types hétérogènes dans un même Vec.
    trait Shape {
        fn area(&self) -> f64;
    }
    struct Square(f64);
    struct Circle(f64);
    impl Shape for Square {
        fn area(&self) -> f64 {
            self.0 * self.0
        }
    }
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.0 * self.0
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square(2.0)), Box::new(Circle(1.0))];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    assert!((total - (4.0 + std::f64::consts::PI)).abs() < 1e-9);
}
