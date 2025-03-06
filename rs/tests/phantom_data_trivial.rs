// More info:
// * https://doc.rust-lang.org/core/marker/struct.PhantomData.html

struct X<T> {
    field: isize,
    phantom: core::marker::PhantomData<T>, // without the marker T is unused; it doesn't compile
}

impl<T> X<T> {
    pub fn new() -> X<T> {
        Self {
            field: 0,
            phantom: core::marker::PhantomData::<T>, // keep type T in struct; cost 0 byte
        }
    }
}

fn main() {
    let x = X::<i64>::new();
    let y = X::<i32>::new();

    let z = y.phantom;

    // Phantom data does not change size
    assert_eq!(std::mem::size_of::<X::<i64>>(), std::mem::size_of::<i64>());
}

#[test]
fn test() {
    main()
}
