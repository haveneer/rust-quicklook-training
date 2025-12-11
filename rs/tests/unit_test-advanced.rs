#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 1, 2)]
    #[case(1 << 31, 1 << 31, 1 << 32)]
    fn sum_two_numbers_should_be_verified(#[case] a: usize, #[case] b: usize, #[case] sum: usize) {
        assert_eq!(a + b, sum);
    }

    use mockall::{automock, predicate};

    #[automock]
    trait AddOne {
        fn add_one(&self, x: u32) -> u32;
    }

    fn use_add_one_on_4(x: &impl AddOne) -> u32 {
        x.add_one(4)
    }

    #[test]
    fn mytest() {
        let mut mock = MockAddOne::new();
        mock.expect_add_one()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, use_add_one_on_4(&mock));
    }
}
