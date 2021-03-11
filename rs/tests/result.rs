mod tests {
    use std::{error, fmt};

    #[derive(Debug)]
    struct MyError {
        msg: String, // requires dynamic allocation
    }

    fn f1(n: Option<i64>) -> Result<i64, MyError> {
        n.ok_or(MyError {
            msg: format!("Error with always dynamically allocated message: {}", "BAD"),
        }) //
        .and_then(|n| Ok(n))
    }

    fn f2(n: Option<i64>) -> Result<i64, MyError> {
        n.ok_or_else(|| MyError {
            // this is a closure
            msg: format!(
                "Error with on demand dynamically allocated message: {}",
                "NOT NO BAD"
            ),
        })
        .and_then(|n| Ok(n))
    }

    #[derive(Debug)]
    enum MyError2 {
        // does not require dynamic allocation : GOOD
        EasyError,
        ComplexError { param1: &'static str, param2: usize },
    }

    impl fmt::Display for MyError2 {
        // Pretty display to render optimized Error type
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                MyError2::EasyError => write!(f, "Easy Error"),
                MyError2::ComplexError { param1, param2 } => write!(f, "{}  {}", param1, param2),
            }
        }
    }

    fn f3(n: Option<i64>) -> Result<i64, MyError2> {
        n.ok_or_else(|| MyError2::ComplexError {
            param1: "Fixed string",
            param2: 666,
        })
        .and_then(|n| Ok(n))
    }

    impl error::Error for MyError2 {}

    // Runtime defined Error type
    fn f4(n: Option<i64>) -> Result<i64, Box<dyn error::Error>> {
        n.ok_or_else(|| MyError2::EasyError.into())
            .and_then(|n| Ok(n))
    }

    fn show<E: std::fmt::Debug>(r: Result<i64, E>) {
        match r {
            Ok(r) => println!("Ok = {}", r),
            Err(e) => println!("Err = {:?}", e),
        }
    }

    #[test]
    fn test_result() {
        show(f1(None));
        show(f1(Some(1)));
        show(f2(None));
        show(f2(Some(1)));
        show(f3(None));
        show(f3(Some(1)));
        show(f4(None));
        show(f4(Some(1)));
    }
}
