/// Integrates `f` over `[a, b]` with the trapezoidal rule on `n` intervals.
pub fn trapezoid(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> Result<f64, String> {
    if n == 0 {
        return Err("n must be > 0".into());
    }
    let h = (b - a) / n as f64;
    let inner: f64 = (1..n).map(|i| f(a + i as f64 * h)).sum();
    Ok(h * (0.5 * (f(a) + f(b)) + inner))
}

#[cfg(test)] // compiled only by `cargo test`
mod tests {
    use super::*; // private items are reachable too

    #[test]
    fn linear_is_exact() {
        let r = trapezoid(|x| 2.0 * x, 0.0, 1.0, 4).unwrap();
        assert_eq!(r, 1.0);
    }

    #[test]
    fn converges_on_sine() -> Result<(), String> {
        let r = trapezoid(f64::sin, 0.0, std::f64::consts::PI, 1000)?;
        assert!((r - 2.0).abs() < 1e-5, "r = {r}");
        Ok(())
    }

    #[test]
    fn rejects_zero_intervals() {
        assert!(trapezoid(f64::sin, 0.0, 1.0, 0).is_err());
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn detects_overflow() {
        let n: u8 = std::hint::black_box(255);
        let _ = n + 1; // debug build: overflow check
    }

    #[test]
    #[ignore] // too slow: `cargo test -- --ignored`
    fn converges_on_huge_mesh() {
        let r = trapezoid(f64::sin, 0.0, std::f64::consts::PI, 100_000_000).unwrap();
        assert!((r - 2.0).abs() < 1e-12);
    }
}
