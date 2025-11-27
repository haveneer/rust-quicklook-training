use std::any::Any;
use std::collections::HashMap;

// Base trait for all plugins
trait Plugin: Any {
    fn name(&self) -> &str;
    fn execute(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

// Concrete Plugin 1
struct LoggerPlugin {
    prefix: String,
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "Logger"
    }

    fn execute(&self) -> String {
        format!("{}: Logging started", self.prefix)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Concrete Plugin 2
struct MetricsPlugin {
    interval_ms: u64,
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> &str {
        "Metrics"
    }

    fn execute(&self) -> String {
        format!("Collecting metrics every {}ms", self.interval_ms)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Plugin Registry
struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    fn register(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }

    fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|b| b.as_ref())
    }

    // Typed retrieval with downcast
    fn get_typed<T: Plugin>(&self, name: &str) -> Option<&T> {
        self.get(name).and_then(|p| p.as_any().downcast_ref::<T>())
    }

    fn execute_all(&self) {
        for plugin in self.plugins.values() {
            println!("{}: {}", plugin.name(), plugin.execute());
        }
    }
}

fn main() {
    let mut registry = PluginRegistry::new();

    registry.register(Box::new(LoggerPlugin {
        prefix: "[APP]".to_string(),
    }));

    registry.register(Box::new(MetricsPlugin { interval_ms: 1000 }));

    println!("=== Executing all plugins ===");
    registry.execute_all();

    println!("\n=== Typed access ===");
    if let Some(logger) = registry.get_typed::<LoggerPlugin>("Logger") {
        println!("Logger prefix: {}", logger.prefix);
    }

    if let Some(metrics) = registry.get_typed::<MetricsPlugin>("Metrics") {
        println!("Metrics interval: {}ms", metrics.interval_ms);
    }
}

#[test]
fn test() {
    main()
}
