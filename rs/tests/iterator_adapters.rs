#[test]
fn map_filter_collect() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Chaîne d'adaptateurs : filtrer, transformer, puis collecter
    let doubled_evens: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 2)
        .collect();

    assert_eq!(doubled_evens, vec![4, 8, 12, 16, 20]);
}

#[test]
fn fold_sum_of_squares() {
    let numbers = [1, 2, 3, 4];

    // fold : accumule un résultat au fil de l'itération (somme des carrés)
    let sum_of_squares = numbers.iter().fold(0, |acc, &n| acc + n * n);

    assert_eq!(sum_of_squares, 1 + 4 + 9 + 16);
}

#[test]
fn lazy_evaluation() {
    // Les adaptateurs sont paresseux : rien ne tourne avant la consommation
    // (par .collect(), .sum(), un for, ...).
    let it = (1..=3)
        .inspect(|n| println!("lu {n}")) // voir passer les éléments
        .map(|n| n * n);
    // le type de it, chemins (core::iter::...) ellipsés
    let ty = {
        let full = std::any::type_name_of_val(&it);
        full.split_inclusive(['<', '>', ',', ' '])
            .map(|part| part.rsplit("::").next().unwrap())
            .collect::<String>()
    };
    println!("{ty} : {} octets", size_of_val(&it));

    println!("adaptateurs construits, rien n'a encore tourné");

    let total: i32 = it.sum(); // c'est ici que "lu ..." s'affiche
    assert_eq!(total, 1 + 4 + 9);
}
