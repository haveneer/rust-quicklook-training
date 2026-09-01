// Depuis 1.82, `use<'a>` remplace le bound `+ 'a` pour capturer explicitement
// la lifetime de `self` dans le RPIT — plus lisible, et déjà valable sous édition 2021.

struct Data {
    values: Vec<i32>,
}

impl Data {
    fn count_positive<'a>(&'a self) -> impl Iterator<Item = usize> + use<'a> {
        self.values.iter().filter(|&&v| v > 0).map(|_| 1)
    }
}

fn main() {
    let d = Data {
        values: vec![-1, 2, 3, -4, 5],
    };
    let n: usize = d.count_positive().sum();
    println!("{n}");
}

#[test]
fn test() {
    main();
}
