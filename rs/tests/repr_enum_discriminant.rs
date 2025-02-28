#![allow(dead_code)]

enum Case {
    A(u8, u8),
    B([u8; 3]),
    C,
}

mod details {
    use super::Case;
    use rand::{thread_rng, Rng};
    use std::mem::Discriminant;

    impl Case {
        pub fn generator() -> Self {
            match thread_rng().gen_range(0..2) {
                0 => Case::A(1, 2),
                1 => Case::B([1, 2, 3]),
                2 => Case::C,
                _ => unreachable!(),
            }
        }

        pub const DISCRIMINANT_A: Discriminant<Case> = std::mem::discriminant(&Case::A(0, 0));
        pub const DISCRIMINANT_B: Discriminant<Case> = std::mem::discriminant(&Case::B([0, 0, 0]));
        pub const DISCRIMINANT_C: Discriminant<Case> = std::mem::discriminant(&Case::C);
    }

    #[test]
    fn test() {
        super::main();
    }
}

fn main() {
    let case = Case::generator();
    match std::mem::discriminant(&case) { // si un jour vous deviez introspecter un type inconnu...
        // complex matching since PartialEq is not implemented using a derive on Discriminant type
        d if d == Case::DISCRIMINANT_A => println!("discriminant A"),
        d if d == Case::DISCRIMINANT_B => println!("discriminant B"),
        d if d == Case::DISCRIMINANT_C => println!("discriminant C"),
        _ => panic!(),
    }
}
