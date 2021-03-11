#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let s = String::from("hello");

        let len = s.len();

        let slice1 = &s[2..5]; // slice are read only reference to a container
        let slice2 = &s[0..len]; // original owner lifetime must be greater
        let slice3 = &s[..];

        println!("slice1 = {}", slice1);
        println!("slice2 = {}", slice2);
        println!("slice3 = {}", slice3);
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    #[test]
    fn test2() {
        let s = String::from("hello world");
        let slice = first_word(&s);
        println!("slice = {}", slice);
    }

    fn first_word_of_slice(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    #[test]
    fn test3() {
        let s = String::from("hello world");
        let slice = first_word_of_slice(&s[..]);
        println!("slice = {}", slice);
    }
}
