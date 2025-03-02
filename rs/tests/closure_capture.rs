#![allow(dead_code)]

struct MyStruct {
    field1: [u8; 32],
    field2: Vec<u8>, // will contain 64 bytes
}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {
            field1: [0; 32],
            field2: vec![0; 64],
        }
    }
}

fn main() {
    let my_struct = MyStruct::new();
    let mut arr = [0u8; 128];
    arr[0] = 1;
    let mut c_by_val = move || {
        arr[0] = 2;
        println!("{} {}", arr[1], my_struct.field1[1])
    };
    arr[0] = 3;
    let c_by_ref = || println!("{} {}", arr[0], my_struct.field1[0]);
    c_by_val();
    c_by_ref();
    println!("size_of(c_by_ref) = {}", size_of_val(&c_by_ref));
    println!("size_of(c_by_val) = {}", size_of_val(&c_by_val));
}

#[test]
fn test() {
    main()
}
