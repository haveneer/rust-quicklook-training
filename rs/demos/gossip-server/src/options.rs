use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// Port to listen
    #[clap(short, long, value_parser)]
    port: u16,

    /// Server to connect when starting
    #[clap(value_parser, name = "SERVER")]
    servers: Vec<std::net::SocketAddr>,
}

impl Options {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn servers(&self) -> &[std::net::SocketAddr] {
        &self.servers
    }
}
