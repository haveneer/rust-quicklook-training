//! A simple implementation of the Y Combinator:
//! λf.(λx.xx)(λx.f(xx))
//! <=> λf.(λx.f(xx))(λx.f(xx))
//! Reader helper : https://fr.wikipedia.org/wiki/Lambda-calcul
//! see https://en.wikipedia.org/wiki/Fixed-point_combinator
//! On various languages: https://rosettacode.org/wiki/Y_combinator#Rust
//! More details: https://humanreadablemag.com/issues/1/articles/the-y-combinator-for-programmers
/// A function type that takes its own type as an input is an infinite recursive type.
/// We introduce the "Apply" trait, which will allow us to have an input with the same type as self, and break the recursion.
/// The input is going to be a trait object that implements the desired function in the interface.
trait Apply<T, R> {
    fn apply(&self, f: &dyn Apply<T, R>, t: T) -> R;
}

/// If we were to pass in self as f, we get:
/// λf.λt.sft
/// => λs.λt.sst [s/f]
/// => λs.ss
impl<T, R, F> Apply<T, R> for F where F: Fn(&dyn Apply<T, R>, T) -> R {
    fn apply(&self, f: &dyn Apply<T, R>, t: T) -> R {
        self(f, t)
    }
}

/// (λt(λx.(λy.xxy))(λx.(λy.f(λz.xxz)y)))t
/// => (λx.xx)(λx.f(xx))
/// => Yf
fn y<T, R>(f: impl Fn(&dyn Fn(T) -> R, T) -> R) -> impl Fn(T) -> R {
    move |t| (&|x: &dyn Apply<T, R>, y| x.apply(x, y))
        (&|x: &dyn Apply<T, R>, y| f(&|z| x.apply(x, z), y), t)
}

/// Factorial of n.
fn fac(n: usize) -> usize {
    let almost_fac = |f: &dyn Fn(usize) -> usize, x| if x == 0 { 1 } else { x * f(x - 1) };
    y(almost_fac)(n)
}

/// nth Fibonacci number.
fn fib(n: usize) -> usize {
    let almost_fib = |f: &dyn Fn((usize, usize, usize)) -> usize, (a0, a1, x)|
        match x {
            0 => a0,
            1 => a1,
            _ => f((a1, a0 + a1, x - 1)),
        };

    y(almost_fib)((1, 1, n))
}

fn main() {
    // Computations are fully optimized at compile time: 
    // https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=82517f70eeefcd0092088df1b13855fe
    let n = 10;
    println!("fac({}) = {}", n, fac(n));
    assert_eq!(fac(n), 3628800);
    println!("fib({}) = {}", n, fib(n));
    assert_eq!(fib(n), 89);
}

#[test]
fn test() { main() }
