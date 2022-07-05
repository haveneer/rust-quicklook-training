use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// Port to listen
    #[clap(short, long, value_parser)]
    port: u16,

    /// Server to connect when starting
    #[clap(value_parser)]
    entry_point: Vec<String>,
}

impl Options {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn entry_points(&self) -> &Vec<String> {
        &self.entry_point
    }
}
