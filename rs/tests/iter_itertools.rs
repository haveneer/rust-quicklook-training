use itertools::{iproduct, izip, Itertools, MinMaxResult};

fn main() {
    let v = [3, 1, 4, 1, 5, 9, 2, 6];

    // Shortcuts: collect_vec, join, sorted (returns an iterator), unique, dedup
    assert_eq!(
        v.iter().sorted().dedup().collect_vec(),
        [&1, &2, &3, &4, &5, &6, &9]
    );
    assert_eq!(v.iter().unique().join("-"), "3-1-4-5-9-2-6");

    // tuple_windows / tuples: fixed-size windows or chunks as tuples, on any iterator
    assert_eq!(v.iter().tuple_windows().filter(|(a, b)| b > a).count(), 4);
    let pairs: Vec<(i32, i32)> = v.into_iter().tuples().collect();
    assert_eq!(pairs, [(3, 1), (4, 1), (5, 9), (2, 6)]);

    // chunk_by: group *consecutive* items sharing a key (like Unix uniq -c)
    let chunks = "aaabccdd".chars().chunk_by(|&c| c);
    let rle = chunks.into_iter().map(|(c, g)| format!("{c}{}", g.count()));
    assert_eq!(rle.collect::<String>(), "a3b1c2d2");

    assert_eq!(v.iter().minmax(), MinMaxResult::MinMax(&1, &9)); // one pass, 1.5n comparisons

    // Combining: cartesian product, n-ary zip, interleave
    let grid = iproduct!(0..2, 0..3).collect_vec();
    let rows = izip!([1, 2], ["a", "b"], [true, false]).collect_vec();
    assert_eq!((1..4).interleave(10..12).collect_vec(), [1, 10, 2, 11, 3]);
    println!("{grid:?}\n{rows:?}");

    // Merging sorted sequences: kmerge (k-way merge, like the end of a merge sort)
    let merged = [vec![1, 4], vec![2, 3], vec![0, 5]].into_iter().kmerge();
    assert_eq!(merged.collect_vec(), [0, 1, 2, 3, 4, 5]);
}

#[test]
fn test() {
    main()
}
