use std::collections::HashMap;

macro_rules! hash_map {
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    let m = hash_map!(
        "a" => 1,
        "b" => 2,
        "c" => 3,
    );
    println!("{:?}", m);
}

#[test]
fn test() {
    main()
}
