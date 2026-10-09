use petgraph::algo::{dijkstra, toposort};
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;

fn main() {
    // Directed graph: nodes and edges carry data (here a name and a cost)
    let mut g: DiGraph<&str, u32> = DiGraph::new();
    let mesh = g.add_node("mesh");
    let assemble = g.add_node("assemble");
    let precond = g.add_node("precond");
    let solve = g.add_node("solve");
    let export = g.add_node("export");
    g.extend_with_edges([
        (mesh, assemble, 3),
        (assemble, precond, 2),
        (assemble, solve, 9),
        (precond, solve, 4),
        (solve, export, 1),
    ]);

    // Dependency order (fails with Err(cycle) if the graph has a cycle)
    let order = toposort(&g, None).expect("no cycle");
    println!(
        "order: {:?}",
        order.iter().map(|&n| g[n]).collect::<Vec<_>>()
    );

    // Shortest paths from `mesh`, edge weight as cost
    let cost = dijkstra(&g, mesh, None, |e| *e.weight());
    println!("mesh -> export costs {}", cost[&export]);

    // Graphviz export (Display of the node data): `dot -Tpng` to draw it
    println!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}

#[test]
fn test() {
    main()
}
