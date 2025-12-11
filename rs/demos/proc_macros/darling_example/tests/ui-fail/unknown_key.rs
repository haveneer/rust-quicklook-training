use darling_example::Config;

#[derive(Config)]
struct UnknownKeyCfg {
    #[config(does_not_exist = true)]
    field: u8,
}

fn main() {}
