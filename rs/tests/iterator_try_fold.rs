use std::ops::ControlFlow;

#[allow(clippy::while_let_on_iterator)]
#[rustfmt::skip]
fn take_while_odd(values: &[i32]) -> i32 {
    // Itération EXTERNE (pull) : l'appelant tire élément par élément avec next().
    let mut it = values.iter();
    let mut sum = 0;
    while let Some(&n) = it.next() { // tire un nouvel élément
        if n % 2 == 0 { // un test par élément
            break; // arrêt prématuré trivial (on s'arrête avant le premier pair)
        }
        sum += n;
    }
    sum
}

#[allow(clippy::unnecessary_fold)]
fn sum_all(values: &[i32]) -> i32 {
    // Itération INTERNE (push) : le conteneur gère la boucle (fold).
    // Rapide (boucle C), mais fold ne sait PAS s'arrêter : il parcourt tout.
    values.iter().fold(0, |acc, &n| acc + n)
}

// try_fold + ControlFlow : itération interne qui, elle, sait s'arrêter en cours
// de route (court-circuit). C'est le mécanisme fondamental de find(), any(),
// all(), position() ... (stabilisé après l'article de Veedrac, 2016).
fn first_even(values: &[i32]) -> Option<i32> {
    let flow = values.iter().try_fold((), |_, &n| {
        if n % 2 == 0 {
            ControlFlow::Break(n) // s'arrête immédiatement, renvoie la valeur
        } else {
            ControlFlow::Continue(())
        }
    });
    match flow {
        ControlFlow::Break(n) => Some(n),
        ControlFlow::Continue(()) => None,
    }
}

fn main() {
    let values = [1, 3, 5, 8, 10, 13];
    println!("take_while_odd = {}", take_while_odd(&values)); // 1+3+5 = 9
    println!("sum_all        = {}", sum_all(&values)); // 40 (passe tout)
    println!("first_even     = {:?}", first_even(&values)); // Some(8)
}

#[test]
fn fold_vs_try_fold() {
    main()
}
