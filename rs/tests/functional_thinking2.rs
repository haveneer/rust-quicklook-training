/* Shell version
   echo "Le mouvement anti-trust s'incrusta dans la mémoire collective." | \
     tr ' ' '\n' | \
     grep 'a' | \
     sed -E 's/.*[^[:alpha:]]//' | \
     tr 'c' '\n' | \
     tr -d 'a' | \
     head -n 3 | \
     tr '[:lower:]' '[:upper:]' | \
     xargs
*/

use itertools::Itertools;

#[test]
fn long_words_declarative() {
    let sentence = "Le mouvement anti-trust s'incrusta dans la mémoire collective.";

    let result: String = sentence
        .split_whitespace()
        .filter(|w| w.contains('a'))
        .flat_map(|w| w.rsplit(|c: char| !c.is_alphabetic()).next())
        .flat_map(|w| w.split('c'))
        .map(|w| w.replace('a', ""))
        .take(3)
        .join(" ")
        .to_uppercase();

    assert_eq!(result, "TRUST IN RUST");
}

#[test]
fn long_words_imperative() {
    let sentence = "Le mouvement anti-trust s'incrusta dans la mémoire collective.";
    let mut result = String::new();
    let mut count = 0;

    'outer: for word in sentence.split_whitespace() {
        if !word.contains('a') {
            continue;
        }
        let mut start = 0;
        if let Some(idx) = word.rfind(|c: char| !c.is_alphabetic()) {
            start = idx + 1;
        }
        for sub in word[start..].split('c') {
            if count > 0 {
                result.push(' ');
            }
            result.push_str(&sub.replace('a', "").to_uppercase());
            count += 1;
            if count == 3 {
                break 'outer;
            }
        }
    }

    assert_eq!(result, "TRUST IN RUST");
}
