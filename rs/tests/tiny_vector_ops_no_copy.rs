use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};

// No Clone/Copy by default
struct TinyVector {
    name: String,
    data: [f64; Self::SIZE], // for generic size, see tiny_vector_generics.rs
}

// Copy is just a marker (nothing to implement); it means memcpy aware (i.e. raw data compatible)
// TinyVector Cannot be marked as Copy
// impl Copy for TinyVector {} // and String is not 'Copy'
impl TinyVector {
    const SIZE: usize = 8;

    pub fn new(name: String, data: [f64; Self::SIZE]) -> Self {
        println!("New {}", name);
        Self { name, data }
    }

    pub fn constant(name: &str, v: f64) -> Self {
        Self::new(name.into(), [v; Self::SIZE])
    }
}

impl Drop for TinyVector {
    // Follow dropped objects
    fn drop(&mut self) {
        println!("Drop {}", self.name);
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
        write!(f, "{}: [{}]", self.name, z)?;
        Ok(())
    }
}

impl Add for TinyVector {
    type Output = TinyVector;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data; // [f64;8] implements copy
        for i in 0..Self::SIZE {
            data[i] += rhs.data[i];
        }
        Self::new(format!("({} + {})", self.name, rhs.name), data)
    }
}

impl<'a, 'b> Sub<&'b TinyVector> for &'a TinyVector {
    type Output = TinyVector;

    fn sub(self, rhs: &'b TinyVector) -> Self::Output {
        // self is already a ref
        let mut data = self.data.clone();
        for i in 0..TinyVector::SIZE {
            data[i] -= rhs.data[i];
        }
        Self::Output::new(format!("({} - {})", self.name, rhs.name), data)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_and_drop() {
        let v = TinyVector::constant("v", 1.);
        std::mem::drop(v);
        println!("End of test");
    }

    #[test]
    fn test_moved_add() {
        let v0 = TinyVector::constant("v0", 1.);
        let v1 = v0;
        let v2 = TinyVector::constant("v2", 2.);

        // let v3 = v0 + v2; // error: v0 used after moved
        let v3 = v1 + v2; // move v1 and v2
        // println!("v1 = {}", v1); // error: v1 used after moved
        // println!("v2 = {}", v2); // error: v2 used after moved
        println!("v3 = {}", v3);
        // let v4 = v3 + v1; // error: use already moved value: `v1`
    }

    #[test]
    fn test_borrow_sub() {
        let v1 = TinyVector::constant("v1", 1.);
        let v2 = TinyVector::constant("v2", 2.);

        let v3 = &v1 - &v2;
        println!("v3 = {}", v3);
        let v4 = &v3 - &v1; // OK no moved data
        println!("v4 = {}", v4);
    }
}
