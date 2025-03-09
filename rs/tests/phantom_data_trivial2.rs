/// Dummy iterator over a slice, which only records the current index
/// but must adhere to the lifetime of the original slice.
struct SliceIter<'a, T> {
    index: usize,
    _marker: std::marker::PhantomData<&'a T>, // ties the lifetime 'a to the iterator without storing T
}

impl<'a, T> SliceIter<'a, T> {
    fn new(_slice: &'a [T]) -> Self {
        SliceIter {
            index: 0,
            _marker: std::marker::PhantomData,
        }
    }
    fn advance(&mut self) {
        self.index += 1;
    }
}

fn main() {
    let mut iter: SliceIter<'_, _>;
    {
        let vector = vec![10, 20, 30];
        iter = SliceIter::new(&vector);
        iter.advance(); // can be used as long as `vector` is not dropped
    }
    // iter.advance(); // error: `vector` does not live long enough
}

#[test]
fn test() {
    main()
}
