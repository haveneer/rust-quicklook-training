#[derive(Default)]
struct MyStructWithManyFields {
    a: i8,
    b: u8,
    c: i16,
    d: u16,
    e: i32,
    f: u32,
    g: i64,
    h: u64,
}

fn main() {
    let x = MyStructWithManyFields {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
        g: 7,
        h: 8,
    };
    let y = MyStructWithManyFields {
        a: 100,
        h: 1000,
        ..x
    };
    let z = MyStructWithManyFields {
        a: 100,
        h: 1000,
        ..MyStructWithManyFields::default()
    };
}

#[test]
fn test() {
    main()
}
