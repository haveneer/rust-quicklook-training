// But : garder les mots de plus de 4 lettres, en majuscules.

#[test]
fn long_words_imperative() {
    let words = vec!["chat", "rust", "programmation", "vec", "fonctionnel"];

    // Impératif : on décrit COMMENT construire le résultat, pas à pas.
    let mut result = Vec::new();
    for w in &words {
        if w.len() > 4 {
            result.push(w.to_uppercase());
        }
    }

    assert_eq!(result, vec!["PROGRAMMATION", "FONCTIONNEL"]);
}

#[test]
fn long_words_declarative() {
    let words = vec!["chat", "rust", "programmation", "vec", "fonctionnel"];

    // Déclaratif : on décrit QUOI obtenir ; la boucle et l'accumulateur disparaissent.
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 4)
        .map(|w| w.to_uppercase())
        .collect();

    assert_eq!(result, vec!["PROGRAMMATION", "FONCTIONNEL"]);
}

#[test]
fn pure_vs_impure() {
    // Pure : le résultat ne dépend que des arguments, rien n'est modifié à l'extérieur.
    fn add_tax(price: f64, rate: f64) -> f64 {
        price * (1.0 + rate)
    }

    // Impure : mute un état partagé (effet de bord) en plus de calculer.
    fn add_tax_in_place(prices: &mut [f64], rate: f64) {
        for p in prices.iter_mut() {
            *p *= 1.0 + rate;
        }
    }

    // add_tax(100.0, 0.2) donnera toujours 120.0 : prévisible, testable isolément.
    assert_eq!(add_tax(100.0, 0.2), 120.0);
    assert_eq!(add_tax(100.0, 0.2), 120.0);

    let mut prices = [100.0, 50.0];
    add_tax_in_place(&mut prices, 0.2);
    assert_eq!(prices, [120.0, 60.0]);
    // Rejouer add_tax_in_place sur les mêmes `prices` changerait le résultat (déjà taxé une fois).
}
