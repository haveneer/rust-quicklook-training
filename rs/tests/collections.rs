#[test]
fn map_hashmap_vs_btreemap() {
    use std::collections::{BTreeMap, HashMap};

    // Construction : depuis un tableau de paires, ou insert() au fil de l'eau
    let mut hm: HashMap<&str, i32> = HashMap::from([("b", 2), ("a", 1)]);
    hm.insert("c", 3);

    // `entry()` : lire-ou-créer / modifier en une seule recherche
    *hm.entry("a").or_insert(0) += 10; // "a" existe déjà -> modifié
    hm.entry("d").or_insert(4); // "d" n'existe pas -> créé
    assert_eq!(hm["a"], 11);
    assert_eq!(hm.get("d"), Some(&4));

    assert!(hm.contains_key("b"));
    hm.remove("b");
    assert!(!hm.contains_key("b"));
    // Ordre d'itération non garanti pour HashMap

    let mut bm: BTreeMap<&str, i32> = hm.into_iter().collect();
    bm.insert("e", 5);
    // BTreeMap conserve les clés triées
    let keys: Vec<_> = bm.keys().collect();
    assert_eq!(keys, vec![&"a", &"c", &"d", &"e"]);

    assert_eq!(bm.len(), 4);
}
