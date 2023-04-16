#[cfg(test)]
mod tests {
    #[test]
    fn closure_shared_read() {
        let data = String::from("Hello");

        let f = |x: &str| {
            // Capture data by reference
            println!("{}, {}", data, x);
        };

        f("world");
        println!("Greetings form is '{}'", data);
        f("John");
    }

    #[test]
    fn closure_move() {
        let data = String::from("Hello");

        let f = move |x: &str| {
            // Now data is moved in this closure (and owned by it)
            println!("{}, {}", data, x);
        };

        f("world");
        // println!("Greetings form is '{}'", data); // error: value moved into closure
        f("John");
    }

    #[test]
    fn closure_move_and_mut() {
        let mut data = String::from("Hello"); // must be 'mut' even if this is its moved version which will be mutated

        let mut f = move |x: &str| {
            // Now data is moved in this closure, and must be 'mut' to be able to modify it
            data.push_str(x);
            println!("New greetings is '{}'", data);
        };

        f(" great");
        f(" wonderful");
        f(" master");
        // println!("Greetings form is '{}'", data); // error: value moved into closure
    }

    #[test]
    fn closure_move_and_back() {
        let mut data = String::from("Hello");

        let f = move |x: &str| -> String {
            // Now data is moved in this closure and goes out when called
            data.push_str(x);
            data
        };
        // println!("Greetings form is '{}'", data); // error: value moved into closure
        let data = f(" great");
        // m(" great"); // error: closure cannot be invoked more than once because it moves the variable `data` out of its environment

        println!("Greetings form is '{}'", data); // error: value moved into closure
    }
}
