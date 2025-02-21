#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
}

enum Case {
    A(u8, u8),
    B(u16),
    C([u8; 3]),
    D(bool),
    E,
}

mod details {
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
            "{context:8} size={:2} bytes={}",
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
    details::show_repr("Color", Color::Blue);
    details::show_repr("Case::A", Case::A(255, 254));
    details::show_repr("Case::B", Case::B(1));
    details::show_repr("Case::C", Case::C([1, 2, 3]));
    details::show_repr("Case::D", Case::D(true));
    details::show_repr("Case::E", Case::E);
}
