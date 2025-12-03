macro_rules! with_temp {
    ($e:expr) => {{
        let temp = $e;
        temp * 2
    }};
}

fn main() {
    let temp = 5;
    let result = with_temp!(10);
    println!("temp = {}, result = {}", temp, result);
    // temp = 5, result = 20
    // Pas de collision !
}

#[test]
fn test() {
    main()
}
