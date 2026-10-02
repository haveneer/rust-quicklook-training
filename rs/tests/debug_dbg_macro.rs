#[derive(Debug)]
struct Cell {
    id: usize,
    volume: f64,
    neighbours: Vec<usize>,
}

fn total_volume(cells: &[Cell]) -> f64 {
    // dbg! prints file:line, the expression and its value to stderr, then returns the value
    dbg!(cells.iter().map(|c| c.volume).sum::<f64>())
}

fn main() {
    let cells = vec![
        Cell {
            id: 0,
            volume: 1.5,
            neighbours: vec![1],
        },
        Cell {
            id: 1,
            volume: 2.5,
            neighbours: vec![0],
        },
    ];
    let total = total_volume(&cells);
    debug_assert!(total > 0.0, "negative volume: {total}"); // removed in release builds
    println!("{:?}", cells[0]); // compact
    println!("{:#?}", cells[1]); // pretty-printed, one field per line
    let _ = dbg!(&cells[0].id, cells[1].neighbours.len()); // several expressions at once (borrow, don't move)
}

#[test]
fn test() {
    main()
}
