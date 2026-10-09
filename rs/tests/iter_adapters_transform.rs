fn main() {
    let cells = ["12.5", "x", "3.0", "", "7.25"];

    // map / filter / filter_map: transform, keep, or both at once
    let lengths: Vec<usize> = cells.iter().map(|c| c.len()).collect();
    let non_empty: Vec<&&str> = cells.iter().filter(|c| !c.is_empty()).collect();
    let values: Vec<f64> = cells.iter().filter_map(|c| c.parse().ok()).collect();
    println!("{lengths:?} {non_empty:?} {values:?}");

    // enumerate: (index, item) without a manual counter
    for (i, v) in values.iter().enumerate() {
        print!("#{i}={v} ");
    }

    // zip: walk two sequences together (stops at the shortest)
    let weights = [0.5, 0.25, 0.25];
    let mean: f64 = values.iter().zip(weights).map(|(v, w)| v * w).sum();
    println!("\nweighted mean = {mean}");

    // chain / rev: concatenate, reverse (double-ended iterators)
    let all: Vec<i32> = (1..=3).chain(7..=8).rev().collect();
    assert_eq!(all, [8, 7, 3, 2, 1]);

    // flatten / flat_map: one level of nesting less
    let faces = [vec![0, 1, 2], vec![2, 3], vec![]];
    let nodes: Vec<i32> = faces.iter().flatten().copied().collect();
    assert_eq!(nodes, [0, 1, 2, 2, 3]);
    let edges: Vec<&[i32]> = faces.iter().flat_map(|f| f.windows(2)).collect();
    assert_eq!(edges, [[0, 1], [1, 2], [2, 3]]);

    // inspect: look at the items passing by (debugging a chain)
    let evens = (1..=4).inspect(|x| print!("[{x}]")).filter(|x| x % 2 == 0);
    println!(" -> {} even", evens.count());
}

#[test]
fn test() {
    main()
}
