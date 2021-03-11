#[cfg(test)]
mod tests {
    fn calculate_length_bad(s: String) -> usize {
        s.len()
    }

    #[test]
    fn test5() {
        let str = String::from("Hello"); // HINT String is not copyable
        let len = calculate_length_bad(str);
        println!("The length of the string is {}", len);
        // println!("The string is {}", str); // FIXME error: borrow of moved value: `str`
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    #[test]
    fn test6() {
        let str = String::from("Hello");
        let len = calculate_length(&str);
        println!("The length of the string is {}", len);
        println!("The string is {}", str); // HINT no more error
    }

    #[test]
    fn test7() {
        let str = String::from("Hello");
        let ref str1 = str; // equivalent declarations
        let str2 = &str;

        let len1 = calculate_length(&str1);
        let len2 = calculate_length(&str2);
        assert_eq!(len1, len2);
    }
}
