pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

trait Trait {
    fn explain(&self) {
        println!("on object {}", self.type_name());
    }
}

struct S;

impl dyn Trait {
    fn dyn_explain(&self) {
        println!("on dyn object {}", self.type_name());
    }
}
impl Trait for S {}
fn main() {
    let a = S;
    a.explain();
    // a.dyn_explain();
    let b: &dyn Trait = &S;
    b.explain();
    b.dyn_explain();
}

#[test]
fn test() {
    main()
}
