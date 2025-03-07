use std::marker::PhantomData;

struct NotConnected;
struct Connected;
struct Disconnected;

struct Network<Status = NotConnected> {
    address: String,
    status: PhantomData<Status>,
}

// Interface when not connected
impl Network<NotConnected> {
    pub fn new(address: String) -> Network<NotConnected> {
        Self::private_new(address)
    }

    fn private_new<T>(address: String) -> Network<T> {
        Network {
            address,
            status: PhantomData,
        }
    }

    pub fn connect(self) -> Network<Connected> {
        Network::private_new(self.address)
    }
}

// Interface when connected
impl Network<Connected> {
    pub fn request(&self) -> String {
        "request".into()
    }
    pub fn disconnect(self) -> Network<Disconnected> {
        Network::private_new(self.address)
    }
}

fn main() {
    let network /* : Network<NotConnected> */ = Network::new("127.0.0.1:8001".to_string());

    // Cannot create object at an arbitrary state
    // let invalid: Network<Connected> = Network::new("127.0.0.1:8001".to_string());

    let connected_network = network.connect();
    // .request() is only available on Network<Connected>
    assert_eq!(connected_network.request(), "request");

    connected_network.disconnect();
}

#[test]
fn test() {
    main()
}
