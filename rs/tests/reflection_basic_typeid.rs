use std::any::{Any, TypeId};

fn main() {
    // TypeId allows comparing types at runtime
    let type_of_i32 = TypeId::of::<i32>();
    let type_of_string = TypeId::of::<String>();

    println!("i32 TypeId: {:?}", type_of_i32);
    println!("String TypeId: {:?}", type_of_string);
    println!("Are they equal? {}", type_of_i32 == type_of_string);

    // TypeId is unique for each concrete type
    assert_eq!(TypeId::of::<i32>(), TypeId::of::<i32>());
    assert_ne!(TypeId::of::<i32>(), TypeId::of::<i64>());

    // LIMITATION: Generic types are distinct after monomorphization
    assert_ne!(TypeId::of::<Vec<i32>>(), TypeId::of::<Vec<String>>());

    // We can use it with Any
    let value: Box<dyn Any> = Box::new(42i32);
    // Without explicit dereferencing, we get the type of the boxed value
    if (*value).type_id() == TypeId::of::<i32>() {
        println!("This is indeed an i32!");
    }
}

#[test]
fn test() {
    main()
}
