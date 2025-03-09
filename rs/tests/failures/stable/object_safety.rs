use std::sync::Arc;

trait MyTrait {
    fn with_self_sized(&self)
    where
        Self: Sized; // object-safety: Sized Self
    fn with_self_return(self) -> Self; // object-safety: Self in return
    fn with_self_receiver(self); // object-safety: invalid receiver
    fn with_invalid_receiver(self: Arc<Box<Self>>); // object-safety: invalid receiver
}

struct MyType;
impl MyTrait for MyType {
    fn with_self_sized(&self) {}
    fn with_self_return(self) -> Self {
        self
    }
    fn with_invalid_receiver(self: Arc<Box<Self>>) {}
    fn with_self_receiver(self) {}
}

fn main() {
    let make_one = || -> Box<dyn MyTrait> { Box::new(MyType) };
    // but Ok with Box<MyType> return

    make_one().with_self_sized();
    make_one().with_self_return();
    Arc::new(make_one()).with_invalid_receiver();
    make_one().with_self_receiver();
}
