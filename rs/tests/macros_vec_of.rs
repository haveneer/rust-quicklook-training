// Macro pour créer un Vec
macro_rules! vec_of {
    ($($element:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut v = Vec::new();
            $(
                v.push($element);
            )*
            v
        }
    };
}

fn main() {
    let v = vec_of!(1, 2, 3, 4, 5);
    println!("{:?}", v); // [1, 2, 3, 4, 5]

    let empty: Vec<i32> = vec_of!();
    println!("{:?}", empty); // []
}

#[test]
fn test() {
    main()
}
