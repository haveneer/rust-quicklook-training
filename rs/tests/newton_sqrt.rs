fn newton_sqrt(x: f64, precision: f64) -> Option<(f64, usize)> {
    // Suite infinie d'approximations successives (méthode de Newton-Raphson).
    // take(100) borne le calcul : sans lui, x < 0 ou une précision hors résolution f64
    // font osciller la suite indéfiniment (jamais convergée -> jamais consommée).
    const MAX_ITERATIONS: usize = 100;
    std::iter::successors(Some(x), |&guess| Some((guess + x / guess) / 2.0))
        .enumerate()
        .take(MAX_ITERATIONS)
        .find(|&(_, guess)| (guess * guess - x).abs() < precision)
        .map(|(i, guess)| (guess, i))
}

fn main() {
    for &x in &[2.0, 4.0, 9.0, 100.0, 0.25, -4.0] {
        match newton_sqrt(x, 1e-10) {
            Some((approx, iterations)) => {
                println!("sqrt({x}) ≈ {approx} en {iterations} itérations");
                assert!((approx - x.sqrt()).abs() < 1e-9);
            }
            None => println!("sqrt({x}) : ne converge pas (x < 0 ?)"),
        }
    }
}

#[test]
fn newton_sqrt_converges() {
    main();
}

#[test]
fn newton_sqrt_converges_fast() {
    // Convergence quadratique : peu d'itérations, même en partant loin de la solution.
    let (_, iterations) = newton_sqrt(1e6, 1e-10).unwrap();
    assert!(iterations < 30);
}

#[test]
fn newton_sqrt_diverges_on_negative() {
    assert_eq!(newton_sqrt(-4.0, 1e-10), None);
}

#[test]
fn newton_sqrt_diverges_on_unreachable_precision() {
    assert_eq!(newton_sqrt(2.0, 1e-20), None);
}
