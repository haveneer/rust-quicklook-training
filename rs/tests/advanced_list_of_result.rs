#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    pub enum DivisionError {
        NotDivisible,
        DivideByZero,
    }

    pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
        if b == 0 {
            Err(DivisionError::DivideByZero)
        } else if a % b == 0 {
            Ok(a / b)
        } else {
            Err(DivisionError::NotDivisible)
        }
    }

    // Tests that verify your `divide` function implementation
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn list_of_results() {
        let numbers = vec![2, 3, 5, 7];
        let division_results = numbers.into_iter().map(|n| divide(120, n));
        let x: Vec<_> = division_results.collect(); // HINT list of result
        assert_eq!(
            format!("{:?}", x),
            "[Ok(60), Ok(40), Ok(24), Err(NotDivisible)]"
        );
    }

    #[test]
    fn result_with_list() {
        let numbers = vec![2, 3, 5, 6];
        let division_results = numbers.into_iter().map(|n| divide(120, n));
        let x: Result<Vec<i32>, DivisionError> = division_results.collect(); // HINT result of list!
        assert_eq!(format!("{:?}", x), "Ok([60, 40, 24, 20])");
    }

    #[test]
    fn result_with_list_with_error() {
        let numbers = vec![2, 3, 5, 7];
        let division_results = numbers.into_iter().map(|n| divide(120, n));
        let x: Result<Vec<i32>, DivisionError> = division_results.collect(); // HINT result of list!
        assert_eq!(format!("{:?}", x), "Err(NotDivisible)"); // collect first error
    }

    #[test]
    fn result_with_list_with_errors() {
        let numbers = vec![2, 3, 0, 7];
        let division_results = numbers.into_iter().map(|n| divide(120, n));
        let x: Result<Vec<i32>, DivisionError> = division_results.collect(); // HINT result of list!
        assert_eq!(format!("{:?}", x), "Err(DivideByZero)"); // collect first error
    }
}
