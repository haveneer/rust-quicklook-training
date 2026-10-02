// cargo build --example debug_target
// rust-lldb target/debug/examples/debug_target
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
enum Boundary {
    Wall,
    Inlet { velocity: f64 },
}

#[allow(dead_code)] // fields are read from the debugger
#[derive(Debug)]
struct Mesh {
    name: String,
    nodes: Vec<[f64; 2]>,
    boundary: Option<Boundary>,
    tags: HashMap<&'static str, usize>,
}

#[allow(clippy::let_and_return)] // keeps a line to stop on
fn node_count(mesh: &Mesh) -> usize {
    let count = mesh.nodes.len(); // breakpoint here
    count
}

fn main() {
    let mesh = Mesh {
        name: String::from("naca0012"),
        nodes: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 0.1]],
        boundary: Some(Boundary::Inlet { velocity: 12.5 }),
        tags: HashMap::from([("wing", 2), ("farfield", 1)]),
    };
    let wall = Boundary::Wall;
    println!("{} nodes, {:?}", node_count(&mesh), wall);
}
