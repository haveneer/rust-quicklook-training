#[test]
fn vec_construction() {
    let empty: Vec<i32> = Vec::new();
    let from_macro = vec![1, 2, 3];
    let mut with_capacity: Vec<i32> = Vec::with_capacity(10); // pré-alloue, len reste 0
    with_capacity.push(1);
    let from_iter: Vec<i32> = (1..=3).collect();

    assert!(empty.is_empty());
    assert_eq!(from_macro, vec![1, 2, 3]);
    assert_eq!(with_capacity, vec![1]);
    assert_eq!(from_iter, vec![1, 2, 3]);
}

#[test]
fn vec_size_and_capacity() {
    let mut v: Vec<i32> = Vec::with_capacity(4);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 4);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 4); // pas de réallocation tant que la capacité suffit

    v.push(3);
    v.push(4);
    v.push(5); // dépasse la capacité -> réallocation (au moins doublée)
    assert_eq!(v.len(), 5);
    assert!(v.capacity() >= 5);
}

#[test]
fn vec_access() {
    let v = vec![10, 20, 30];

    // Indexation directe : vérifiée à l'exécution, panique si hors bornes
    assert_eq!(v[1], 20);
    // let _ = v[10]; // panic: index out of bounds

    // `get` : pas de panique, retourne un Option
    assert_eq!(v.get(1), Some(&20));
    assert_eq!(v.get(10), None);

    // `get_unchecked` (unsafe, non couvert ici) esquive la vérification :
    // à réserver aux cas où la borne est déjà garantie ailleurs.
    // TODO: try to use it !
}

#[test]
fn vec_iteration_vs_indexing() {
    let v = vec![1, 2, 3, 4];

    // Style indexé : correct, mais plus verbeux et sujet aux erreurs (off-by-one)
    let mut sum_indexed = 0;
    for i in 0..v.len() {
        sum_indexed += v[i];
    }

    // Style itéré : idiomatique, pas de risque de mauvais indice
    let sum_iterated: i32 = v.iter().sum();

    assert_eq!(sum_indexed, sum_iterated);
}
