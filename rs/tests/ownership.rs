#[cfg(test)]
mod tests {
    #[test]
    fn implicit_move() {
        let numbers = vec![1, 2, 3, 4, 5];

        let other_numbers = numbers;

        println!("{:?}", other_numbers);
        // numbers is freed here
    }

    #[test]
    fn explicit_clone() {
        // Allocate array on heap
        let numbers = vec![1, 2, 3, 4, 5];
        println!("{:?}", numbers);

        // Move ownership to other_numbers
        let other_numbers = numbers;
        println!("{:?}", other_numbers);

        // Now we cannot access numbers anymore because value was moved.
        // println!("{:?}", numbers); // error: does not COMPILE

        // Make a (deep) copy -> no move of ownership
        let cloned_numbers = other_numbers.clone();
        println!("clone = {:?}, source = {:?}", cloned_numbers, other_numbers);
        // Free numbers AND other_numbers vectors
    }

    mod move_and_functions {
        #[test]
        fn move_and_functions() {
            let numbers = vec![1, 2, 3, 4, 5];
            consume(numbers);  // Gives ownership to `consume`

            let produced_numbers = produce();  // Takes ownership
            println!("{:?}", produced_numbers);
            // produced_numbers gets out of scope -> free memory
        }

        fn consume(numbers: Vec<i32>) {
            let sum: i32 = numbers.iter().sum();
            println!("The sum is {}", sum);
            // numbers gets out of scope -> free memory
        }

        fn produce() -> Vec<i32> {
            let mut numbers: Vec<i32> = Vec::new();
            for i in 0..4 { numbers.push(i); }
            numbers  // Gives ownership to caller : NO COPY
        }
    }

    mod borrow_and_functions {
        #[test]
        fn borrow_and_functions() {
            let mut numbers = vec![1, 2, 3, 4, 5];

            println!("The sum is {}",    // Passes reference, 
                     consume(&numbers)); // keeps ownership
            println!("The sum is {}",                // Mutable reference,
                     add_and_consume(&mut numbers)); // keeps ownership

            println!("{:?}", numbers);
        }

        fn consume(numbers: &Vec<i32>) -> i32 {
            // numbers is READ-ONLY, cannot be mutated
            // numbers.push(42);  // error: does NOT COMPILE
            let sum: i32 = numbers.iter().sum();
            sum
        }

        fn add_and_consume(numbers: &mut Vec<i32>) -> i32 {
            numbers.push(42);
            consume(numbers)
        }
    }

    #[cfg(feature = "compiletest")]
    use compiler_test;

    #[cfg(feature = "compiletest")]
    #[test]
    fn ownership_failures() {
        compiler_test::run_mode("compile-fail", Some("ownership_failures"),
                                vec![
                                    "implicit_move",
                                    "explicit_clone",
                                    "borrow_and_functions",
                                    // "in_async", // compiletest_rs seems not able to load crates
                                ]);
    }
}