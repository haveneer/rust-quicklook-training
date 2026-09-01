// Depuis Rust 1.82, `use<...>` précise explicitement ce que le RPIT capture,
// sans passer par un bound d'outlives ni un trait marqueur.

// `use<'_>` capture la lifetime anonyme de `x` : équivalent aux 2 "tricks" édition 2021
fn f_use_lifetime(x: &()) -> impl Sized + use<'_> {
    x
}

// `use<T>` force la dépendance à un type générique même s'il n'apparaît pas dans le retour
fn f_use_type<T: Sized>(_x: T) -> impl Sized + use<T> {
    std::mem::size_of::<T>()
}

fn main() {
    let v = ();
    let _ = f_use_lifetime(&v);
    let _ = f_use_type(42_u32);
}

#[test]
fn test() {
    main();
}
