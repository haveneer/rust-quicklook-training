#[cfg(test)]
mod tests {
    fn f1() -> i8 {
        let f = || return 1;
        f();
        0
    }

    fn f2() -> i8 {
        return 1;
        0
    }

    #[test]
    fn test1() {
        assert_eq!(f1(), 0);
        assert_eq!(f2(), 1);
    }
}
