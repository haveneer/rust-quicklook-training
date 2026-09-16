use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

// #[derive(Clone, Copy)] // Generated case (done by hand below)
struct TinyVector {
    data: [f64; Self::SIZE],
}

impl TinyVector {
    const SIZE: usize = 8;

    pub fn constant(v: f64) -> Self {
        Self {
            data: [v; Self::SIZE],
        }
    }
}

impl fmt::Display for TinyVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let z = self
            .data
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", z)
    }
}

impl Clone for TinyVector {
    // Requis uniquement pour la cohérence de l'API (Copy: Clone), pas pour la copie
    // elle-même: le compilateur ne l'appelle jamais pour dupliquer une valeur Copy,
    // il copie directement bit-à-bit. panic! ci-dessous est donc jamais atteint.
    #[allow(clippy::non_canonical_clone_impl)] // panic! volontaire, à but pédagogique
    fn clone(&self) -> Self {
        panic!("Call of clone() on a TinyVector")
    }
}

impl Copy for TinyVector {}

impl Add for TinyVector {
    type Output = TinyVector;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data; // Copy: self n'est pas déplacé, juste bit-copié
        for (d, r) in data.iter_mut().zip(rhs.data) {
            *d += r;
        }
        Self { data }
    }
}

fn main() {
    let v1 = TinyVector::constant(1.);
    let v2 = TinyVector::constant(2.);

    let v3 = v1 + v2; // v1 reste disponible après: Copy, pas de move
    let v4 = v1 + v3; // donc réutilisable autant de fois que voulu
    println!("v3 = {}", v3);
    println!("v4 = {}", v4);
}

#[test]
fn test_copy_add() {
    main();
}
