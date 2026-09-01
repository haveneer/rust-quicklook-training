struct Data {
    values: Vec<i32>,
}

impl Data {
    // Sans lifetime explicite +'a, ne compile pas : le type caché de l'itérateur
    // emprunte `self`, mais `Item = usize` ne porte aucune lifetime pour le laisser deviner.
    // error[E0700]: hidden type for `impl Iterator<Item = usize>` captures lifetime that does not appear in bounds

    // note: il faut donc lier manuellement la lifetime du paramètre au RPIT.
    fn count_positive<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
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
