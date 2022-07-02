// More info:
// * https://doc.rust-lang.org/core/marker/struct.PhantomData.html

#[cfg(test)]
mod tests {
    use super::*;

    struct X<T> {
        field: isize,
        phantom: core::marker::PhantomData<T>,
    }

    impl<T> X<T> {
        pub fn new() -> X<T> {
            Self {
                field: 0,
                phantom: core::marker::PhantomData::<T>, // keep type T in struct; cost 0 byte
            }
        }
    }

    #[test]
    fn test() {
        let x = X::<i64>::new();
        let y = X::<i32>::new();

        let z = y.phantom;

        // Phantom data does not change size
        assert_eq!(std::mem::size_of::<X::<i64>>(), std::mem::size_of::<i64>());
    }
}
