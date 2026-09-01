// Sans annotation, la lifetime de `x` n'est pas capturée par le RPIT -> ne compile pas :
// error[E0700]: hidden type for `impl Sized` captures lifetime that does not appear in bounds
// fn f_failure(x: &()) -> impl Sized {
//     x
// }

// Trick 1 : lier explicitement la lifetime au type de retour avec un bound `+ 'a`
fn f_trick_bound<'a>(x: &'a ()) -> impl Sized + 'a {
    x
}

// Trick 2 : un trait marqueur "Capture<'a>" implémenté pour tout `'a`, utilisé comme bound
trait Capture<'a> {}
impl<'a, T: ?Sized> Capture<'a> for T {}

fn f_trick_marker<'a>(x: &'a ()) -> impl Sized + Capture<'a> {
    x
}

fn main() {
    let v = ();
    let _ = f_trick_bound(&v);
    let _ = f_trick_marker(&v);
}

#[test]
fn test() {
    main();
}
