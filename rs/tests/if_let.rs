#![allow(dead_code)]
// See also while let

#[test]
fn test_if_let_simple() {
    let option = Some(1); // same with Result
    let r = if let Some(_i) = option { 1 } else { 0 };
    assert_eq!(r, 1);
}

#[test]
fn test_it_let_nested() {
    struct MyStruct {
        a: i32,
        b: i32,
    }
    enum MyEnum {
        A(MyStruct),
        B(i32),
    }

    // result -> option
    let x = MyEnum::A(MyStruct { a: 1, b: 2 });
    let r = if let MyEnum::A(MyStruct { a: 1, .. }) = x {
        1
    } else {
        0
    };
    assert_eq!(r, 1);
}

#[test]
fn test_while_let() {
    let mut v = (1..8).into_iter().map(|i| Some(i)).collect::<Vec<_>>();
    v.insert(3, None);

    while let Some(Some(i)) = v.pop() {
        println!("{i}");
    }
}

#[test]
fn test_if_let_scope() {
    mod hints {
        // The behavior changes after edition 2024
        pub const HINT1: &str = "after edition 2024 (≥1.85)";
        pub const HINT2: &str = "before edition 2024 (<1.85)";
    }

    let m = std::sync::Mutex::new(Option::<()>::None);

    if let Some(_) = *m.try_lock().unwrap() {
        println!("Passed in first lock");
    } else {
        if let Some(_) = m.try_lock().ok() {
            println!("Passed in second lock: {}", hints::HINT1);
        } else {
            println!("Already locked: {}", hints::HINT2);
        }
    }; // ; required for edition <2024
}
