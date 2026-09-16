use std::collections::HashMap;

// L'ordre de déclaration des champs fixe l'ordre lexicographique du Ord dérivé
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

fn main() {
    // Default: la valeur "neutre" du type, et la syntaxe de mise à jour de struct
    let v1 = Version {
        major: 1,
        ..Default::default()
    };
    println!("{v1:?}"); // Version { major: 1, minor: 0, patch: 0 }

    // Ord dérivé: comparaison lexicographique, champ par champ
    let mut versions = vec![
        Version {
            major: 1,
            minor: 10,
            patch: 0,
        },
        Version {
            major: 1,
            minor: 2,
            patch: 3,
        },
    ];
    versions.sort(); // requiert Ord
    println!("{versions:?}"); // 1.2.3 avant 1.10.0

    // Hash + Eq vont de pair: contrat  a == b  =>  hash(a) == hash(b)
    let mut notes = HashMap::new();
    notes.insert(v1, "première version stable");
    println!("{:?}", notes.get(&v1));

    // f64 est PartialEq mais PAS Eq: NaN != NaN casse la réflexivité (a == a)
    let nan = f64::NAN;
    #[allow(clippy::eq_op)] // comparaison réflexive volontaire: c'est justement le propos
    let reflexive = nan == nan;
    println!("{reflexive}"); // false
                             // => pas de HashMap<f64, _>, et pas de .sort() car pas Ord non plus
    let mut xs = vec![3.0, nan, 1.0];
    // xs.sort();               // ne compile pas: `f64: Ord` non satisfait
    xs.sort_by(f64::total_cmp); // ordre total explicite, à la charge de l'appelant
    println!("{xs:?}");
    println!("{:?}", 1.0_f64.partial_cmp(&nan)); // None: aucun ordre avec NaN
}

#[test]
fn test() {
    main();
}
