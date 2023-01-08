use std::marker::PhantomData;

struct Connected;

struct NotConnected;

struct Network<Status = NotConnected> {
    status: PhantomData<Status>,
}

// Interface when not connected
impl Network<NotConnected> {
    pub fn new<T>() -> Network<T> {
        Network {
            status: PhantomData,
        }
    }

    pub fn connect(self) -> Network<Connected> {
        Network::new()
    }
}

// Interface when connected
impl Network<Connected> {
    pub fn request(self) -> String {
        "request".into()
    }
}

fn main() {
    let network /* : Network<NotConnected> */ = Network::new();
    let connected_network = network.connect();
    // ?.request() is only available on Network<Connected>
    assert_eq!(connected_network.request(), "request");
}

#[test]
fn test() { main() }