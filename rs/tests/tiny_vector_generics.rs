struct TinyVector<const size: usize> {
    data: [f64; size],
}

impl<const size: usize> TinyVector<size> {
    pub fn zero() -> Self {
        Self { data: [0f64; size] }
    }
    pub fn constant(v: f64) -> Self {
        Self { data: [v; size] }
    }
}

#[test]
fn test() {
    let v1 = TinyVector::<4>::zero();
    let v2 = TinyVector::<4>::constant(1.);
}
