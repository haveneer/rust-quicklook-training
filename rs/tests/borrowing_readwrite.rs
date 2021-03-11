#[cfg(test)]
mod tests {
    fn change_bad(s: &String) {
        // s.push_str(" world"); // error: cannot borrow `*s` as mutable, as it is behind a `&` reference
    }

    #[test]
    fn test7() {
        let str = String::from("Hello");
        change_bad(&str); // FIXME change does not apply
        println!("The string is {}", str);
    }

    fn change(s: &mut String) {
        s.push_str(" world"); // HINT no more error (3 mut annotations to solve the problem)
    }

    #[test]
    fn test8() {
        let mut str = String::from("Hello");
        change(&mut str);
        println!("The string is {}", str);
    }

    #[test]
    // Exlusive mut borrowing
    fn test9() {
        let mut str = String::from("Hello");
        let r1 = &mut str;
        // let r2 = &mut str; // FIXME what happens with this line ?
        change(r1);

        println!("The string is {}", str);
    }

    #[test]
    // Exlusive mut borrowing
    fn test10() {
        // We change the order of the operations
        let mut str = String::from("Hello");
        let r1 = &mut str;
        change(r1);

        let r2 = &mut str;
        change(r2);

        println!("The string is {}", str);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    #[test]
    // Exlusive mut borrowing
    fn test11() {
        let mut str = String::from("Hello");
        let r1 = &mut str;
        // let r2  = &str; // TODO what happens if we enable these two lines
        // let len = calculate_length(r2);
        change(r1);
        // let r2  = &str; // TODO what happens if we enable these two lines
        // let len = calculate_length(r2);

        println!("The string is {}", str);
    }
}
