struct Value;

trait Trait {
    fn method(self);
}

impl Trait for Value {
    // TODO 1. What if you comment out this impl?
    fn method(self) {
        println!("called on Value");
    }
}

impl Trait for &Value {
    // TODO 2. What if you comment out this impl and call (&v).method();
    fn method(self) {
        println!("called on &Value");
    }
}

impl Trait for &&Value {
    fn method(self) {
        println!("called on &&Value");
    }
}

//#region [TODO 3. Even deeper: comment out all previous impl]
struct EndValue;

impl Trait for &EndValue {
    fn method(self) {
        println!("called on EndValue");
    }
}

impl std::ops::Deref for Value {
    type Target = EndValue;

    fn deref(&self) -> &Self::Target {
        &EndValue
    }
}
//#endregion

fn main() {
    let v = Value;
    v.method();
}

#[test]
fn test() {
    main()
}
