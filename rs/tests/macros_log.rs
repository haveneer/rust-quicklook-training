macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        println!("[{}] {}:{} - {}",
            $level,
            file!(),
            line!(),
            format!($($arg)*))
    };
}

fn main() {
    log!("INFO", "User {} logged in", "alice");
    log!("ERROR", "Connection failed");
}

#[test]
fn test() {
    main()
}
