/// A generic identifier associated with a type `T`, without storing an actual `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Id<T> {
    value: u64,
    _marker: std::marker::PhantomData<T>, // phantom marker linking the ID to a type T
}

impl<T> Id<T> {
    fn new(val: u64) -> Self {
        Id {
            value: val,
            _marker: std::marker::PhantomData,
        }
    }
}

struct User;
struct Product;
type UserId = Id<User>; // specific ID types
type ProductId = Id<Product>; // specific ID types

fn main() {
    let user_id: UserId = Id::new(100);
    let product_id: ProductId = Id::new(100);
    assert_eq!(size_of::<UserId>(), size_of::<u64>()); // no size cost

    // let invalid: UserId = product_id; // error: incompatible types
    // user_id == product_id; // error: comparison of different types
}

#[test]
fn test() {
    main()
}
