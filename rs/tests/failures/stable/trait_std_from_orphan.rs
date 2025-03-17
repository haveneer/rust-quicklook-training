struct MyType;

impl From<Vec<i32>> for MyType {
    fn from(_value: Vec<i32>) -> Self {
        todo!() // orphan rule: valide car appliqué sur MyType
    }
}

impl From<MyType> for Vec<i32> {
    fn from(_value: MyType) -> Self {
        todo!() // orphan rule: valide car From<MyType> est un type monomorphé local
    }
}

impl From<String> for Vec<i32> {
    fn from(_value: String) -> Self {
        todo!() // orphan rule: invalid car tout est non local
    }
}

fn main() {}
