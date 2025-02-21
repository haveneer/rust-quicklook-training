#![allow(dead_code)]

struct RustVertex {
    x: i32,
    y: i64,
    z: i8,
}

mod details {
    #[repr(C)]
    pub struct CVertex {
        x: i32,
        y: i64,
        z: i8,
    }

    fn to_bytes<T>(t: &T) -> &[u8] {
        let p: *const T = t;
        let p = p as *const u8;
        unsafe { std::slice::from_raw_parts(p, size_of::<T>()) }
    }

    fn to_str_bytes<T>(t: &T) -> String {
        itertools::join(to_bytes(t).into_iter().map(|b| format!("{:02x}", b)), "")
    }

    pub fn demo() {
        use crate::RustVertex;
        let v1 = RustVertex { x: 1, y: 2, z: 3 };
        let v2 = CVertex { x: 1, y: 2, z: 3 };
        println!("Rust  size={:2} bytes={}", size_of_val(&v1), to_str_bytes(&v1));
        println!("C/C++ size={:2} bytes={}", size_of_val(&v2), to_str_bytes(&v2));
    }

    #[test] // we cannot use #[test] on demo to make available in rust playground
    fn test() {
        demo();
    }
}

fn main() {
    details::demo();
}
