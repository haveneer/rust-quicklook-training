use darling_example::Config;

#[derive(Config)]
struct BadConfig {
    #[config(min = 10, max = 5)]
    level: i32,
}

fn main() {}
