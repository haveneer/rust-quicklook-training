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
        println!("Greetings form is '{}'", data); // As many as you want!
    }

    #[test]
    #[rustfmt::skip]
    fn closure_move() {
        let data = String::from("Hello");

        let f = /* move */ |x: &str| { // force move ?
            // Now 'data' is moved in this closure (and owned by it)
            let moved_data: String = data; // data is consumed here
            println!("{}, {}", moved_data, x);
        };

        f("John"); // Once and sometimes only once...
        // f("Doe"); // data moved from the closure to the body and then dropped
        // println!("Greetings form is '{}'", data); // error: value moved into closure
    }

    #[test]
    fn closure_exclusive_mut() {
        let mut data = String::from("Hello"); // must be 'mut' even if this is its moved version which will be mutated

        let mut f = |x: &str| {
            data.push_str(x);
            println!("New greetings is '{}'", data);
        };

        f(" wonderful");
        // println!("Greetings form is '{}'", data); // value still used by closure
        f(" master");
        println!("Greetings form is '{}'", data); // Ok closure lifetime is over
    }
}
