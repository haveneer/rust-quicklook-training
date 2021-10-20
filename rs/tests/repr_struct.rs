struct Vertex1 {
    x: i32,
    y: i64,
    z: i8,
}

#[repr(C)]
struct Vertex2 {
    x: i32,
    y: i64,
    z: i8,
}

fn to_bytes<T>(t: &T) -> &[u8] {
    let p: *const T = t;
    let p = p as *const u8;
    unsafe {
        std::slice::from_raw_parts(p, std::mem::size_of::<T>())
    }
}

fn main() {
    let v1 = Vertex1 { x: 1, y: 2, z: 3 };
    let v2 = Vertex2 { x: 1, y: 2, z: 3 };
    println!("Rust  {} {:?}", std::mem::size_of_val(&v1), to_bytes(&v1));
    println!("C/C++ {} {:?}", std::mem::size_of_val(&v2), to_bytes(&v2));
}

#[test]
fn test() { main() }