#[cfg(test)]
mod tests {

    #[derive(Default)]
    struct Struct {
        a: i8,
        b: i8,
    }

    fn f() -> Struct {
        Struct::default()
    }

    #[test]
    fn test() {
        let Struct { a, b } = f();
    }
}
