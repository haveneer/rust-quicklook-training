trait Vector {
    type Data;
    // type defined by impl
    fn sum(&self) -> Self::Data; // Data is already accessible
}

#[derive(Debug)]
struct MyVector {
    data: [f64; 8],
}

impl Vector for MyVector {
    type Data = f64; // impl of trait subtype

    fn sum(&self) -> Self::Data {
        self.data
            .iter()
            .fold(Self::Data::default(), |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        type X = <MyVector as Vector>::Data;
        let x = X::default();
        let v = MyVector { data: [x; 8] };
        println!("v = {:?}", v);
        println!("v.sum() = {}", v.sum());
    }
}
