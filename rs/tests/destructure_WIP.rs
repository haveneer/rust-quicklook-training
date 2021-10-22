#[cfg(test)]
mod tests {

    #[derive(Default)]
    struct MyStruct {
        a: i8,
        b: i8,
    }

    fn f() -> MyStruct {
        MyStruct::default()
    }

    #[test]
    fn test() {
        let MyStruct { a, b } = f();
    }
}
