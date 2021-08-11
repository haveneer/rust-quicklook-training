// See also while let

#[cfg(test)]
mod tests {
    #[test]
    fn test_option() {
        let option = Some(1);
        let f = || {
            if let Some(_i) = option {
                1
            } else {
                0
            }
        };
        assert_eq!(f(), 1);
    }

    #[test]
    fn test_result() {
        let result: Result<i8, ()> = Ok(1);
        let f = || {
            if let Ok(_i) = result {
                1
            } else {
                0
            }
        };
        assert_eq!(f(), 1);
    }

    #[test]
    fn test_mixed1() {
        // result -> option
        let result: Result<i8, ()> = Ok(1);
        let f = || {
            if let Some(_i) = result.ok() {
                1
            } else {
                0
            }
        };
        assert_eq!(f(), 1);
    }

    #[test]
    fn test_mixed2() {
        // option -> result
        let option = Some(1);
        let f = || {
            if let Ok(_i) = option.ok_or(()) {
                1
            } else {
                0
            }
        };
        assert_eq!(f(), 1);
    }
}
