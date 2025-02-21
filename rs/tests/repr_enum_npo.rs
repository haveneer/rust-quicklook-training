#![allow(dead_code)]

struct OptionOf<X> {
    v: Option<X>,
}

mod details {
    use crate::OptionOf;

    impl<X> OptionOf<X> {
        pub fn new(x: X) -> Self {
            Self { v: Some(x) }
        }
    }

    fn to_bytes<T>(t: &T) -> &[u8] {
        let p: *const T = t;
        let p = p as *const u8;
        unsafe { std::slice::from_raw_parts(p, size_of::<T>()) }
    }

    fn to_str_bytes<T>(t: &T) -> String {
        itertools::join(to_bytes(t).into_iter().map(|b| format!("{:02x}", b)), "")
    }

    pub fn show_repr<T>(context: &str, t: T) {
        println!(
            "{context:22} size={:2} bytes={}",
            size_of_val(&t),
            to_str_bytes(&t)
        );
    }

    #[test]
    fn test() {
        super::main();
    }
}

fn main() {
    use std::num::NonZeroI32;
    let x = 1;

    details::show_repr("OptionOf<i32>", OptionOf::new(255));
    details::show_repr("OptionOf<Box<i32>>", OptionOf::new(Box::new(255)));
    details::show_repr("OptionOf<&i32>", OptionOf::new(&x));
    details::show_repr("OptionOf<bool>", OptionOf::new(true));
    details::show_repr(
        "OptionOf<NonZeroI32>",
        OptionOf::new(NonZeroI32::new(255).unwrap()),
    );
}
