use std::any::{Any, TypeId};

// A vector that enforces type homogeneity
struct TypedVec {
    items: Vec<Box<dyn Any>>,
    type_id: Option<TypeId>,
}

impl TypedVec {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            type_id: None,
        }
    }

    fn push<T: 'static>(&mut self, item: T) -> Result<(), &'static str> {
        let item_type_id = TypeId::of::<T>();

        match self.type_id {
            None => {
                // First element defines the type
                self.type_id = Some(item_type_id);
                self.items.push(Box::new(item));
                Ok(())
            }
            Some(expected_type_id) if expected_type_id == item_type_id => {
                self.items.push(Box::new(item));
                Ok(())
            }
            Some(_) => Err("Type mismatch: all items must have the same type"),
        }
    }

    fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.items
            .get(index)
            .and_then(|item| item.downcast_ref::<T>())
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn type_name(&self) -> Option<&'static str> {
        self.type_id.map(|_| {
            // Cannot get the name directly from TypeId
            // but we could maintain a HashMap<TypeId, &'static str>
            "unknown"
        })
    }
}

fn main() {
    let mut vec = TypedVec::new();

    // Push some i32
    vec.push(10).unwrap();
    vec.push(20).unwrap();
    vec.push(30).unwrap();

    println!("Vector has {} items", vec.len());

    // Typed retrieval
    if let Some(value) = vec.get::<i32>(1) {
        println!("Item at index 1: {}", value);
    }

    // Attempt to add a different type
    match vec.push("string") {
        Ok(_) => println!("Added string (unexpected!)"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Type safety demonstration ===");
    let mut vec2 = TypedVec::new();
    vec2.push(String::from("hello")).unwrap();
    vec2.push(String::from("world")).unwrap();

    // Retrieval with wrong type
    match vec2.get::<i32>(0) {
        Some(_) => println!("Got i32 (unexpected!)"),
        None => println!("Type mismatch: expected String, tried i32"),
    }

    match vec2.get::<String>(0) {
        Some(s) => println!("Got String: {}", s),
        None => println!("Not found"),
    }
}

#[test]
fn test() {
    main()
}
