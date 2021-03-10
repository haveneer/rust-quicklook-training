struct TinyVector<const SIZE: usize> {
    data: [f64; SIZE],
}

impl<const SIZE: usize> TinyVector<SIZE> {
    pub fn zero() -> Self {
        Self { data: [0f64; SIZE] }
    }
    pub fn constant(v: f64) -> Self {
        Self { data: [v; SIZE] }
    }
}

#[test]
fn test() {
    let v1 = TinyVector::<4>::zero();
    let v2 = TinyVector::<4>::constant(1.);
}
