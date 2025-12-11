use darling_example::Config;

// Enums are not supported by this derive; should fail with a helpful error
#[derive(Config)]
enum BadShape {
    A { x: i32 },
}

fn main() {}
