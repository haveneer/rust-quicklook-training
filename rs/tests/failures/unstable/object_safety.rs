trait MyTrait {
    fn with_self_sized(&self)
    where
        Self: Sized; // object-safety: Sized Self
    fn with_custom_receiver(self: Box<Self>); // object-safety: invalid receiver
    fn with_self_return(self) -> Self; // object-safety: Self in return
}

struct MyType;
impl MyTrait for MyType {
    fn with_self_sized(&self) {}
    fn with_custom_receiver(self: Box<Self>) {}
    fn with_self_return(self) -> Self {
        self
    }
}

fn main() {
    let obj: Box<dyn MyTrait> = Box::new(MyType);
    // but Ok with:
    // let obj = Box::new(MyType);

    obj.with_custom_receiver();
    obj.with_self_sized();
    obj.with_self_return();
}
