#![allow(dead_code)]
// C++ code to compare
// #include <cstdint>
// #include <iostream>
//
// struct Empty { };
//
// struct MyStruct {
//     int8_t byte;
//     // [[no_unique_address]]
//     Empty empty;
// };
//
// int main() {
//     MyStruct s;
//     std::cout << sizeof(s) << std::endl;
// }

// #[derive(Debug)] // not possible on union
#[repr(C)]
union MyCUnion {
    f1: u16,
    f2: [u8; 4],
}

#[repr(C)] // same as default
struct MyEmptyStruct {}

#[repr(C)] // same as default
struct MyStructWithAnEmptyField {
    byte: u8,
    empty: MyEmptyStruct,
}

enum MyEnum {
    A(u8, u8),
    B(u16),
    C([u8; 3]),
    D(bool),
    E,
}

#[repr(C)]
enum MyCEnum {
    A(u8, u8),
    B(u16),
    C([u8; 3]),
    D(bool),
    E,
}

mod details {
    pub fn to_bytes<T>(t: &T) -> &[u8] {
        let p: *const T = t;
        let p = p as *const u8;
        unsafe { std::slice::from_raw_parts(p, size_of::<T>()) }
    }

    pub fn to_str_bytes<T>(t: &T) -> String {
        itertools::join(to_bytes(t).into_iter().map(|b| format!("{:02x}", b)), "")
    }
}

#[rustfmt::skip]
fn main() {
    let union = MyCUnion { f1: 256 };
    println!("MyCUnion {{ f1: 256 }}          : {}", details::to_str_bytes(&union));
    let union = MyCUnion { f2: [0, 1, 0, 0] }; // cf endianness
    println!("MyCUnion {{ f2: [0, 1, 0, 0] }} : {}", details::to_str_bytes(&union));

    println!("size_od(MyCUnion)     : {}", size_of::<MyCUnion>());
    println!("size_od(MyStruct)     : {}", size_of::<MyStructWithAnEmptyField>()); // 2 in C/C++ by default
    println!("size_od(MyEmptyStruct): {}", size_of::<MyEmptyStruct>()); // always 1 in C/C++
    println!("size_od(MyCEnum)      : {}", size_of::<MyCEnum>());
    println!("size_od(MyEnum)       : {}", size_of::<MyEnum>());
}

#[test]
fn test() {
    main()
}
