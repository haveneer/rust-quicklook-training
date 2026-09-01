#[test]
fn map_filter_collect() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

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
    let numbers = vec![1, 2, 3, 4];

    // fold : accumule un résultat au fil de l'itération (ici la somme des carrés)
    let sum_of_squares = numbers.iter().fold(0, |acc, &n| acc + n * n);

    assert_eq!(sum_of_squares, 1 + 4 + 9 + 16);
}

#[test]
fn lazy_evaluation() {
    // Les adaptateurs sont paresseux : rien ne s'exécute avant d'être consommés
    // (par .collect(), .sum(), un for, ...).
    let it = (1..=3).map(|n| {
        println!("mapping {n}");
        n * n
    });
    println!("adaptateur construit, rien n'a encore tourné");

    let total: i32 = it.sum(); // c'est ici que "mapping ..." s'affiche
    assert_eq!(total, 1 + 4 + 9);
}
