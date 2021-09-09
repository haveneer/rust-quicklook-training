pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(divide_non_zero_result(1, 1), 2); // HINT never run
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

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
