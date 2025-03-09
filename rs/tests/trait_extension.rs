pub trait IteratorExt: Iterator {
    fn accumulate_map<U, F>(self, f: F) -> Vec<U>
    where
        Self: Sized,               // `Self` should be "Sized" to consume it as "self"
        F: FnMut(Self::Item) -> U, // Transformation to apply
    {
        self.map(f).collect() // map and collect at once
    }
}

impl<T> IteratorExt for T where T: Iterator {} // because Iterator is a trait not a type

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubles = numbers.into_iter().accumulate_map(|x| x * 2);
    println!("Doubles : {:?}", doubles);
}

#[test]
fn test() {
    main()
}
