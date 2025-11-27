// tests/reflection_limitations.rs
use std::any::{Any, TypeId};

trait Animal {
    fn speak(&self) -> &str;
}

#[derive(Debug)]
struct Dog;
impl Animal for Dog {
    fn speak(&self) -> &str { "Woof!" }
}

#[derive(Debug)]
struct Cat;
impl Animal for Cat {
    fn speak(&self) -> &str { "Meow!" }
}

fn main() {
    println!("=== Rust vs other languages ===\n");

    // ❌ NOT POSSIBLE in Rust (no complete runtime reflection)
    // - List all fields of a struct
    // - Call a method by its name (string)
    // - Create a type dynamically
    // - Modify private fields

    // ✅ POSSIBLE in Rust with Any
    let animals: Vec<Box<dyn Any>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    println!("Type checking with TypeId:");
    for animal in &animals {
        if animal.is::<Dog>() {
            println!("  - It's a dog!");
        } else if animal.is::<Cat>() {
            println!("  - It's a cat!");
        }
    }

    println!("\n=== Downcast ===");
    if let Some(dog) = animals[0].downcast_ref::<Dog>() {
        println!("Successful downcast: {:?}", dog);
    }

    println!("\n=== Type name introspection ===");
    println!("Dog type: {}", std::any::type_name::<Dog>());
    println!("Vec<i32> type: {}", std::any::type_name::<Vec<i32>>());

    // ✅ Alternative: use procedural macros (derive)
    println!("\n💡 For more reflection capabilities, use:");
    println!("   - Procedural macros (derive)");
    println!("   - serde for serialization");
    println!("   - Trait objects for polymorphism");
}

#[test]
fn test() {
    main()
}