/// Solver settings: every field has a meaningful default value.
#[derive(Debug, Clone)]
struct SolverConfig {
    tolerance: f64,
    max_iterations: usize,
    relaxation: f64,
    label: String,
}

/// Written by hand: a derived `Default` would give `tolerance: 0.0`.
impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-8,
            max_iterations: 1_000,
            relaxation: 1.0,
            label: String::from("solver"),
        }
    }
}

fn main() {
    // Only the fields that differ from the defaults are written
    let coarse = SolverConfig {
        tolerance: 1e-4,
        ..Default::default()
    };
    // Any value of the same type can serve as the base
    let relaxed = SolverConfig {
        relaxation: 0.7,
        ..coarse.clone() // without clone, `coarse.label` would be moved
    };
    assert_eq!(relaxed.max_iterations, 1_000);
    println!("{coarse:?}\n{relaxed:?}");
}

#[test]
fn test() {
    main()
}
