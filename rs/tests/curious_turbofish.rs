trait MyTrait {
    fn new() -> Self;
    fn len(&self) -> usize;
}

impl<T> MyTrait for Vec<T> {
    fn new() -> Self {
        println!("Vec::new from MyTrait");
        Vec::<T>::new()
    }

    fn len(&self) -> usize {
        Vec::<T>::len(&self)
    }
}

#[test]
fn test() {
    let v = vec![1];
    let v: Vec<i32> = Vec::new(); // usual implicit form
    let v = Vec::<i32>::new(); // turbo fish without outer < >
    let v = <Vec<i32>>::new(); // no turbo fish but outer < >
    let v = <Vec<i32> as MyTrait>::new(); // explicit ambiguous call
    <Vec<i32> as MyTrait>::len(&v); // MyTrait redefines len()
    let s = <String>::new(); // <T>::method syntax (sometimes useful to remove ambiguities)
}
