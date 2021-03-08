mod tests {
    use std::fmt;
    use std::fmt::Formatter;
    use std::ops::Add;

    const SIZE: usize = 8; // for generic size, see tiny_vector_generics.rs

    // Add Clone/Copy
    // #[derive(Clone, Copy)] // Generated case (done by hand below)
    struct TinyVector {
        // Only 'Copy' field
        name: [u8; 64], // how to not repeat this constant ?
        name_size: usize,
        data: [f64; SIZE],
    }

    impl TinyVector {
        const MAX_NAME_SIZE: usize = 64;

        pub fn new(str_name: String, data: [f64; SIZE]) -> Self {
            println!("New {}", str_name);
            let (name, name_size) = Self::string_to_bytes(str_name);
            Self {
                name,
                name_size,
                data,
            }
        }

        pub fn constant(name: &str, v: f64) -> Self {
            Self::new(name.into(), [v; SIZE])
        }

        fn string_to_bytes(str: String) -> ([u8; Self::MAX_NAME_SIZE], usize) {
            let u8_name = str.as_bytes();
            let u8_name_size = u8_name.len();
            if u8_name_size > Self::MAX_NAME_SIZE {
                panic!("Too long name")
            }
            let mut name = [0u8; Self::MAX_NAME_SIZE];
            name[..u8_name_size].copy_from_slice(u8_name);
            (name, u8_name_size)
        }

        fn name(&self) -> String {
            match std::str::from_utf8(&self.name[..self.name_size]) {
                Ok(str) => str.into(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
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
            write!(f, "[{}]", z)?;
            Ok(())
        }
    }

    impl Clone for TinyVector {
        // never called since embedded implements Copy
        fn clone(&self) -> Self {
            panic!("Call of clone() on a TinnyVector")
        }
    }

    impl Copy for TinyVector {}

    // impl Add for TinyVector {
    //     type Output = TinyVector;
    //
    //     fn add(self, rhs: Self) -> Self::Output {
    //         let mut data = self.data; // no clone: data moved
    //         for i in 0..SIZE {
    //             data[i] += rhs.data[i];
    //         }
    //         Self::new(format!("({} + {})", self.name(), rhs.name()), data)
    //     }
    // }

    // macro to implement all combinaison ref|move
    auto_ops::impl_op_ex!(+ |a: &TinyVector, b: &TinyVector| -> TinyVector {
        let mut data = a.data; // no clone: data moved
        for i in 0..SIZE {
            data[i] += b.data[i];
        }
        TinyVector::new(format!("({} + {})", a.name(), b.name()), data)
    });

    #[test]
    fn test_copy_add() {
        let v0 = TinyVector::constant("v0", 1.);
        let v1 = v0; // Copy, no move
        let v2 = TinyVector::constant("v2", 2.);

        let v3 = v0 + v2; // v0 still available due to allowed Copy (by trait)
        let v3 = v1 + v2;
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
        println!("v3 = {}", v3);
        let v4 = v3 + v1; // OK
    }
}
