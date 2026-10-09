#[test]
fn maps_and_sets_operations() {
    use std::collections::{BTreeMap, BTreeSet, HashMap};

    // Construction : depuis un tableau de paires, ou insert() au fil de l'eau
    let mut hm: HashMap<&str, i32> = HashMap::from([("b", 2), ("a", 1)]);
    hm.insert("c", 3);
    assert_eq!(hm.insert("a", 11), Some(1)); // remplace et rend l'ancienne valeur

    // Rechercher par clé : O(1) en moyenne, Option si absente
    assert_eq!(hm.get("a"), Some(&11));
    if let Some(v) = hm.get_mut("c") {
        *v += 1; // modification sur place
    }

    // Retirer : rend la valeur retirée ; retain filtre en place
    assert_eq!(hm.remove("b"), Some(2));
    assert_eq!(hm.remove("z"), None);
    hm.retain(|_, v| *v > 3);
    // Ordre d'itération non garanti pour HashMap

    let mut bm: BTreeMap<&str, i32> = hm.into_iter().collect();
    bm.insert("e", 5);
    bm.insert("d", 4);
    // BTreeMap conserve les clés triées : requêtes par plage en O(log n + k)
    let keys: Vec<_> = bm.keys().collect();
    assert_eq!(keys, vec![&"a", &"c", &"d", &"e"]);
    let window: Vec<_> = bm.range("b".."e").map(|(k, _)| *k).collect();
    assert_eq!(window, ["c", "d"]);

    // Ensembles : insert dit si l'élément était nouveau
    let mut set = BTreeSet::new();
    assert!(set.insert("mesh"));
    assert!(!set.insert("mesh"));
}
