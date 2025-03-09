use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

trait AnyDebug: Any + Debug {
    fn as_any(&self) -> &dyn Any; // Utility method for downcasting
}
impl<T: Any + Debug> AnyDebug for T {
    fn as_any(&self) -> &dyn Any {
        self // Upcast: allows treating Self as &dyn Any
    }
}

impl dyn AnyDebug {
    // Generic method to convert (downcast) a trait object into a reference to its concrete type
    fn downcast_ref<T: AnyDebug + 'static>(&self) -> Option<&T> {
        // T must implement Animal and be 'static for downcasting via Any to be possible
        self.as_any().downcast_ref::<T>()
    }
}

struct Config {
    data: HashMap<String, Box<dyn AnyDebug>>, // Stores dynamically typed values
}

impl Config {
    fn new() -> Self {
        Config {
            data: HashMap::new(),
        }
    }

    // Function to insert a value into the configuration
    fn insert<T: 'static + Debug>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }

    // Function to retrieve a value with safe downcasting
    fn get<T: 'static + Debug>(&self, key: &str) -> Option<&T> {
        self.data
            .get(key)
            .and_then(|value| value.downcast_ref::<T>()) // Safe downcasting
    }
}

fn main() {
    let mut config = Config::new();

    // Insert different types of data
    config.insert("max_connections", 100);
    config.insert("api_url", String::from("https://api.example.com"));
    config.insert("debug_mode", true);
    config.insert("allowed_ips", vec!["192.168.1.1", "10.0.0.1"]);

    // Show config content
    config.data.iter().for_each(|(k, v)| {
        println!("{} -> {:?}", k, v);
    });

    // Retrieve and display values with safe downcasting
    if let Some(max_connections) = config.get::<i32>("max_connections") {
        println!("Max Connections: {}", max_connections);
    }

    if let Some(api_url) = config.get::<String>("api_url") {
        println!("API URL: {}", api_url);
    }

    if let Some(debug_mode) = config.get::<bool>("debug_mode") {
        println!("Debug Mode: {}", debug_mode);
    }

    if let Some(allowed_ips) = config.get::<Vec<&str>>("allowed_ips") {
        println!("Allowed IPs: {:?}", allowed_ips);
    } else {
        println!("Allowed IPs: Not found or wrong type.");
    }
}

#[test]
fn test() {
    main()
}
